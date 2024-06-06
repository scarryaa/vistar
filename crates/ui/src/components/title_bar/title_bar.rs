use gpui::{
    div, px, rgb, transparent_black, ElementId, InteractiveElement, IntoElement, ParentElement,
    Pixels, RenderOnce, Styled, WindowContext,
};

#[derive(IntoElement)]
pub struct TitleBar {}

impl TitleBar {
    #[cfg(not(target_os = "windows"))]
    pub fn height(cx: &mut WindowContext) -> Pixels {
        (1.75 * cx.rem_size()).max(px(34.))
    }

    #[cfg(target_os = "windows")]
    pub fn height(_cx: &mut WindowContext) -> Pixels {
        // todo(windows) instead of hard coded size report the actual size to the Windows platform API
        px(32.)
    }

    pub fn new(_id: impl Into<ElementId>) -> Self {
        Self {}
    }
}

impl RenderOnce for TitleBar {
    fn render(self, cx: &mut gpui::WindowContext) -> impl IntoElement {
        let height = Self::height(cx);
        div()
            .rounded_tr_lg()
            .rounded_tl_lg()
            .bg(transparent_black())
            .h(height)
            .on_mouse_move(move |ev, cx| {
                if ev.dragging() {
                    cx.start_system_move();
                }
            })
            .flex()
            .flex_row()
            .children([
                div()
                    .flex()
                    .h(Self::height(cx))
                    .w(px(150.))
                    .bg(rgb(0x19191a))
                    .rounded_tl_lg(),
                div()
                    .flex()
                    .h(Self::height(cx))
                    .w(px(450.))
                    .bg(rgb(0x232225))
                    .rounded_tr_lg(),
            ])
    }
}
