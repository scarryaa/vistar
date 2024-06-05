use gpui::{
    div, px, rgb, rgba, ClickEvent, InteractiveElement, IntoElement, ParentElement, RenderOnce,
    Styled, WindowContext,
};

use crate::Clickable;

#[derive(IntoElement)]
pub struct FileItem {
    label: String,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl FileItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_click: None,
        }
    }
}

impl Clickable for FileItem {
    fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for FileItem {
    fn render(self, _cx: &mut gpui::WindowContext) -> impl IntoElement {
        div()
            .line_height(px(30.))
            .px(px(5.))
            .text_color(rgb(0xf3f3f3))
            .child(self.label)
            .hover(|style| style.bg(rgba(0xffffff05)))
    }
}
