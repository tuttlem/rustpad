use druid::EventCtx;

use crate::app::commands::EDITOR_ID;
use crate::app::state::{AppState, SearchMode};
use crate::editor::{APPLY_SELECTION, ByteRange, CharRange, SelectionState};
use crate::search::{SearchRequest, find_backward, find_forward};

use super::text_utils::char_to_byte;

pub fn show_search(data: &mut AppState, mode: Option<SearchMode>) {
    if let Some(mode) = mode {
        data.search_mode = mode;
    }
    data.search_visible = true;
}

pub fn run_search(ctx: &mut EventCtx, data: &mut AppState, forward: bool) {
    let request = panel_request(data).or_else(|| data.last_search.clone());
    if request.is_none() {
        data.search_mode = SearchMode::Find;
        data.search_visible = true;
        return;
    }
    let mut request = request.unwrap();
    request.search_down = forward;
    let start = if forward {
        data.selection.char_range.end
    } else {
        data.selection.char_range.start
    };
    let result = if forward {
        find_forward(&data.text, &request, start)
    } else {
        find_backward(&data.text, &request, start)
    };
    if let Some(range) = result {
        data.last_search = Some(request);
        highlight_range(ctx, data, range);
    } else {
        data.info_message = Some(format!("Cannot find \"{}\"", request.needle));
    }
}

pub fn replace_once(ctx: &mut EventCtx, data: &mut AppState) {
    let request = match panel_request(data) {
        Some(req) => req,
        None => {
            data.info_message = Some("Enter text to find.".to_string());
            return;
        }
    };
    let selected = &data.text[data.selection.byte_range.start..data.selection.byte_range.end];
    let matches = if request.match_case {
        selected == request.needle
    } else {
        selected.eq_ignore_ascii_case(&request.needle)
    };
    if !matches {
        run_search(ctx, data, true);
        return;
    }
    let start_byte = data.selection.byte_range.start;
    let end_byte = data.selection.byte_range.end;
    data.text
        .replace_range(start_byte..end_byte, &data.search.replacement);
    let inserted_chars = data.search.replacement.chars().count();
    let start_char = data.selection.char_range.start;
    data.selection = SelectionState {
        char_range: CharRange {
            start: start_char + inserted_chars,
            end: start_char + inserted_chars,
        },
        byte_range: ByteRange {
            start: start_byte + data.search.replacement.len(),
            end: start_byte + data.search.replacement.len(),
        },
    };
    ctx.submit_command(
        APPLY_SELECTION
            .with(data.selection.byte_range)
            .to(EDITOR_ID),
    );
}

pub fn replace_all(data: &mut AppState) {
    let mut request = match panel_request(data) {
        Some(req) => req,
        None => {
            data.info_message = Some("Enter text to find.".to_string());
            return;
        }
    };
    request.wrap = false;
    request.search_down = true;
    let mut text = data.text.clone();
    let mut cursor = 0;
    let mut replaced = 0;
    while let Some(range) = find_forward(&text, &request, cursor) {
        let start_byte = char_to_byte(&text, range.start);
        let end_byte = char_to_byte(&text, range.end);
        text.replace_range(start_byte..end_byte, &data.search.replacement);
        cursor = range.start + data.search.replacement.chars().count();
        replaced += 1;
    }
    data.text = text;
    data.info_message = Some(format!("Replaced {replaced} occurrence(s)."));
}

pub fn goto_line(ctx: &mut EventCtx, data: &mut AppState) {
    let input = data.search.goto_line.trim();
    if input.is_empty() {
        data.info_message = Some("Enter a line number.".to_string());
        return;
    }
    match input.parse::<usize>() {
        Ok(target) if target > 0 => {
            let mut current = 1usize;
            let mut char_index = 0usize;
            for ch in data.text.chars() {
                if current == target {
                    break;
                }
                if ch == '\n' {
                    current += 1;
                }
                char_index += 1;
            }
            if current != target {
                data.info_message = Some("Line not found.".to_string());
                return;
            }
            let byte = char_to_byte(&data.text, char_index);
            ctx.submit_command(
                APPLY_SELECTION
                    .with(ByteRange {
                        start: byte,
                        end: byte,
                    })
                    .to(EDITOR_ID),
            );
        }
        _ => {
            data.info_message = Some("Invalid line number.".to_string());
        }
    }
}

fn panel_request(data: &AppState) -> Option<SearchRequest> {
    let needle = data.search.query.trim();
    if needle.is_empty() {
        None
    } else {
        Some(SearchRequest::new(
            needle.to_string(),
            data.search.match_case,
            data.search.search_down,
            data.search.wrap,
        ))
    }
}

fn highlight_range(ctx: &mut EventCtx, data: &mut AppState, range: CharRange) {
    let start_byte = char_to_byte(&data.text, range.start);
    let end_byte = char_to_byte(&data.text, range.end);
    ctx.submit_command(
        APPLY_SELECTION
            .with(ByteRange {
                start: start_byte,
                end: end_byte,
            })
            .to(EDITOR_ID),
    );
}
