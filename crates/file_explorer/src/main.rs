use gpui::{
    div, px, rgb, size, App, AppContext, Bounds, ParentElement, Render, Styled, VisualContext,
    WindowBounds, WindowOptions,
};
use ui::TitleBar;

struct Main {}

impl Render for Main {
    fn render(&mut self, _cx: &mut gpui::ViewContext<Self>) -> impl gpui::prelude::IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .child(TitleBar::new("title_bar"))
            .child(
                div()
                    .flex()
                    .bg(rgb(0x1c1a1e))
                    .size_full()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .w(px(250.0))
                            .bg(rgb(0x282c34))
                            .child(div().p(px(10.0)).child("Sidebar"))
                            .child(div().flex_1().child(div())),
                    )
                    .child(
                        div()
                            .flex_1()
                            .bg(rgb(0x1c1a1e))
                            .child(div().p(px(10.0)).child("File Explorer Main Area"))
                            .child(div().flex_1().child(div())),
                    ),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(600.), px(600.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |cx| cx.new_view(|_cx| Main {}),
        );
    })
}
