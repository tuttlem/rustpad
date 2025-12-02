use druid::commands::{OPEN_FILE, SAVE_FILE_AS};
use druid::widget::Controller;
use druid::{Env, Event, EventCtx, FileInfo, Widget, commands};

use crate::app::commands::{
    CMD_DECREASE_FONT, CMD_EXIT, CMD_FIND_NEXT, CMD_FIND_PREV, CMD_GO_TO, CMD_INCREASE_FONT,
    CMD_NEW_FILE, CMD_REPLACE_ALL, CMD_REPLACE_ONE, CMD_SAVE_AS, CMD_SAVE_FILE, CMD_SET_FONT,
    CMD_SHOW_ABOUT, CMD_SHOW_SEARCH, CMD_TIME_DATE, CMD_TOGGLE_STATUS, CMD_TOGGLE_WRAP,
};
use crate::app::state::AppState;

mod edit_ops;
mod file_ops;
mod search_ops;
mod text_utils;

pub struct AppController;

impl<W: Widget<AppState>> Controller<AppState, W> for AppController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(CMD_NEW_FILE) => {
                file_ops::new_file(data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_SAVE_FILE) => {
                file_ops::save(ctx, data, false);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_SAVE_AS) => {
                file_ops::save(ctx, data, true);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_EXIT) => {
                ctx.submit_command(commands::QUIT_APP);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_TIME_DATE) => {
                edit_ops::insert_timestamp(ctx, data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_TOGGLE_WRAP) => {
                edit_ops::toggle_wrap(data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_TOGGLE_STATUS) => {
                edit_ops::toggle_status_bar(data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_SHOW_ABOUT) => {
                edit_ops::show_about(data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_SHOW_SEARCH) => {
                let mode = cmd.get(CMD_SHOW_SEARCH).copied();
                search_ops::show_search(data, mode);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_SET_FONT) => {
                if let Some(choice) = cmd.get(CMD_SET_FONT) {
                    if edit_ops::set_font_choice(data, *choice) {
                        ctx.request_layout();
                        ctx.request_paint();
                    }
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_INCREASE_FONT) => {
                if edit_ops::increase_font_size(data) {
                    ctx.request_layout();
                    ctx.request_paint();
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_DECREASE_FONT) => {
                if edit_ops::decrease_font_size(data) {
                    ctx.request_layout();
                    ctx.request_paint();
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_FIND_NEXT) => {
                search_ops::run_search(ctx, data, true);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_FIND_PREV) => {
                search_ops::run_search(ctx, data, false);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_REPLACE_ONE) => {
                search_ops::replace_once(ctx, data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_REPLACE_ALL) => {
                search_ops::replace_all(data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(CMD_GO_TO) => {
                search_ops::goto_line(ctx, data);
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(OPEN_FILE) => {
                if let Some(info) = cmd.get::<FileInfo>(OPEN_FILE) {
                    file_ops::handle_open_selection(data, info);
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(SAVE_FILE_AS) => {
                if let Some(info) = cmd.get::<FileInfo>(SAVE_FILE_AS) {
                    file_ops::handle_save_selection(data, info);
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
