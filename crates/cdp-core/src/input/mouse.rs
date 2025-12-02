use std::{
    future::Future,
    sync::Arc,
    time::{Duration, Instant},
};

use rand::Rng;
use serde_json::Value;

pub use cdp_protocol::input::MouseButton;
use cdp_protocol::input::{
    DispatchMouseEvent, DispatchMouseEventReturnObject, DispatchMouseEventTypeOption,
};
use cdp_protocol::runtime::{Evaluate, EvaluateReturnObject};

use crate::{
    domain_manager::DomainManager,
    error::{CdpError, Result},
    session::Session,
};

/// Configuration for a single mouse click.
#[derive(Debug, Clone)]
pub struct MouseClickOptions {
    /// Mouse button to use; defaults to the left button.
    pub button: MouseButton,
    /// Optional delay to wait before releasing the button.
    pub delay_after_press: Option<Duration>,
}

impl Default for MouseClickOptions {
    fn default() -> Self {
        Self {
            button: MouseButton::Left,
            delay_after_press: None,
        }
    }
}

/// Configuration for a double-click action.
#[derive(Debug, Clone)]
pub struct DoubleClickOptions {
    /// Mouse button to use; defaults to the left button.
    pub button: MouseButton,
    /// Delay inserted between the two presses.
    pub delay_between_clicks: Duration,
}

impl Default for DoubleClickOptions {
    fn default() -> Self {
        Self {
            button: MouseButton::Left,
            delay_between_clicks: Duration::from_millis(50),
        }
    }
}

/// Options for holding the mouse button until a condition is met.
#[derive(Debug, Clone)]
pub struct PressHoldOptions {
    /// Mouse button to use; defaults to the left button.
    pub button: MouseButton,
    /// Interval between polling the condition callback.
    pub poll_interval: Duration,
    /// Maximum amount of time to wait; `None` disables the timeout.
    pub timeout: Option<Duration>,
    /// Minimum duration to hold the button before evaluating the condition.
    pub min_duration: Option<Duration>,
}

impl Default for PressHoldOptions {
    fn default() -> Self {
        Self {
            button: MouseButton::Left,
            poll_interval: Duration::from_millis(100),
            timeout: None,
            min_duration: None,
        }
    }
}

/// Speed curve used while generating drag motion.
#[derive(Debug, Clone, Copy, Default)]
pub enum DragEasing {
    /// Linear speed, without acceleration.
    Linear,
    /// Smooth ease-in/ease-out cubic profile.
    #[default]
    EaseInOutCubic,
    /// Quick start with a slower finish.
    EaseOutQuad,
}

impl DragEasing {
    fn evaluate(self, t: f64) -> f64 {
        let clamped = t.clamp(0.0, 1.0);
        match self {
            DragEasing::Linear => clamped,
            DragEasing::EaseInOutCubic => {
                if clamped < 0.5 {
                    4.0 * clamped * clamped * clamped
                } else {
                    let s = -2.0 * clamped + 2.0;
                    1.0 - (s * s * s) / 2.0
                }
            }
            DragEasing::EaseOutQuad => 1.0 - (1.0 - clamped) * (1.0 - clamped),
        }
    }
}

/// Configuration controlling drag gestures.
#[derive(Debug, Clone)]
pub struct DragOptions {
    /// Mouse button to use; defaults to the left button.
    pub button: MouseButton,
    /// Total duration of the drag gesture.
    pub total_duration: Duration,
    /// Number of intermediate steps; higher values produce smoother motion.
    pub steps: usize,
    /// Maximum jitter, in pixels, applied to intermediate points.
    pub jitter_px: f64,
    /// Optional delay after pressing the button and before moving.
    pub hold_before_move: Option<Duration>,
    /// Optional delay after reaching the target and before releasing.
    pub settle_after_move: Option<Duration>,
    /// Whether the cursor should move to the start before pressing.
    pub move_cursor_to_start: bool,
    /// Curve used to distribute motion during the drag.
    pub easing: DragEasing,
}

impl Default for DragOptions {
    fn default() -> Self {
        Self {
            button: MouseButton::Left,
            total_duration: Duration::from_millis(700),
            steps: 28,
            jitter_px: 0.8,
            hold_before_move: Some(Duration::from_millis(80)),
            settle_after_move: Some(Duration::from_millis(60)),
            move_cursor_to_start: true,
            easing: DragEasing::default(),
        }
    }
}

/// High-level helper for dispatching mouse input via the CDP Input domain.
#[derive(Clone)]
pub struct Mouse {
    session: Arc<Session>,
    domain_manager: Arc<DomainManager>,
}

/// Current mouse coordinates in both viewport and screen space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MousePosition {
    /// X coordinate relative to the top-left of the viewport (CSS pixels).
    pub viewport_x: f64,
    /// Y coordinate relative to the top-left of the viewport (CSS pixels).
    pub viewport_y: f64,
    /// X coordinate relative to the top-left of the screen (CSS pixels).
    pub screen_x: f64,
    /// Y coordinate relative to the top-left of the screen (CSS pixels).
    pub screen_y: f64,
}

impl Mouse {
    pub(crate) fn new(session: Arc<Session>, domain_manager: Arc<DomainManager>) -> Self {
        Self {
            session,
            domain_manager,
        }
    }

    async fn dispatch(&self, params: DispatchMouseEvent) -> Result<()> {
        self.session
            .send_command::<_, DispatchMouseEventReturnObject>(params, None)
            .await
            .map(|_| ())
    }

    /// Moves the cursor to the given coordinates and returns the resolved position.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let mouse = page.mouse();
    /// let position = mouse.move_to(120.0, 80.0).await?;
    /// assert_eq!(position.viewport_x, 120.0);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn move_to(&self, x: f64, y: f64) -> Result<MousePosition> {
        let params = DispatchMouseEvent {
            r#type: DispatchMouseEventTypeOption::MouseMoved,
            x,
            y,
            modifiers: None,
            timestamp: None,
            button: None,
            buttons: Some(0),
            click_count: Some(0),
            force: None,
            tangential_pressure: None,
            tilt_x: None,
            tilt_y: None,
            twist: None,
            delta_x: None,
            delta_y: None,
            pointer_type: None,
        };

        self.dispatch(params).await?;
        self.position_for(x, y).await
    }

    async fn position_for(&self, viewport_x: f64, viewport_y: f64) -> Result<MousePosition> {
        let expression = format!(
            "(() => {{
                const viewportX = {viewport_x};
                const viewportY = {viewport_y};
                const screenBaseX = window.screenX ?? window.screenLeft ?? 0;
                const screenBaseY = window.screenY ?? window.screenTop ?? 0;
                return {{
                    viewportX,
                    viewportY,
                    screenX: screenBaseX + viewportX,
                    screenY: screenBaseY + viewportY
                }};
            }})()",
            viewport_x = viewport_x,
            viewport_y = viewport_y,
        );

        let evaluate = Evaluate {
            expression,
            object_group: None,
            include_command_line_api: None,
            silent: None,
            context_id: None,
            return_by_value: Some(true),
            generate_preview: None,
            user_gesture: None,
            await_promise: None,
            throw_on_side_effect: None,
            timeout: None,
            disable_breaks: None,
            repl_mode: None,
            allow_unsafe_eval_blocked_by_csp: None,
            unique_context_id: None,
            serialization_options: None,
        };

        let eval_result = self
            .session
            .send_command::<_, EvaluateReturnObject>(evaluate, None)
            .await?;

        let value: Value = eval_result
            .result
            .value
            .ok_or_else(|| CdpError::tool("Failed to obtain mouse position after move"))?;

        let screen_x = value
            .get("screenX")
            .and_then(Value::as_f64)
            .ok_or_else(|| CdpError::tool("Invalid screenX returned for mouse position"))?;
        let screen_y = value
            .get("screenY")
            .and_then(Value::as_f64)
            .ok_or_else(|| CdpError::tool("Invalid screenY returned for mouse position"))?;
        let viewport_x = value
            .get("viewportX")
            .and_then(Value::as_f64)
            .ok_or_else(|| CdpError::tool("Invalid viewportX returned for mouse position"))?;
        let viewport_y = value
            .get("viewportY")
            .and_then(Value::as_f64)
            .ok_or_else(|| CdpError::tool("Invalid viewportY returned for mouse position"))?;

        Ok(MousePosition {
            viewport_x,
            viewport_y,
            screen_x,
            screen_y,
        })
    }

    async fn press(&self, x: f64, y: f64, button: MouseButton, click_count: u32) -> Result<()> {
        let params = DispatchMouseEvent {
            r#type: DispatchMouseEventTypeOption::MousePressed,
            x,
            y,
            modifiers: None,
            timestamp: None,
            button: Some(button.clone()),
            buttons: None,
            click_count: Some(click_count),
            force: None,
            tangential_pressure: None,
            tilt_x: None,
            tilt_y: None,
            twist: None,
            delta_x: None,
            delta_y: None,
            pointer_type: None,
        };
        self.dispatch(params).await
    }

    async fn release(&self, x: f64, y: f64, button: MouseButton, click_count: u32) -> Result<()> {
        let params = DispatchMouseEvent {
            r#type: DispatchMouseEventTypeOption::MouseReleased,
            x,
            y,
            modifiers: None,
            timestamp: None,
            button: Some(button.clone()),
            buttons: None,
            click_count: Some(click_count),
            force: None,
            tangential_pressure: None,
            tilt_x: None,
            tilt_y: None,
            twist: None,
            delta_x: None,
            delta_y: None,
            pointer_type: None,
        };
        self.dispatch(params).await
    }

    /// Performs a single click at the provided coordinates.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::{MouseClickOptions, MouseButton, Page};
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let mouse = page.mouse();
    /// let mut options = MouseClickOptions::default();
    /// options.button = MouseButton::Right;
    /// mouse.click(300.0, 200.0, options).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn click(&self, x: f64, y: f64, options: MouseClickOptions) -> Result<()> {
        self.press(x, y, options.button.clone(), 1).await?;
        if let Some(delay) = options.delay_after_press {
            tokio::time::sleep(delay).await;
        }
        self.release(x, y, options.button, 1).await
    }

    /// Convenience helper for a left-button click.
    pub async fn left_click(&self, x: f64, y: f64) -> Result<()> {
        self.click(x, y, MouseClickOptions::default()).await
    }

    /// Convenience helper for a right-button click.
    pub async fn right_click(&self, x: f64, y: f64) -> Result<()> {
        let options = MouseClickOptions {
            button: MouseButton::Right,
            ..Default::default()
        };
        self.click(x, y, options).await
    }

    /// Performs a double-click; the default delay is 50ms between presses.
    pub async fn double_click(&self, x: f64, y: f64, options: DoubleClickOptions) -> Result<()> {
        self.press(x, y, options.button.clone(), 1).await?;
        self.release(x, y, options.button.clone(), 1).await?;
        tokio::time::sleep(options.delay_between_clicks).await;
        self.press(x, y, options.button.clone(), 2).await?;
        self.release(x, y, options.button, 2).await
    }

    /// Holds the given button at the coordinates for the requested duration.
    pub async fn press_and_hold(
        &self,
        x: f64,
        y: f64,
        button: MouseButton,
        duration: Duration,
    ) -> Result<()> {
        self.press(x, y, button.clone(), 1).await?;
        tokio::time::sleep(duration).await;
        self.release(x, y, button, 1).await
    }

    /// Holds the given button until the condition callback returns `true` or a timeout occurs.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::{PressHoldOptions, Page};
    /// # use std::sync::Arc;
    /// # use std::time::Duration;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let mouse = page.mouse();
    /// let mut options = PressHoldOptions::default();
    /// options.timeout = Some(Duration::from_secs(2));
    /// let success = mouse
    ///     .press_and_hold_until(400.0, 250.0, options, || async { Ok(true) })
    ///     .await?;
    /// assert!(success);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn press_and_hold_until<F, Fut>(
        &self,
        x: f64,
        y: f64,
        options: PressHoldOptions,
        mut condition: F,
    ) -> Result<bool>
    where
        F: FnMut() -> Fut + Send,
        Fut: Future<Output = Result<bool>> + Send,
    {
        self.press(x, y, options.button.clone(), 1).await?;

        if let Some(min_duration) = options.min_duration {
            tokio::time::sleep(min_duration).await;
        }

        let start = Instant::now();
        let mut result = Ok(false);

        loop {
            if let Some(timeout) = options.timeout
                && start.elapsed() >= timeout
            {
                break;
            }

            match condition().await {
                Ok(true) => {
                    result = Ok(true);
                    break;
                }
                Ok(false) => {}
                Err(err) => {
                    result = Err(err);
                    break;
                }
            }

            tokio::time::sleep(options.poll_interval).await;
        }

        let release_result = self.release(x, y, options.button, 1).await;

        match (result, release_result) {
            (Err(err), Ok(_)) => Err(err),
            (_, Err(release_err)) => Err(release_err),
            (Ok(val), Ok(_)) => Ok(val),
        }
    }

    /// Simulates drag gestures by generating a jittered, eased trajectory between two points.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::{input::mouse::DragOptions, Page};
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let mouse = page.mouse();
    /// mouse.drag_to(20.0, 20.0, 220.0, 260.0, DragOptions::default()).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn drag_to(
        &self,
        start_x: f64,
        start_y: f64,
        end_x: f64,
        end_y: f64,
        options: DragOptions,
    ) -> Result<()> {
        let DragOptions {
            button,
            total_duration,
            steps,
            jitter_px,
            hold_before_move,
            settle_after_move,
            move_cursor_to_start,
            easing,
        } = options;

        if move_cursor_to_start {
            let _ = self.move_to(start_x, start_y).await?;
        }

        self.press(start_x, start_y, button.clone(), 1).await?;

        if let Some(delay) = hold_before_move
            && !delay.is_zero()
        {
            tokio::time::sleep(delay).await;
        }

        let total_steps = steps.max(2);
        let delta_x = end_x - start_x;
        let delta_y = end_y - start_y;
        let step_delay = total_duration.div_f64(total_steps as f64);
        let button_mask = Self::button_bitmask(&button);
        let mut rng = rand::rng();

        for step_idx in 1..=total_steps {
            let progress = step_idx as f64 / total_steps as f64;
            let eased = easing.evaluate(progress);

            let base_x = start_x + delta_x * eased;
            let base_y = start_y + delta_y * eased;

            let apply_jitter = jitter_px > 0.0 && step_idx < total_steps;
            let jitter_x = if apply_jitter {
                rng.random_range(-jitter_px..=jitter_px)
            } else {
                0.0
            };
            let jitter_y = if apply_jitter {
                rng.random_range(-jitter_px..=jitter_px)
            } else {
                0.0
            };

            let params = DispatchMouseEvent {
                r#type: DispatchMouseEventTypeOption::MouseMoved,
                x: base_x + jitter_x,
                y: base_y + jitter_y,
                modifiers: None,
                timestamp: None,
                button: Some(button.clone()),
                buttons: Some(button_mask),
                click_count: Some(0),
                force: None,
                tangential_pressure: None,
                tilt_x: None,
                tilt_y: None,
                twist: None,
                delta_x: None,
                delta_y: None,
                pointer_type: None,
            };

            self.dispatch(params).await?;

            if step_idx < total_steps && !step_delay.is_zero() {
                tokio::time::sleep(step_delay).await;
            }
        }

        if let Some(delay) = settle_after_move
            && !delay.is_zero()
        {
            tokio::time::sleep(delay).await;
        }

        self.release(end_x, end_y, button, 1).await
    }

    fn button_bitmask(button: &MouseButton) -> u32 {
        match button {
            MouseButton::None => 0,
            MouseButton::Left => 1,
            MouseButton::Right => 2,
            MouseButton::Middle => 4,
            MouseButton::Back => 8,
            MouseButton::Forward => 16,
        }
    }
}
