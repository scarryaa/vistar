use gpui::{
    div, px, rgb, rgba, size, AnyElement, App, AppContext, Bounds, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, VisualContext, WindowBounds, WindowOptions,
};
use std::{fs, path::PathBuf};
use ui::TitleBar;

lazy_static::lazy_static! {
    pub static ref HOME: PathBuf = dirs::home_dir().expect("failed to determine home directory");
    pub static ref DOCUMENTS: PathBuf = dirs::document_dir().expect("Failed to determine documents directory");
    pub static ref DOWNLOADS: PathBuf = dirs::download_dir().expect("Failed to determine downloads directory");
    pub static ref MUSIC: PathBuf = dirs::audio_dir().expect("Failed to determine documents directory");
    pub static ref PICTURES: PathBuf = dirs::picture_dir().expect("Failed to determine pictures directory");
    pub static ref VIDEOS: PathBuf = dirs::video_dir().expect("Failed to determine videos directory");
    pub static ref TRASH: PathBuf = PathBuf::new();

}
pub struct Main {
    text: String,
    folder_contents: Vec<String>,
}

impl Main {
    fn fetch_folder_contents(&mut self, folder: &str) {
        self.folder_contents = fs::read_dir(folder)
            .unwrap()
            .filter_map(|entry| entry.ok().and_then(|e| e.file_name().into_string().ok()))
            .collect();
    }

    fn folder_contents_elements(&self) -> Vec<AnyElement> {
        self.folder_contents
            .iter()
            .map(|item| div().child(item.clone()).into_any_element())
            .collect()
    }
}

impl Render for Main {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let make_sidebar_item = |label: &str, folder: &str, cx: &mut gpui::ViewContext<Self>| {
            let label_owned = label.to_owned();
            let folder_owned = folder.to_owned();

            div()
                .rounded(px(8.))
                .line_height(px(35.))
                .px(px(10.))
                .text_color(rgb(0xf3f3f3))
                .child(label_owned.clone())
                .hover(|style| style.bg(rgba(0xffffff05)))
                .on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener(move |this, _event, cx| {
                        this.text = label_owned.clone();
                        this.fetch_folder_contents(&folder_owned);
                        cx.notify();
                    }),
                )
        };

        div()
            .rounded_br_lg()
            .rounded_bl_lg()
            .flex()
            .flex_col()
            .size_full()
            .child(TitleBar::new("title_bar"))
            .child(
                div()
                    .rounded_br_lg()
                    .rounded_bl_lg()
                    .flex()
                    .bg(rgb(0x232225))
                    .size_full()
                    .child(
                        div().flex().flex_col().w(px(150.)).bg(rgb(0x19191a)).child(
                            div().flex_1().child(
                                div()
                                    .rounded_bl_lg()
                                    .px(px(8.))
                                    .py(px(10.))
                                    .flex_col()
                                    .child(make_sidebar_item("Recent", "path/to/recent", cx))
                                    .child(make_sidebar_item("Favorites", "path/to/favorites", cx))
                                    .child(make_sidebar_item("Home", HOME.to_str().unwrap(), cx))
                                    .child(make_sidebar_item(
                                        "Documents",
                                        DOCUMENTS.to_str().unwrap(),
                                        cx,
                                    ))
                                    .child(make_sidebar_item(
                                        "Downloads",
                                        DOWNLOADS.to_str().unwrap(),
                                        cx,
                                    ))
                                    .child(make_sidebar_item("Music", MUSIC.to_str().unwrap(), cx))
                                    .child(make_sidebar_item(
                                        "Pictures",
                                        PICTURES.to_str().unwrap(),
                                        cx,
                                    ))
                                    .child(make_sidebar_item(
                                        "Videos",
                                        VIDEOS.to_str().unwrap(),
                                        cx,
                                    ))
                                    .child(make_sidebar_item("Trash", TRASH.to_str().unwrap(), cx)),
                            ),
                        ),
                    )
                    .child(
                        div()
                            .rounded_br_lg()
                            .rounded_bl_lg()
                            .flex_1()
                            .bg(rgb(0x232225))
                            .text_color(rgb(0xffffff))
                            .child(
                                div()
                                    .flex_1()
                                    .p(px(16.))
                                    .child(div().child(format!("Viewing: {}", self.text.clone())))
                                    .child(
                                        div().flex_col().children(self.folder_contents_elements()),
                                    ),
                            ),
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
                    text: "Favorites".into(),
                    folder_contents: vec![],
                })
            },
        );
    })
}
