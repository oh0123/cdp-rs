use std::sync::Arc;

use cdp_core::{Browser, Page};

fn assert_send_future<F>(_future: F)
where
    F: std::future::Future + Send + 'static,
{
}

#[test]
fn launcher_connect_existing_future_is_send() {
    assert_send_future(
        Browser::launcher()
            .connect_to_existing("http://127.0.0.1:9222")
            .launch(),
    );
}

#[test]
fn launcher_default_future_is_send() {
    assert_send_future(Browser::launcher().launch());
}

#[test]
fn keyboard_type_text_future_is_send() {
    fn assert_keyboard(page: Arc<Page>) {
        assert_send_future(async move {
            let keyboard = page.keyboard();
            keyboard
                .type_text_with_delay("hello", 10, 30)
                .await
                .unwrap();
        });
    }

    let _ = assert_keyboard;
}

#[test]
fn mouse_drag_future_is_send() {
    fn assert_mouse(page: Arc<Page>) {
        assert_send_future(async move {
            let mouse = page.mouse();
            mouse
                .drag_to(
                    0.0,
                    0.0,
                    10.0,
                    10.0,
                    cdp_core::input::mouse::DragOptions::default(),
                )
                .await
                .unwrap();
        });
    }

    let _ = assert_mouse;
}
