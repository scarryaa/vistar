use gpui::{
    div, px, rgb, rgba, size, AnyElement, App, AppContext, Bounds, InteractiveElement, IntoElement,
    ParentElement, Render, Styled, VisualContext, WindowBounds, WindowOptions,
};
use lazy_static::lazy_static;
use paths::*;
use std::{
    fs,
    path::{Path, PathBuf},
};
use ui::TitleBar;

#[cfg(target_os = "linux")]
mod paths {
    use super::*;
    use dirs;

    lazy_static! {
        pub static ref HOME: PathBuf =
            dirs::home_dir().expect("failed to determine home directory");
        pub static ref DOCUMENTS: PathBuf =
            dirs::document_dir().expect("Failed to determine documents directory");
        pub static ref DOWNLOADS: PathBuf =
            dirs::download_dir().expect("Failed to determine downloads directory");
        pub static ref MUSIC: PathBuf =
            dirs::audio_dir().expect("Failed to determine music directory");
        pub static ref PICTURES: PathBuf =
            dirs::picture_dir().expect("Failed to determine pictures directory");
        pub static ref VIDEOS: PathBuf =
            dirs::video_dir().expect("Failed to determine videos directory");
        pub static ref LOCAL: PathBuf = HOME.join(".local");
        pub static ref TRASH: PathBuf = HOME.join(".local/share/Trash/files");
        pub static ref RECENT: PathBuf = LOCAL.join("share/file_explorer/recent");
        pub static ref FAVORITES: PathBuf = LOCAL.join("share/file_explorer/favorites");
    }
}

#[cfg(target_os = "windows")]
mod paths {
    use super::*;
    use dirs;

    lazy_static! {
        pub static ref HOME: PathBuf =
            dirs::data_dir().expect("failed to determine data directory");
        pub static ref DOCUMENTS: PathBuf =
            dirs::document_dir().expect("Failed to determine documents directory");
        pub static ref DOWNLOADS: PathBuf =
            dirs::download_dir().expect("Failed to determine downloads directory");
        pub static ref MUSIC: PathBuf =
            dirs::audio_dir().expect("Failed to determine music directory");
        pub static ref PICTURES: PathBuf =
            dirs::picture_dir().expect("Failed to determine pictures directory");
        pub static ref VIDEOS: PathBuf =
            dirs::video_dir().expect("Failed to determine videos directory");
        pub static ref LOCAL: PathBuf =
            dirs::data_dir().expect("Failed to determine data directory");
        pub static ref TRASH: PathBuf = LOCAL.join("Trash");
        pub static ref RECENT: PathBuf = LOCAL.join("file_explorer/recent");
        pub static ref FAVORITES: PathBuf = LOCAL.join("file_explorer/favorites");
    }
}

#[cfg(target_os = "macos")]
mod paths {
    use super::*;
    use dirs;

    lazy_static! {
        pub static ref HOME: PathBuf =
            dirs::home_dir().expect("failed to determine home directory");
        pub static ref DOCUMENTS: PathBuf =
            dirs::document_dir().expect("Failed to determine documents directory");
        pub static ref DOWNLOADS: PathBuf =
            dirs::download_dir().expect("Failed to determine downloads directory");
        pub static ref MUSIC: PathBuf =
            dirs::audio_dir().expect("Failed to determine music directory");
        pub static ref PICTURES: PathBuf =
            dirs::picture_dir().expect("Failed to determine pictures directory");
        pub static ref VIDEOS: PathBuf =
            dirs::video_dir().expect("Failed to determine videos directory");
        pub static ref LOCAL: PathBuf = HOME.join(".local");
        pub static ref TRASH: PathBuf = HOME.join(".local/share/Trash/files");
        pub static ref RECENT: PathBuf = LOCAL.join("share/file_explorer/recent");
        pub static ref FAVORITES: PathBuf = LOCAL.join("share/file_explorer/favorites");
    }
}

pub struct Main {
    text: String,
    folder_contents: Vec<String>,
}

impl Main {
    fn check_or_create_folder(&self, folder: &Path) {
        if !folder.exists() {
            match fs::create_dir_all(folder) {
                Ok(_) => println!("Directory created: {}", folder.display()),
                Err(e) => eprintln!(
                    "Failed to create directory: {}. Error: {}",
                    folder.display(),
                    e
                ),
            }
        }
    }

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

    fn initialize_directories(&self) {
        self.check_or_create_folder(&RECENT);
        self.check_or_create_folder(&FAVORITES);
    }
}

impl Render for Main {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let make_sidebar_item = |label: &str, folder: &Path, cx: &mut gpui::ViewContext<Self>| {
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
                        this.fetch_folder_contents(folder_owned.to_str().unwrap());
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
                                    .child(make_sidebar_item("Recent", &RECENT, cx))
                                    .child(make_sidebar_item("Favorites", &FAVORITES, cx))
                                    .child(make_sidebar_item("Home", &HOME, cx))
                                    .child(make_sidebar_item("Documents", &DOCUMENTS, cx))
                                    .child(make_sidebar_item("Downloads", &DOWNLOADS, cx))
                                    .child(make_sidebar_item("Music", &MUSIC, cx))
                                    .child(make_sidebar_item("Pictures", &PICTURES, cx))
                                    .child(make_sidebar_item("Videos", &VIDEOS, cx))
                                    .child(make_sidebar_item("Trash", &TRASH, cx)),
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
                                div().flex_1().p(px(16.)).child(
                                    div().flex_col().children(self.folder_contents_elements()),
                                ),
                            ),
                    ),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let main_view = Main {
            text: "Favorites".into(),
            folder_contents: vec![],
        };

        main_view.initialize_directories();

        let bounds = Bounds::centered(None, size(px(600.), px(600.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            move |cx| cx.new_view(|_cx| main_view),
        );
    })
}
