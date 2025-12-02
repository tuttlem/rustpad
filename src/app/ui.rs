use druid::widget::{Either, Flex, Label, ViewSwitcher};
use druid::{Color, Widget, WidgetExt};

use super::commands::EDITOR_ID;
use super::controller::AppController;
use super::search_panel;
use super::state::AppState;
use crate::editor::{EDITOR_FONT_KEY, EditorWidget};

pub fn build_root() -> impl Widget<AppState> {
    let editor_switcher = ViewSwitcher::new(
        |data: &AppState, _| data.word_wrap,
        |wrap, _data, _env| {
            Box::new(EditorWidget::new(*wrap).with_id(EDITOR_ID).env_scope(
                |env, data: &AppState| {
                    env.set(EDITOR_FONT_KEY, data.font_descriptor());
                },
            ))
        },
    )
    .expand();

    let status_bar = Flex::row()
        .with_child(Label::dynamic(|data: &AppState, _| {
            format!("Ln {}, Col {}", data.metrics.line, data.metrics.column)
        }))
        .with_spacer(12.0)
        .with_child(Label::dynamic(|data: &AppState, _| {
            format!("Sel {}", data.metrics.selection)
        }))
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

    Flex::column()
        .with_child(message)
        .with_child(search_panel::build_search_panel())
        .with_flex_child(editor_switcher, 1.0)
        .with_child(Either::new(
            |data: &AppState, _| data.show_status_bar,
            status_bar,
            Label::new(""),
        ))
        .padding(6.0)
        .controller(AppController)
}
