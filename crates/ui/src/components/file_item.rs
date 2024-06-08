use std::{
    fs::{self, Metadata},
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::SystemTime,
};

use gpui::{div, px, rgb, AnyElement, InteractiveElement, IntoElement, ParentElement, Styled};

pub struct FileItem {
    path: PathBuf,
    name: String,
    metadata: Metadata,
    on_click: Option<Arc<Mutex<dyn FnMut(&str) + Send + Sync>>>,
}

impl FileItem {
    pub fn new(path: &Path, on_click: Option<Arc<Mutex<dyn FnMut(&str) + Send + Sync>>>) -> Self {
        let metadata = fs::metadata(path).expect("Unable to read metadata");
        let name = path.file_name().unwrap().to_string_lossy().into_owned();

        Self {
            path: path.to_path_buf(),
            name,
            metadata,
            on_click,
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

        div()
            .rounded(px(8.))
            .px(px(10.))
            .py(px(5.))
            .bg(rgb(0x333333))
            .hover(|style| style.bg(rgb(0x444444)))
            .on_mouse_down(gpui::MouseButton::Left, move |_event, _cx| {
                if let Some(handler) = click_handler.clone() {
                    let mut handler = handler.lock().unwrap();
                    handler(&path_clone.to_str().unwrap());
                }
            })
            .child(
                div()
                    .flex()
                    .flex_row()
                    .justify_between()
                    .child(div().child(self.name.clone()))
                    .child(
                        div()
                            .text_color(rgb(0xAAAAAA))
                            .child(self.format_metadata()),
                    ),
            )
            .into_any_element()
    }
}
