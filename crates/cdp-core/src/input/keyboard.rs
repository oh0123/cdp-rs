use std::sync::Arc;
use std::time::Duration;

use rand::Rng;
use tokio::time::sleep;

use cdp_protocol::input::{
    DispatchKeyEvent, DispatchKeyEventReturnObject, DispatchKeyEventTypeOption, InsertText,
    InsertTextReturnObject,
};

use crate::{domain_manager::DomainManager, error::Result, session::Session};

/// High-level helper for dispatching keyboard events through the CDP Input domain.
///
/// # Examples
/// ```no_run
/// # use cdp_core::Page;
/// # use std::sync::Arc;
/// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
/// let keyboard = page.keyboard();
/// keyboard.press_key("Enter").await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct Keyboard {
    session: Arc<Session>,
    domain_manager: Arc<DomainManager>,
}

impl Keyboard {
    pub(crate) fn new(session: Arc<Session>, domain_manager: Arc<DomainManager>) -> Self {
        Self {
            session,
            domain_manager,
        }
    }

    async fn dispatch_key_event(
        &self,
        event_type: DispatchKeyEventTypeOption,
        key: &KeyInput,
        modifiers: KeyModifiers,
        auto_repeat: bool,
    ) -> Result<()> {
        let params = DispatchKeyEvent {
            r#type: event_type,
            modifiers: modifiers.bits(),
            timestamp: None,
            text: key.text.clone(),
            unmodified_text: key.text.clone(),
            key_identifier: None,
            code: key.code.clone(),
            key: Some(key.key.clone()),
            windows_virtual_key_code: key.key_code,
            native_virtual_key_code: key.key_code,
            auto_repeat: Some(auto_repeat),
            is_keypad: Some(key.is_keypad),
            is_system_key: Some(false),
            location: key.location,
            commands: None,
        };

        self.session
            .send_command::<_, DispatchKeyEventReturnObject>(params, None)
            .await
            .map(|_| ())
    }

    /// Dispatches a `keyDown` event for the provided key input.
    pub async fn key_down(&self, input: impl Into<KeyInput>) -> Result<()> {
        self.key_down_with_modifiers(input, KeyModifiers::empty())
            .await
    }

    /// Dispatches a `keyDown` event while applying the given modifiers.
    pub async fn key_down_with_modifiers(
        &self,
        input: impl Into<KeyInput>,
        modifiers: KeyModifiers,
    ) -> Result<()> {
        let key: KeyInput = input.into();
        self.dispatch_key_event(DispatchKeyEventTypeOption::KeyDown, &key, modifiers, false)
            .await
    }

    /// Dispatches a matching `keyUp` event.
    pub async fn key_up(&self, input: impl Into<KeyInput>) -> Result<()> {
        self.key_up_with_modifiers(input, KeyModifiers::empty())
            .await
    }

    /// Dispatches a `keyUp` event while applying the given modifiers.
    pub async fn key_up_with_modifiers(
        &self,
        input: impl Into<KeyInput>,
        modifiers: KeyModifiers,
    ) -> Result<()> {
        let key: KeyInput = input.into();
        self.dispatch_key_event(DispatchKeyEventTypeOption::KeyUp, &key, modifiers, false)
            .await
    }

    /// Sends a full key press (a `keyDown` followed by a `keyUp`).
    pub async fn press_key(&self, input: impl Into<KeyInput>) -> Result<()> {
        self.press_key_with_modifiers(input, KeyModifiers::empty())
            .await
    }

    /// Sends a key press while holding the provided modifiers.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::{KeyboardModifier, KeyModifiers, Page};
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let keyboard = page.keyboard();
    /// keyboard
    ///     .press_key_with_modifiers("r", KeyModifiers::from_iter([KeyboardModifier::Control]))
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn press_key_with_modifiers(
        &self,
        input: impl Into<KeyInput>,
        modifiers: KeyModifiers,
    ) -> Result<()> {
        let key: KeyInput = input.into();
        self.dispatch_key_event(DispatchKeyEventTypeOption::KeyDown, &key, modifiers, false)
            .await?;
        self.dispatch_key_event(DispatchKeyEventTypeOption::KeyUp, &key, modifiers, false)
            .await
    }

    /// Presses the primary key that corresponds to the given character.
    pub async fn press_character(&self, ch: char) -> Result<()> {
        self.press_key(KeyInput::from(ch)).await
    }

    /// Presses modifiers in order, executes the target key once, and releases modifiers in reverse.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::{KeyboardModifier, Page};
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let keyboard = page.keyboard();
    /// keyboard
    ///     .press_with_modifiers("l", [KeyboardModifier::Control, KeyboardModifier::Shift])
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn press_with_modifiers(
        &self,
        key: impl Into<KeyInput>,
        modifiers: impl IntoIterator<Item = KeyboardModifier>,
    ) -> Result<()> {
        let modifiers_vec: Vec<KeyboardModifier> = modifiers.into_iter().collect();
        if modifiers_vec.is_empty() {
            return self.press_key(key).await;
        }

        let mut active = KeyModifiers::empty();

        // Press modifiers one by one and keep track of the active bitmask.
        for modifier in &modifiers_vec {
            active = active.with(*modifier);
            let modifier_key = modifier.key_input();
            self.dispatch_key_event(
                DispatchKeyEventTypeOption::KeyDown,
                &modifier_key,
                active,
                false,
            )
            .await?;
        }

        // Dispatch the target key with all modifiers applied.
        let target_key: KeyInput = key.into();
        self.dispatch_key_event(
            DispatchKeyEventTypeOption::KeyDown,
            &target_key,
            active,
            false,
        )
        .await?;
        self.dispatch_key_event(
            DispatchKeyEventTypeOption::KeyUp,
            &target_key,
            active,
            false,
        )
        .await?;

        // Release modifiers in reverse order to mimic user input.
        for modifier in modifiers_vec.into_iter().rev() {
            active = active.without(modifier);
            let modifier_key = modifier.key_input();
            self.dispatch_key_event(
                DispatchKeyEventTypeOption::KeyUp,
                &modifier_key,
                active,
                false,
            )
            .await?;
        }

        Ok(())
    }

    /// Invokes `Input.insertText` to inject the provided text in one operation.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let keyboard = page.keyboard();
    /// keyboard.insert_text("Hello, CDP!").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn insert_text(&self, text: &str) -> Result<()> {
        let params = InsertText {
            text: text.to_string(),
        };

        self.session
            .send_command::<_, InsertTextReturnObject>(params, None)
            .await?;

        Ok(())
    }

    /// Simulates human typing by adding a random delay between `keyDown`/`keyUp` pairs.
    ///
    /// # Examples
    /// ```no_run
    /// # use cdp_core::Page;
    /// # use std::sync::Arc;
    /// # async fn example(page: Arc<Page>) -> anyhow::Result<()> {
    /// let keyboard = page.keyboard();
    /// keyboard.type_text_with_delay("Hello", 40, 120).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn type_text_with_delay(
        &self,
        text: &str,
        min_delay_ms: u64,
        max_delay_ms: u64,
    ) -> Result<()> {
        if text.is_empty() {
            return Ok(());
        }

        let (min_delay, max_delay) = if min_delay_ms <= max_delay_ms {
            (min_delay_ms, max_delay_ms)
        } else {
            (max_delay_ms, min_delay_ms)
        };

        let mut rng = rand::rng();

        for ch in text.chars() {
            self.press_character(ch).await?;

            if min_delay > 0 || max_delay > 0 {
                let delay = if min_delay == max_delay {
                    max_delay
                } else {
                    rng.random_range(min_delay..=max_delay)
                };

                if delay > 0 {
                    sleep(Duration::from_millis(delay)).await;
                }
            }
        }

        Ok(())
    }
}

/// Represents a key definition that can be dispatched to the browser.
#[derive(Debug, Clone)]
pub struct KeyInput {
    /// Display value for the key, for example `a` or `Enter`.
    pub key: String,
    /// Optional character produced by the key, forwarded to `text`/`unmodifiedText`.
    pub text: Option<String>,
    /// DOM `code` value used by the browser, when known.
    pub code: Option<String>,
    /// Platform-specific virtual key code (Windows/native).
    pub key_code: Option<u32>,
    /// Keyboard location (0 = Standard, 1 = Left, 2 = Right, 3 = Numpad).
    pub location: Option<u32>,
    /// Marks whether the key originates from the numeric keypad.
    pub is_keypad: bool,
}

impl KeyInput {
    /// Builds a key definition with the given display value.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            text: None,
            code: None,
            key_code: None,
            location: None,
            is_keypad: false,
        }
    }

    /// Builds a key definition from a single character.
    pub fn from_char(ch: char) -> Self {
        let key = ch.to_string();
        Self {
            key: key.clone(),
            text: Some(key),
            code: None,
            key_code: Some(ch as u32),
            location: None,
            is_keypad: false,
        }
    }

    /// Sets the character returned by the key.
    pub fn with_text(mut self, text: impl Into<String>) -> Self {
        self.text = Some(text.into());
        self
    }

    /// Sets the DOM `code` value for the key.
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Populates the virtual key code.
    pub fn with_key_code(mut self, key_code: u32) -> Self {
        self.key_code = Some(key_code);
        self
    }

    /// Sets the keyboard location value.
    pub fn with_location(mut self, location: u32) -> Self {
        self.location = Some(location);
        self
    }

    /// Marks the key as originating from the numeric keypad.
    pub fn from_keypad(mut self) -> Self {
        self.is_keypad = true;
        self
    }
}

impl From<&str> for KeyInput {
    fn from(value: &str) -> Self {
        let mut chars = value.chars();
        if let Some(first) = chars.next()
            && chars.next().is_none()
        {
            return KeyInput::from_char(first);
        }
        KeyInput::new(value)
    }
}

impl From<String> for KeyInput {
    fn from(value: String) -> Self {
        KeyInput::from(value.as_str())
    }
}

impl From<char> for KeyInput {
    fn from(value: char) -> Self {
        KeyInput::from_char(value)
    }
}

/// Supported keyboard modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyboardModifier {
    /// Alt key.
    Alt,
    /// Control key.
    Control,
    /// Meta key (Command on macOS).
    Meta,
    /// Shift key.
    Shift,
}

impl KeyboardModifier {
    const fn bit(self) -> u32 {
        match self {
            KeyboardModifier::Alt => 1,
            KeyboardModifier::Control => 2,
            KeyboardModifier::Meta => 4,
            KeyboardModifier::Shift => 8,
        }
    }

    fn key_input(self) -> KeyInput {
        match self {
            KeyboardModifier::Alt => KeyInput::new("Alt")
                .with_code("AltLeft")
                .with_key_code(18)
                .with_location(1),
            KeyboardModifier::Control => KeyInput::new("Control")
                .with_code("ControlLeft")
                .with_key_code(17)
                .with_location(1),
            KeyboardModifier::Meta => KeyInput::new("Meta")
                .with_code("MetaLeft")
                .with_key_code(91)
                .with_location(1),
            KeyboardModifier::Shift => KeyInput::new("Shift")
                .with_code("ShiftLeft")
                .with_key_code(16)
                .with_location(1),
        }
    }
}

/// Bitmask wrapper that tracks active modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyModifiers(u32);

impl KeyModifiers {
    /// Creates an empty modifier set.
    pub const fn empty() -> Self {
        Self(0)
    }

    /// Returns true if no modifiers are active.
    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    /// Returns a new set with the provided modifier enabled.
    pub const fn with(self, modifier: KeyboardModifier) -> Self {
        Self(self.0 | modifier.bit())
    }

    /// Returns a new set without the provided modifier.
    pub const fn without(self, modifier: KeyboardModifier) -> Self {
        Self(self.0 & !modifier.bit())
    }

    /// Produces the bitmask expected by CDP commands.
    pub const fn bits(self) -> Option<u32> {
        if self.0 == 0 { None } else { Some(self.0) }
    }
}

impl FromIterator<KeyboardModifier> for KeyModifiers {
    fn from_iter<T: IntoIterator<Item = KeyboardModifier>>(iter: T) -> Self {
        let mut modifiers = Self::empty();
        for modifier in iter {
            modifiers = modifiers.with(modifier);
        }
        modifiers
    }
}
