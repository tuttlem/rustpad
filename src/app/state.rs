use std::path::PathBuf;
use std::sync::Arc;

use druid::text::{FontDescriptor, FontFamily};
use druid::{Data, Lens};

use crate::editor::{EditorMetrics, SelectionState};
use crate::search::SearchRequest;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub text: String,
    pub file_path: Option<Arc<String>>,
    pub saved_snapshot: String,
    pub word_wrap: bool,
    pub show_status_bar: bool,
    pub metrics: EditorMetrics,
    pub selection: SelectionState,
    pub font: FontSettings,
    pub info_message: Option<String>,
    pub search: SearchPanelState,
    pub search_visible: bool,
    pub search_mode: SearchMode,
    pub last_search: Option<SearchRequest>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            text: String::new(),
            file_path: None,
            saved_snapshot: String::new(),
            word_wrap: false,
            show_status_bar: true,
            metrics: EditorMetrics::default(),
            selection: SelectionState::default(),
            font: FontSettings::default(),
            info_message: None,
            search: SearchPanelState::default(),
            search_visible: false,
            search_mode: SearchMode::Find,
            last_search: None,
        }
    }

    pub fn display_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|arc| {
                PathBuf::from(arc.as_str())
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
            })
            .unwrap_or_else(|| "Untitled".to_string())
    }

    pub fn is_dirty(&self) -> bool {
        self.text != self.saved_snapshot
    }

    pub fn mark_saved(&mut self) {
        self.saved_snapshot = self.text.clone();
    }

    pub fn font_descriptor(&self) -> FontDescriptor {
        self.font.descriptor()
    }

    pub fn pathbuf(&self) -> Option<PathBuf> {
        self.file_path
            .as_ref()
            .map(|arc| PathBuf::from(arc.as_str()))
    }
}

#[derive(Clone, Data, Lens)]
pub struct FontSettings {
    pub choice: FontChoice,
    pub size: f64,
}

impl Default for FontSettings {
    fn default() -> Self {
        Self {
            choice: FontChoice::Consolas,
            size: 15.0,
        }
    }
}

impl FontSettings {
    pub fn descriptor(&self) -> FontDescriptor {
        let family = match self.choice {
            FontChoice::Consolas => FontFamily::new_unchecked("Consolas"),
            FontChoice::Courier => FontFamily::new_unchecked("Courier New"),
            FontChoice::Arial => FontFamily::new_unchecked("Arial"),
            FontChoice::Times => FontFamily::new_unchecked("Times New Roman"),
        };
        FontDescriptor::new(family).with_size(self.size)
    }
}

#[derive(Clone, Copy, Data, PartialEq, Eq)]
pub enum FontChoice {
    Consolas,
    Courier,
    Arial,
    Times,
}

#[derive(Clone, Data, Lens)]
pub struct SearchPanelState {
    pub query: String,
    pub replacement: String,
    pub goto_line: String,
    pub match_case: bool,
    pub search_down: bool,
    pub wrap: bool,
}

impl Default for SearchPanelState {
    fn default() -> Self {
        Self {
            query: String::new(),
            replacement: String::new(),
            goto_line: String::new(),
            match_case: false,
            search_down: true,
            wrap: true,
        }
    }
}

#[derive(Clone, Copy, Data, PartialEq, Eq)]
pub enum SearchMode {
    Find,
    Replace,
    GoTo,
}
