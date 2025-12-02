mod commands;
pub mod controller;
mod menu;
mod search_panel;
pub mod state;
mod ui;

pub use state::AppState;

use druid::text::{FontDescriptor, FontFamily};
use druid::{AppLauncher, Color, PlatformError, WindowDesc, theme};

pub fn run() -> Result<(), PlatformError> {
    let window = WindowDesc::new(ui::build_root())
        .title("RustPad")
        .menu(menu::make_menu)
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
