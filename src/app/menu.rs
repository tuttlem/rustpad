use druid::commands::SHOW_OPEN_PANEL;
use druid::keyboard_types::Key;
use druid::menu::MenuEventCtx;
use druid::{Env, FileDialogOptions, FileSpec, Menu, MenuItem, SysMods, WindowId, commands};

use super::commands::{
    CMD_DECREASE_FONT, CMD_EXIT, CMD_FIND_NEXT, CMD_FIND_PREV, CMD_INCREASE_FONT, CMD_NEW_FILE,
    CMD_SAVE_AS, CMD_SAVE_FILE, CMD_SET_FONT, CMD_SHOW_ABOUT, CMD_SHOW_SEARCH, CMD_TIME_DATE,
    CMD_TOGGLE_STATUS, CMD_TOGGLE_WRAP,
};
use super::state::{AppState, FontChoice, SearchMode};

pub fn make_menu(_window: Option<WindowId>, _state: &AppState, _env: &Env) -> Menu<AppState> {
    let file = Menu::new("File")
        .entry(
            MenuItem::new("New")
                .command(CMD_NEW_FILE)
                .hotkey(SysMods::Cmd, "n"),
        )
        .entry(
            MenuItem::new("Open...")
                .hotkey(SysMods::Cmd, "o")
                .on_activate(|ctx, _, _| show_open_dialog(ctx)),
        )
        .entry(
            MenuItem::new("Save")
                .command(CMD_SAVE_FILE)
                .hotkey(SysMods::Cmd, "s"),
        )
        .entry(
            MenuItem::new("Save As...")
                .command(CMD_SAVE_AS)
                .hotkey(SysMods::CmdShift, Key::Character("F".into())),
        )
        .separator()
        .entry(MenuItem::new("Exit").command(CMD_EXIT));

    let edit = Menu::new("Edit")
        .entry(
            MenuItem::new("Undo")
                .command(commands::UNDO)
                .hotkey(SysMods::Cmd, "z"),
        )
        .entry(
            MenuItem::new("Redo")
                .command(commands::REDO)
                .hotkey(SysMods::CmdShift, "z"),
        )
        .separator()
        .entry(
            MenuItem::new("Cut")
                .command(commands::CUT)
                .hotkey(SysMods::Cmd, "x"),
        )
        .entry(
            MenuItem::new("Copy")
                .command(commands::COPY)
                .hotkey(SysMods::Cmd, "c"),
        )
        .entry(
            MenuItem::new("Paste")
                .command(commands::PASTE)
                .hotkey(SysMods::Cmd, "v"),
        )
        .separator()
        .entry(
            MenuItem::new("Find...")
                .command(CMD_SHOW_SEARCH.with(SearchMode::Find))
                .hotkey(SysMods::Cmd, "f"),
        )
        .entry(
            MenuItem::new("Find Next")
                .command(CMD_FIND_NEXT)
                .hotkey(SysMods::None, Key::F3),
        )
        .entry(
            MenuItem::new("Find Previous")
                .command(CMD_FIND_PREV)
                .hotkey(SysMods::Shift, Key::F3),
        )
        .entry(
            MenuItem::new("Replace...")
                .command(CMD_SHOW_SEARCH.with(SearchMode::Replace))
                .hotkey(SysMods::Cmd, "h"),
        )
        .entry(
            MenuItem::new("Go To...")
                .command(CMD_SHOW_SEARCH.with(SearchMode::GoTo))
                .hotkey(SysMods::Cmd, "g"),
        )
        .separator()
        .entry(
            MenuItem::new("Select All")
                .command(commands::SELECT_ALL)
                .hotkey(SysMods::Cmd, "a"),
        )
        .entry(
            MenuItem::new("Time/Date")
                .command(CMD_TIME_DATE)
                .hotkey(SysMods::CmdShift, "T"),
        );

    let font_menu = Menu::new("Font")
        .entry(MenuItem::new("Consolas").command(CMD_SET_FONT.with(FontChoice::Consolas)))
        .entry(MenuItem::new("Courier").command(CMD_SET_FONT.with(FontChoice::Courier)))
        .entry(MenuItem::new("Arial").command(CMD_SET_FONT.with(FontChoice::Arial)))
        .entry(MenuItem::new("Times").command(CMD_SET_FONT.with(FontChoice::Times)));

    let format = Menu::new("Format")
        .entry(MenuItem::new("Word Wrap").command(CMD_TOGGLE_WRAP))
        .separator()
        .entry(
            MenuItem::new("Increase Font Size")
                .command(CMD_INCREASE_FONT)
                .hotkey(SysMods::Cmd, "="),
        )
        .entry(
            MenuItem::new("Decrease Font Size")
                .command(CMD_DECREASE_FONT)
                .hotkey(SysMods::Cmd, "-"),
        )
        .separator()
        .entry(font_menu);

    let view = Menu::new("View").entry(MenuItem::new("Status Bar").command(CMD_TOGGLE_STATUS));

    let help = Menu::new("Help").entry(MenuItem::new("About RustPad").command(CMD_SHOW_ABOUT));

    Menu::empty()
        .entry(file)
        .entry(edit)
        .entry(format)
        .entry(view)
        .entry(help)
}

fn show_open_dialog(ctx: &mut MenuEventCtx) {
    let options = FileDialogOptions::new()
        .allowed_types(vec![FileSpec::new("Text", &["txt", "md", "rs", ""])])
        .name_label("Document")
        .title("Open");
    ctx.submit_command(SHOW_OPEN_PANEL.with(options));
}
