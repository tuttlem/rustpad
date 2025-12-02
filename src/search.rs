use druid::Data;

use crate::editor::CharRange;

#[derive(Clone, Data, Debug, PartialEq, Eq)]
pub struct SearchRequest {
    pub needle: String,
    pub match_case: bool,
    pub search_down: bool,
    pub wrap: bool,
}

impl SearchRequest {
    pub fn new(needle: String, match_case: bool, search_down: bool, wrap: bool) -> Self {
        Self {
            needle,
            match_case,
            search_down,
            wrap,
        }
    }
}

pub fn find_forward(text: &str, request: &SearchRequest, start: usize) -> Option<CharRange> {
    if request.needle.is_empty() {
        return None;
    }
    let chars: Vec<char> = text.chars().collect();
    let needle: Vec<char> = request.needle.chars().collect();
    if needle.is_empty() {
        return None;
    }
    let mut index = start.min(chars.len());
    let mut wrapped = false;
    loop {
        while index + needle.len() <= chars.len() {
            if matches_at(&chars, index, &needle, request.match_case) {
                return Some(CharRange {
                    start: index,
                    end: index + needle.len(),
                });
            }
            index += 1;
        }
        if request.wrap && !wrapped {
            index = 0;
            wrapped = true;
            continue;
        }
        break;
    }
    None
}

pub fn find_backward(text: &str, request: &SearchRequest, start: usize) -> Option<CharRange> {
    if request.needle.is_empty() {
        return None;
    }
    let chars: Vec<char> = text.chars().collect();
    let needle: Vec<char> = request.needle.chars().collect();
    if needle.is_empty() {
        return None;
    }
    let mut index = start.min(chars.len());
    let mut wrapped = false;
    loop {
        if index < needle.len() {
            if request.wrap && !wrapped {
                index = chars.len();
                wrapped = true;
            } else {
                break;
            }
        }
        while index >= needle.len() {
            let candidate = index - needle.len();
            if matches_at(&chars, candidate, &needle, request.match_case) {
                return Some(CharRange {
                    start: candidate,
                    end: candidate + needle.len(),
                });
            }
            if candidate == 0 {
                break;
            }
            index -= 1;
        }
        if request.wrap && !wrapped {
            index = chars.len();
            wrapped = true;
            continue;
        }
        break;
    }
    None
}

fn matches_at(haystack: &[char], pos: usize, needle: &[char], match_case: bool) -> bool {
    haystack
        .get(pos..pos + needle.len())
        .map(|slice| {
            slice.iter().zip(needle.iter()).all(|(a, b)| {
                if match_case {
                    a == b
                } else {
                    a.eq_ignore_ascii_case(b)
                }
            })
        })
        .unwrap_or(false)
}
