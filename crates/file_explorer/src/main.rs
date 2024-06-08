use gpui::{
    black, div, percentage, px, rgb, rgba, size, AnyElement, App, AppContext, Bounds,
    InteractiveElement, IntoElement, ParentElement, Render, Styled, ViewContext, VisualContext,
    WindowBounds, WindowOptions,
};
use lazy_static::lazy_static;
use paths::*;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Child,
    sync::Arc,
};
use ui::{FileItem, TitleBar};

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
    folder_contents: Vec<PathBuf>,
    path: PathBuf,
    drives: Vec<PathBuf>,
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

    #[cfg(target_os = "windows")]
    fn fetch_drives(&mut self) {
        use std::io;
        use std::ptr;
        use winapi::um::fileapi::GetLogicalDriveStringsW;

        unsafe {
            let mut drive_strings = [0u16; 256];
            let result = GetLogicalDriveStringsW(256, drive_strings.as_mut_ptr());
            if result == 0 {
                println!("Failed to get logical drives");
                return;
            }

            let mut drives = Vec::new();
            let mut i = 0;
            while i < result as usize {
                let drive_str =
                    String::from_utf16_lossy(&drive_strings[i..i + 4]).trim_end_matches('\u{0}');
                let drive = PathBuf::from(drive_str);

                // Check for access before adding the drive
                if fs::metadata(&drive).is_ok() {
                    drives.push(drive);
                } else {
                    println!("Access denied to drive: {}", drive_str);
                }
                i += 4;
            }
            self.drives = drives;
        }
    }

    #[cfg(target_family = "unix")]
    fn fetch_drives(&mut self) {
        use sysinfo::{DiskExt, System, SystemExt};

        let sys = System::new_all();
        let drives = sys
            .disks()
            .into_iter()
            .filter_map(|disk| {
                let path = disk.mount_point().to_path_buf();
                // Check for access before adding the drive
                if fs::metadata(&path).is_ok() {
                    Some(path)
                } else {
                    println!("Access denied to drive: {}", path.display());
                    None
                }
            })
            .collect::<Vec<_>>();

        self.drives = drives;
    }

    fn fetch_folder_contents(&mut self, folder: &str) {
        self.folder_contents = fs::read_dir(folder)
            .unwrap()
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();
    }

    fn folder_contents_elements(&self, cx: &mut ViewContext<Self>) -> Vec<AnyElement> {
        self.folder_contents
            .iter()
            .map(|item| {
                FileItem::new(
                    item,
                    Some(Arc::new(|path| println!("File clicked {:?}", path))),
                )
                .into_any_element()
            })
            .collect()
    }

    fn initialize_directories(&self) {
        self.check_or_create_folder(&RECENT);
        self.check_or_create_folder(&FAVORITES);
    }
}

impl Render for Main {
    fn render(&mut self, cx: &mut gpui::ViewContext<Self>) -> impl IntoElement {
        let titlebar = cx.new_view(|_cx| TitleBar::new("title_bar"));

        let make_separator = || {
            div()
                .w_full()
                .h(px(1.))
                .mx(px(5.))
                .bg(rgb(0x545454))
                .rounded(px(8.))
                .mr(px(10.))
                .my(px(6.))
        };

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
                        this.path = folder_owned.clone();
                        this.fetch_folder_contents(folder_owned.to_str().unwrap());
                        cx.notify();
                    }),
                )
        };

        if titlebar.read(cx).path != self.path.to_str().unwrap().to_string() {
            titlebar.update(cx, |_titlebar, _cx| {
                _titlebar.path = self.path.to_str().unwrap().to_string()
            })
        }

        let mut sidebar_items_after_separator = div();
        sidebar_items_after_separator = sidebar_items_after_separator.child(make_separator());

        // Add a sidebar item for each drive in the new div
        for drive in &self.drives {
            sidebar_items_after_separator = sidebar_items_after_separator.child(make_sidebar_item(
                drive.to_str().unwrap(),
                drive,
                cx,
            ));
        }

        let sidebar_items = div()
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
            .child(make_sidebar_item("Trash", &TRASH, cx))
            .child(sidebar_items_after_separator);

        div()
            .rounded_br_lg()
            .rounded_bl_lg()
            .flex()
            .flex_col()
            .size_full()
            .child(titlebar)
            .child(
                div()
                    .rounded_br_lg()
                    .rounded_bl_lg()
                    .flex()
                    .bg(rgb(0x232225))
                    .size_full()
                    .children([
                        div()
                            .flex()
                            .flex_col()
                            .w(px(150.))
                            .bg(rgb(0x19191a))
                            .child(sidebar_items),
                        div()
                            .rounded_br_lg()
                            .rounded_bl_lg()
                            .flex_1()
                            .bg(rgb(0x232225))
                            .text_color(rgb(0xffffff))
                            .child(div().flex_1().p(px(16.)).child(
                                div().flex_col().children(self.folder_contents_elements(cx)),
                            )),
                    ]),
            )
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
        let mut main_view = Main {
            text: "Favorites".into(),
            folder_contents: vec![],
            path: PathBuf::new(),
            drives: vec![],
        };

        main_view.initialize_directories();
        main_view.fetch_drives();

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
