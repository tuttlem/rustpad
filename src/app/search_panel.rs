use druid::widget::{Button, Checkbox, Either, Flex, Label, TextBox, ViewSwitcher};
use druid::{Color, LensExt, Target, Widget, WidgetExt};

use super::commands::{CMD_FIND_NEXT, CMD_FIND_PREV, CMD_GO_TO, CMD_REPLACE_ALL, CMD_REPLACE_ONE};
use super::state::{AppState, SearchMode, SearchPanelState};

pub fn build_search_panel() -> impl Widget<AppState> {
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
                .with_child(
                    Checkbox::new("Match case")
                        .lens(AppState::search.then(SearchPanelState::match_case)),
                )
                .with_spacer(12.0)
                .with_child(
                    Checkbox::new("Wrap around")
                        .lens(AppState::search.then(SearchPanelState::wrap)),
                )
                .with_spacer(12.0)
                .with_child(
                    Checkbox::new("Search down")
                        .lens(AppState::search.then(SearchPanelState::search_down)),
                ),
        )
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("Find Next")
                        .on_click(|ctx, _, _| ctx.submit_command(CMD_FIND_NEXT.to(Target::Global))),
                )
                .with_spacer(8.0)
                .with_child(
                    Button::new("Find Previous")
                        .on_click(|ctx, _, _| ctx.submit_command(CMD_FIND_PREV.to(Target::Global))),
                )
                .with_spacer(8.0)
                .with_child(
                    Button::new("Close")
                        .on_click(|_, data: &mut AppState, _| data.search_visible = false),
                ),
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
                .with_child(
                    Checkbox::new("Match case")
                        .lens(AppState::search.then(SearchPanelState::match_case)),
                )
                .with_spacer(12.0)
                .with_child(
                    Checkbox::new("Wrap around")
                        .lens(AppState::search.then(SearchPanelState::wrap)),
                ),
        )
        .with_spacer(8.0)
        .with_child(
            Flex::row()
                .with_child(
                    Button::new("Find Next")
                        .on_click(|ctx, _, _| ctx.submit_command(CMD_FIND_NEXT.to(Target::Global))),
                )
                .with_spacer(8.0)
                .with_child(
                    Button::new("Replace").on_click(|ctx, _, _| {
                        ctx.submit_command(CMD_REPLACE_ONE.to(Target::Global))
                    }),
                )
                .with_spacer(8.0)
                .with_child(
                    Button::new("Replace All").on_click(|ctx, _, _| {
                        ctx.submit_command(CMD_REPLACE_ALL.to(Target::Global))
                    }),
                )
                .with_spacer(8.0)
                .with_child(
                    Button::new("Close")
                        .on_click(|_, data: &mut AppState, _| data.search_visible = false),
                ),
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
                .with_child(
                    Button::new("Go To")
                        .on_click(|ctx, _, _| ctx.submit_command(CMD_GO_TO.to(Target::Global))),
                )
                .with_spacer(8.0)
                .with_child(
                    Button::new("Close")
                        .on_click(|_, data: &mut AppState, _| data.search_visible = false),
                ),
        )
}
