use chrono::Local;
use druid::EventCtx;

use crate::app::commands::EDITOR_ID;
use crate::app::state::{AppState, FontChoice};
use crate::editor::{APPLY_SELECTION, ByteRange, CharRange, SelectionState};

use super::text_utils::char_to_byte;

pub fn insert_timestamp(ctx: &mut EventCtx, data: &mut AppState) {
    let stamp = Local::now().format("%I:%M %p %m/%d/%Y").to_string();
    insert_text(ctx, data, &stamp);
}

pub fn toggle_wrap(data: &mut AppState) {
    data.word_wrap = !data.word_wrap;
    if data.word_wrap {
        data.show_status_bar = false;
    }
}

pub fn toggle_status_bar(data: &mut AppState) {
    if !data.word_wrap {
        data.show_status_bar = !data.show_status_bar;
    }
}

pub fn show_about(data: &mut AppState) {
    data.info_message = Some("RustPad\nA retro-inspired editor built with Druid.".to_string());
}

pub fn set_font_choice(data: &mut AppState, choice: FontChoice) -> bool {
    if data.font.choice != choice {
        data.font.choice = choice;
        true
    } else {
        false
    }
}

pub fn increase_font_size(data: &mut AppState) -> bool {
    adjust_font_size(data, 1.0)
}

pub fn decrease_font_size(data: &mut AppState) -> bool {
    adjust_font_size(data, -1.0)
}

fn adjust_font_size(data: &mut AppState, delta: f64) -> bool {
    const MIN_FONT_SIZE: f64 = 8.0;
    const MAX_FONT_SIZE: f64 = 48.0;
    let new_size = (data.font.size + delta).clamp(MIN_FONT_SIZE, MAX_FONT_SIZE);
    if (new_size - data.font.size).abs() > f64::EPSILON {
        data.font.size = new_size;
        true
    } else {
        false
    }
}

fn insert_text(ctx: &mut EventCtx, data: &mut AppState, value: &str) {
    let mut new_text = String::new();
    let start = char_to_byte(&data.text, data.selection.char_range.start);
    let end = char_to_byte(&data.text, data.selection.char_range.end);
    new_text.push_str(&data.text[..start]);
    new_text.push_str(value);
    new_text.push_str(&data.text[end..]);
    data.text = new_text;
    let byte = start + value.len();
    data.selection = SelectionState {
        char_range: CharRange {
            start: data.selection.char_range.start + value.chars().count(),
            end: data.selection.char_range.start + value.chars().count(),
        },
        byte_range: ByteRange {
            start: byte,
            end: byte,
        },
    };
    ctx.submit_command(
        APPLY_SELECTION
            .with(ByteRange {
                start: byte,
                end: byte,
            })
            .to(EDITOR_ID),
    );
}
