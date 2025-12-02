use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use chrono::Local;
use druid::commands::{OPEN_FILE, SAVE_FILE_AS, SHOW_OPEN_PANEL, SHOW_SAVE_PANEL};
use druid::text::{FontDescriptor, FontFamily};
use druid::theme;
use druid::widget::{Button, Checkbox, Controller, Either, Flex, Label, TextBox, ViewSwitcher};
use druid::menu::MenuEventCtx;
use druid::keyboard_types::Key;
use druid::{commands, AppLauncher, Color, Data, Env, Event, EventCtx, FileDialogOptions, FileInfo, FileSpec, Lens, LensExt, Menu, MenuItem, PlatformError, Selector, SysMods, Target, Widget, WidgetExt, WidgetId, WindowDesc, WindowId};

use crate::editor::{CharRange, EditorMetrics, EditorWidget, SelectionState, APPLY_SELECTION, ByteRange, EDITOR_FONT_KEY};
use crate::search::{find_backward, find_forward, SearchRequest};

const CMD_NEW_FILE: Selector<()> = Selector::new("rustpad.cmd.new");
const CMD_SAVE_FILE: Selector<()> = Selector::new("rustpad.cmd.save");
const CMD_SAVE_AS: Selector<()> = Selector::new("rustpad.cmd.save-as");
const CMD_EXIT: Selector<()> = Selector::new("rustpad.cmd.exit");
const CMD_TIME_DATE: Selector<()> = Selector::new("rustpad.cmd.time-date");
const CMD_TOGGLE_WRAP: Selector<()> = Selector::new("rustpad.cmd.wrap");
const CMD_TOGGLE_STATUS: Selector<()> = Selector::new("rustpad.cmd.status");
const CMD_SHOW_ABOUT: Selector<()> = Selector::new("rustpad.cmd.about");
const CMD_SHOW_SEARCH: Selector<SearchMode> = Selector::new("rustpad.cmd.show-search");
const CMD_FIND_NEXT: Selector<()> = Selector::new("rustpad.cmd.find-next");
const CMD_FIND_PREV: Selector<()> = Selector::new("rustpad.cmd.find-prev");
const CMD_REPLACE_ONE: Selector<()> = Selector::new("rustpad.cmd.replace-one");
const CMD_REPLACE_ALL: Selector<()> = Selector::new("rustpad.cmd.replace-all");
const CMD_GO_TO: Selector<()> = Selector::new("rustpad.cmd.goto");

const EDITOR_ID: WidgetId = WidgetId::reserved(1);

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

    #[allow(dead_code)]
    pub fn display_name(&self) -> String {
        self.file_path
            .as_ref()
            .and_then(|arc| PathBuf::from(arc.as_str()).file_name().map(|n| n.to_string_lossy().to_string()))
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
    fn descriptor(&self) -> FontDescriptor {
        let family = match self.choice {
            FontChoice::Consolas | FontChoice::Courier => druid::text::FontFamily::MONOSPACE,
            FontChoice::Arial | FontChoice::Times => druid::text::FontFamily::SANS_SERIF,
        };
        FontDescriptor::new(family).with_size(self.size)
    }
}

#[derive(Clone, Copy, Data, PartialEq, Eq)]
#[allow(dead_code)]
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

pub fn run() -> Result<(), PlatformError> {
    let window = WindowDesc::new(build_root())
        .title("RustPad")
        .menu(make_menu)
        .window_size((900.0, 640.0));

    AppLauncher::with_window(window)
        .configure_env(|env, _| {
            env.set(theme::WINDOW_BACKGROUND_COLOR, Color::WHITE);
            env.set(theme::BACKGROUND_LIGHT, Color::WHITE);
            env.set(theme::BACKGROUND_DARK, Color::WHITE);
            env.set(theme::TEXT_COLOR, Color::BLACK);
            env.set(theme::BUTTON_LIGHT, Color::rgb8(0xF5, 0xF5, 0xF5));
            env.set(theme::BUTTON_DARK, Color::rgb8(0xE0, 0xE0, 0xE0));
            env.set(theme::BORDER_DARK, Color::grey(0.5));
            env.set(theme::UI_FONT, FontDescriptor::new(FontFamily::SANS_SERIF));
        })
        .launch(AppState::new())
}

fn build_root() -> impl Widget<AppState> {
    let editor_switcher = ViewSwitcher::new(
        |data: &AppState, _| data.word_wrap,
        |wrap, _data, _env| {
            Box::new(
                EditorWidget::new(*wrap)
                    .with_id(EDITOR_ID)
                    .env_scope(|env, data: &AppState| {
                        env.set(EDITOR_FONT_KEY, data.font_descriptor());
                    }),
            )
        },
    )
    .expand();

    let status_bar = Flex::row()
        .with_child(Label::dynamic(|data: &AppState, _| format!("Ln {}, Col {}", data.metrics.line, data.metrics.column)))
        .with_spacer(12.0)
        .with_child(Label::dynamic(|data: &AppState, _| format!("Sel {}", data.metrics.selection)))
        .with_flex_spacer(1.0)
        .with_child(Label::new("UTF-8"))
        .padding((6.0, 2.0))
        .border(Color::grey(0.7), 1.0);

    let message = Either::new(
        |data: &AppState, _| data.info_message.is_some(),
        Label::dynamic(|data: &AppState, _| data.info_message.clone().unwrap_or_default())
            .padding((6.0, 2.0))
            .background(Color::rgb8(0xFF, 0xF3, 0xC0))
            .border(Color::grey(0.6), 1.0),
        Label::new(""),
    );

    let search_panel = build_search_panel();

    Flex::column()
        .with_child(message)
        .with_child(search_panel)
        .with_flex_child(editor_switcher, 1.0)
        .with_child(Either::new(
            |data: &AppState, _| data.show_status_bar,
            status_bar,
            Label::new(""),
        ))
        .padding(6.0)
        .controller(AppController)
}

fn build_search_panel() -> impl Widget<AppState> {
    Either::new(
        |data: &AppState, _| data.search_visible,
        ViewSwitcher::new(
            |data: &AppState, _| data.search_mode,
            |mode, _data, _env| match mode {
                SearchMode::Find => find_view().boxed(),
                SearchMode::Replace => replace_view().boxed(),
                SearchMode::GoTo => goto_view().boxed(),
            },
        )
        .padding(8.0)
        .border(Color::grey(0.6), 1.0)
        .background(Color::grey8(0xF5)),
        Label::new(""),
    )
}

fn find_view() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new("Find what:"))
                .with_spacer(8.0)
                .with_flex_child(
                    TextBox::new().lens(AppState::search.then(SearchPanelState::query)),
                    1.0,
                ),
        )
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(Checkbox::new("Match case").lens(AppState::search.then(SearchPanelState::match_case)))
                .with_spacer(12.0)
                .with_child(Checkbox::new("Wrap around").lens(AppState::search.then(SearchPanelState::wrap)))
                .with_spacer(12.0)
                .with_child(Checkbox::new("Search down").lens(AppState::search.then(SearchPanelState::search_down))),
        )
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(Button::new("Find Next").on_click(|ctx, _, _| ctx.submit_command(CMD_FIND_NEXT.to(Target::Global))))
                .with_spacer(8.0)
                .with_child(Button::new("Find Previous").on_click(|ctx, _, _| ctx.submit_command(CMD_FIND_PREV.to(Target::Global))))
                .with_spacer(8.0)
                .with_child(Button::new("Close").on_click(|_, data: &mut AppState, _| data.search_visible = false)),
        )
}

fn replace_view() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new("Find what:"))
                .with_spacer(8.0)
                .with_flex_child(
                    TextBox::new().lens(AppState::search.then(SearchPanelState::query)),
                    1.0,
                ),
        )
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(Label::new("Replace with:"))
                .with_spacer(8.0)
                .with_flex_child(
                    TextBox::new().lens(AppState::search.then(SearchPanelState::replacement)),
                    1.0,
                ),
        )
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(Checkbox::new("Match case").lens(AppState::search.then(SearchPanelState::match_case)))
                .with_spacer(12.0)
                .with_child(Checkbox::new("Wrap around").lens(AppState::search.then(SearchPanelState::wrap))),
        )
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(Button::new("Find Next").on_click(|ctx, _, _| ctx.submit_command(CMD_FIND_NEXT.to(Target::Global))))
                .with_spacer(8.0)
                .with_child(Button::new("Replace").on_click(|ctx, _, _| ctx.submit_command(CMD_REPLACE_ONE.to(Target::Global))))
                .with_spacer(8.0)
                .with_child(Button::new("Replace All").on_click(|ctx, _, _| ctx.submit_command(CMD_REPLACE_ALL.to(Target::Global))))
                .with_spacer(8.0)
                .with_child(Button::new("Close").on_click(|_, data: &mut AppState, _| data.search_visible = false)),
        )
}

fn goto_view() -> impl Widget<AppState> {
    Flex::column()
        .with_child(
            Flex::row()
                .with_child(Label::new("Line number:"))
                .with_spacer(8.0)
                .with_flex_child(
                    TextBox::new().lens(AppState::search.then(SearchPanelState::goto_line)),
                    1.0,
                ),
        )
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(Button::new("Go To").on_click(|ctx, _, _| ctx.submit_command(CMD_GO_TO.to(Target::Global))))
                .with_spacer(8.0)
                .with_child(Button::new("Close").on_click(|_, data: &mut AppState, _| data.search_visible = false)),
        )
}

fn make_menu(_window: Option<WindowId>, _state: &AppState, _env: &Env) -> Menu<AppState> {
    let file = Menu::new("File")
        .entry(MenuItem::new("New").command(CMD_NEW_FILE).hotkey(SysMods::Cmd, "n"))
        .entry(
            MenuItem::new("Open...")
                .hotkey(SysMods::Cmd, "o")
                .on_activate(|ctx, _, _| show_open_dialog(ctx)),
        )
        .entry(MenuItem::new("Save").command(CMD_SAVE_FILE).hotkey(SysMods::Cmd, "s"))
        .entry(MenuItem::new("Save As...").command(CMD_SAVE_AS).hotkey(SysMods::CmdShift, Key::Character("F".into())))
        .separator()
        .entry(MenuItem::new("Exit").command(CMD_EXIT));

    let edit = Menu::new("Edit")
        .entry(MenuItem::new("Undo").command(commands::UNDO).hotkey(SysMods::Cmd, "z"))
        .entry(MenuItem::new("Redo").command(commands::REDO).hotkey(SysMods::CmdShift, "z"))
        .separator()
        .entry(MenuItem::new("Cut").command(commands::CUT).hotkey(SysMods::Cmd, "x"))
        .entry(MenuItem::new("Copy").command(commands::COPY).hotkey(SysMods::Cmd, "c"))
        .entry(MenuItem::new("Paste").command(commands::PASTE).hotkey(SysMods::Cmd, "v"))
        .separator()
        .entry(MenuItem::new("Find...").command(CMD_SHOW_SEARCH.with(SearchMode::Find)).hotkey(SysMods::Cmd, "f"))
        .entry(MenuItem::new("Find Next").command(CMD_FIND_NEXT).hotkey(SysMods::None, Key::F3))
        .entry(MenuItem::new("Find Previous").command(CMD_FIND_PREV).hotkey(SysMods::Shift, Key::F3))
        .entry(MenuItem::new("Replace...").command(CMD_SHOW_SEARCH.with(SearchMode::Replace)).hotkey(SysMods::Cmd, "h"))
        .entry(MenuItem::new("Go To...").command(CMD_SHOW_SEARCH.with(SearchMode::GoTo)).hotkey(SysMods::Cmd, "g"))
        .separator()
        .entry(MenuItem::new("Select All").command(commands::SELECT_ALL).hotkey(SysMods::Cmd, "a"))
        .entry(MenuItem::new("Time/Date").command(CMD_TIME_DATE).hotkey(SysMods::CmdShift, "T"));

    let format = Menu::new("Format")
        .entry(MenuItem::new("Word Wrap").command(CMD_TOGGLE_WRAP));

    let view = Menu::new("View")
        .entry(MenuItem::new("Status Bar").command(CMD_TOGGLE_STATUS));

    let help = Menu::new("Help")
        .entry(MenuItem::new("About RustPad").command(CMD_SHOW_ABOUT));

    Menu::empty()
        .entry(file)
        .entry(edit)
        .entry(format)
        .entry(view)
        .entry(help)
}

struct AppController;

impl<W: Widget<AppState>> Controller<AppState, W> for AppController {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env) {
        match event {
            Event::Command(cmd) if cmd.is(CMD_NEW_FILE) => {
                data.text.clear();
                data.file_path = None;
                data.mark_saved();
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_SAVE_FILE) => {
                self.save(ctx, data, false);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_SAVE_AS) => {
                self.save(ctx, data, true);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_EXIT) => {
                ctx.submit_command(commands::QUIT_APP);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_TIME_DATE) => {
                let stamp = Local::now().format("%I:%M %p %m/%d/%Y").to_string();
                self.insert_text(ctx, data, &stamp);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_TOGGLE_WRAP) => {
                data.word_wrap = !data.word_wrap;
                if data.word_wrap {
                    data.show_status_bar = false;
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_TOGGLE_STATUS) => {
                if !data.word_wrap {
                    data.show_status_bar = !data.show_status_bar;
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_SHOW_ABOUT) => {
                data.info_message = Some("RustPad\nA retro-inspired editor built with Druid.".to_string());
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_SHOW_SEARCH) => {
                if let Some(mode) = cmd.get(CMD_SHOW_SEARCH) {
                    data.search_mode = *mode;
                }
                data.search_visible = true;
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_FIND_NEXT) => {
                self.run_search(ctx, data, true);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_FIND_PREV) => {
                self.run_search(ctx, data, false);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_REPLACE_ONE) => {
                self.replace_once(ctx, data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_REPLACE_ALL) => {
                self.replace_all(data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_GO_TO) => {
                self.goto_line(ctx, data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(OPEN_FILE) => {
                if let Some(info) = cmd.get::<FileInfo>(OPEN_FILE) {
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
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(SAVE_FILE_AS) => {
                if let Some(info) = cmd.get::<FileInfo>(SAVE_FILE_AS) {
                    let path = info.path().to_owned();
                    if let Err(err) = fs::write(&path, &data.text) {
                        data.info_message = Some(format!("Unable to save file: {err}"));
                    } else {
                        data.file_path = Some(Arc::new(path.to_string_lossy().to_string()));
                        data.mark_saved();
                    }
                }
                ctx.set_handled();
            }
            Event::WindowCloseRequested => {
                if data.is_dirty() {
                    data.info_message = Some("Unsaved changes will be lost.".to_string());
                    ctx.set_handled();
                }
            }
            Event::MouseDown(_) => {
                data.info_message = None;
            }
            _ => {}
        }

        child.event(ctx, event, data, env);
    }
}

impl AppController {
    fn save(&self, ctx: &mut EventCtx, data: &mut AppState, save_as: bool) {
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

    fn insert_text(&self, ctx: &mut EventCtx, data: &mut AppState, value: &str) {
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
            byte_range: ByteRange { start: byte, end: byte },
        };
        ctx.submit_command(APPLY_SELECTION.with(ByteRange { start: byte, end: byte }).to(EDITOR_ID));
    }

    fn panel_request(&self, data: &AppState) -> Option<SearchRequest> {
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

    fn run_search(&self, ctx: &mut EventCtx, data: &mut AppState, forward: bool) {
        let request = self.panel_request(data).or_else(|| data.last_search.clone());
        if request.is_none() {
            data.search_mode = if forward { SearchMode::Find } else { SearchMode::Find };
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
            self.highlight_range(ctx, data, range);
        } else {
            data.info_message = Some(format!("Cannot find \"{}\"", request.needle));
        }
    }

    fn highlight_range(&self, ctx: &mut EventCtx, data: &mut AppState, range: CharRange) {
        let start_byte = char_to_byte(&data.text, range.start);
        let end_byte = char_to_byte(&data.text, range.end);
        ctx.submit_command(APPLY_SELECTION.with(ByteRange { start: start_byte, end: end_byte }).to(EDITOR_ID));
    }

    fn replace_once(&self, ctx: &mut EventCtx, data: &mut AppState) {
        let request = match self.panel_request(data) {
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
            self.run_search(ctx, data, true);
            return;
        }
        let start_byte = data.selection.byte_range.start;
        let end_byte = data.selection.byte_range.end;
        data.text.replace_range(start_byte..end_byte, &data.search.replacement);
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
        ctx.submit_command(APPLY_SELECTION.with(data.selection.byte_range).to(EDITOR_ID));
    }

    fn replace_all(&self, data: &mut AppState) {
        let mut request = match self.panel_request(data) {
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

    fn goto_line(&self, ctx: &mut EventCtx, data: &mut AppState) {
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
                ctx.submit_command(APPLY_SELECTION.with(ByteRange { start: byte, end: byte }).to(EDITOR_ID));
            }
            _ => {
                data.info_message = Some("Invalid line number.".to_string());
            }
        }
    }
}

fn char_to_byte(text: &str, char_index: usize) -> usize {
    if char_index == 0 {
        return 0;
    }
    let mut count = 0;
    for (byte, _) in text.char_indices() {
        if count == char_index {
            return byte;
        }
        count += 1;
    }
    text.len()
}

fn show_open_dialog(ctx: &mut MenuEventCtx) {
    let options = FileDialogOptions::new()
        .allowed_types(vec![FileSpec::new("Text", &["txt", "md", "rs", ""])])
        .name_label("Document")
        .title("Open");
    ctx.submit_command(SHOW_OPEN_PANEL.with(options));
}
