use gpui::{
    div, px, rgb, size, App, AppContext, Bounds, IntoElement, ParentElement, Render, Styled,
    VisualContext, WindowBounds, WindowOptions,
};
use ui::{Clickable, FileItem, TitleBar};

pub struct Main {
    selected_item: String,
}

impl Main {
    pub fn new(selected_item: String) -> Self {
        Self { selected_item }
    }

    fn render_main_view(&self) -> impl IntoElement {
        div().child(format!("Viewing: {}", self.selected_item))
    }
}

impl Render for Main {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let make_file_item = |label: &str, cx: &mut gpui::ViewContext<Self>| {
            let label_clone = label.to_string();

            FileItem::new(label).on_click(cx.listener(move |this, _event, cx| {
                this.selected_item = label_clone.clone();
                cx.notify();
            }))
        };

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
                        div().flex().flex_col().w(px(250.)).bg(rgb(0x282c34)).child(
                            div().flex_1().child(
                                div()
                                    .flex_col()
                                    .child(make_file_item("Favorites", cx))
                                    .child(make_file_item("Home", cx))
                                    .child(make_file_item("Documents", cx))
                                    .child(make_file_item("Downloads", cx))
                                    .child(make_file_item("Music", cx))
                                    .child(make_file_item("Pictures", cx))
                                    .child(make_file_item("Videos", cx)),
                            ),
                        ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .bg(rgb(0x1c1a1e))
                            .text_color(rgb(0xffffff))
                            .child(div().flex_1().child(self.render_main_view())),
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
            |cx| {
                cx.new_view(|_cx| Main {
                    selected_item: "Favorites".to_string(),
                })
            },
        );
    })
}
