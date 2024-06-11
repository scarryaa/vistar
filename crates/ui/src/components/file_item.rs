use std::{
    fs::{self, Metadata},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::SystemTime,
};

use gpui::{
    div, px, rgb, rgba, svg, white, AnyElement, InteractiveElement, IntoElement, ParentElement,
    Styled,
};

pub struct FileItem {
    path: PathBuf,
    name: String,
    metadata: Metadata,
    on_click: Option<Arc<Mutex<dyn FnMut(&str) + Send + Sync>>>,
    is_folder: bool,
}

impl FileItem {
    pub fn new(
        path: &Path,
        on_click: Option<Arc<Mutex<dyn FnMut(&str) + Send + Sync>>>,
        is_folder: bool,
    ) -> Self {
        let metadata = fs::metadata(path).expect("Unable to read metadata");
        let name = path.file_name().unwrap().to_string_lossy().into_owned();

        Self {
            path: path.to_path_buf(),
            name,
            metadata,
            on_click,
            is_folder,
        }
    }

    fn format_metadata(&self) -> String {
        let size = self.metadata.len();
        let modified = self.metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        let modified: chrono::DateTime<chrono::Utc> = modified.into();
        format!(
            "{} bytes, modified: {}",
            size,
            modified.format("%Y-%m-%d %H:%M:%S")
        )
    }
}

impl IntoElement for FileItem {
    type Element = AnyElement;

    fn into_element(self) -> AnyElement {
        let path_clone = self.path.clone();
        let click_handler = self.on_click.clone();
        let icon_path = if self.is_folder {
            "icons/file_icons/folder.svg"
        } else {
            "icons/file_icons/file_text.svg"
        };

        div()
            .rounded(px(8.))
            .px(px(10.))
            .py(px(5.))
            .hover(|style| style.bg(rgba(0xffffff0d)))
            .on_mouse_down(gpui::MouseButton::Left, move |_event, _cx| {
                if let Some(handler) = click_handler.clone() {
                    let mut handler = handler.lock().unwrap();
                    handler(&path_clone.to_str().unwrap());
                }
            })
            .child(
                div()
                    .w(px(60.))
                    .flex()
                    .flex_col()
                    .items_center()
                    .content_center()
                    .justify_center()
                    .children([
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .content_center()
                            .justify_center()
                            .child(
                                svg()
                                    .path(icon_path)
                                    .w(px(45.))
                                    .h(px(45.))
                                    .text_color(white()),
                            ),
                        div().flex().flex_wrap().child(self.name.clone()),
                    ]),
            )
            .into_any_element()
    }
}
