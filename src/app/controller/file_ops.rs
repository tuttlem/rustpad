use std::fs;
use std::sync::Arc;

use druid::commands::SHOW_SAVE_PANEL;
use druid::{EventCtx, FileDialogOptions, FileInfo, FileSpec};

use crate::app::state::AppState;

pub fn new_file(data: &mut AppState) {
    data.text.clear();
    data.file_path = None;
    data.mark_saved();
}

pub fn save(ctx: &mut EventCtx, data: &mut AppState, save_as: bool) {
    if !save_as {
        if let Some(path) = data.pathbuf() {
            if let Err(err) = fs::write(&path, &data.text) {
                data.info_message = Some(format!("Unable to save file: {err}"));
            } else {
                data.mark_saved();
            }
            return;
        }
    }
    let options = FileDialogOptions::new()
        .allowed_types(vec![FileSpec::new("Text", &["txt", "md", "rs", ""])])
        .default_type(FileSpec::new("Text", &["txt"]))
        .name_label("Document")
        .title("Save As")
        .button_text("Save");
    ctx.submit_command(SHOW_SAVE_PANEL.with(options));
}

pub fn handle_save_selection(data: &mut AppState, info: &FileInfo) {
    let path = info.path().to_owned();
    if let Err(err) = fs::write(&path, &data.text) {
        data.info_message = Some(format!("Unable to save file: {err}"));
    } else {
        data.file_path = Some(Arc::new(path.to_string_lossy().to_string()));
        data.mark_saved();
    }
}

pub fn handle_open_selection(data: &mut AppState, info: &FileInfo) {
    let path = info.path().to_owned();
    match fs::read_to_string(&path) {
        Ok(contents) => {
            data.text = contents;
            data.file_path = Some(Arc::new(path.to_string_lossy().to_string()));
            data.mark_saved();
        }
        Err(err) => data.info_message = Some(format!("Unable to open file: {err}")),
    }
}
