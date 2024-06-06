use gpui::{
    div, px, rgb, rgba, InteractiveElement, IntoElement, ParentElement, RenderOnce, Styled,
};

#[derive(IntoElement)]
pub struct FileItem {
    label: String,
}

impl FileItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
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
