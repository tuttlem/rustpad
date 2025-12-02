use druid::{Selector, WidgetId};

use super::state::{FontChoice, SearchMode};

pub const CMD_NEW_FILE: Selector<()> = Selector::new("rustpad.cmd.new");
pub const CMD_SAVE_FILE: Selector<()> = Selector::new("rustpad.cmd.save");
pub const CMD_SAVE_AS: Selector<()> = Selector::new("rustpad.cmd.save-as");
pub const CMD_EXIT: Selector<()> = Selector::new("rustpad.cmd.exit");
pub const CMD_TIME_DATE: Selector<()> = Selector::new("rustpad.cmd.time-date");
pub const CMD_TOGGLE_WRAP: Selector<()> = Selector::new("rustpad.cmd.wrap");
pub const CMD_TOGGLE_STATUS: Selector<()> = Selector::new("rustpad.cmd.status");
pub const CMD_SHOW_ABOUT: Selector<()> = Selector::new("rustpad.cmd.about");
pub const CMD_SHOW_SEARCH: Selector<SearchMode> = Selector::new("rustpad.cmd.show-search");
pub const CMD_FIND_NEXT: Selector<()> = Selector::new("rustpad.cmd.find-next");
pub const CMD_FIND_PREV: Selector<()> = Selector::new("rustpad.cmd.find-prev");
pub const CMD_REPLACE_ONE: Selector<()> = Selector::new("rustpad.cmd.replace-one");
pub const CMD_REPLACE_ALL: Selector<()> = Selector::new("rustpad.cmd.replace-all");
pub const CMD_GO_TO: Selector<()> = Selector::new("rustpad.cmd.goto");
pub const CMD_SET_FONT: Selector<FontChoice> = Selector::new("rustpad.cmd.font-choice");
pub const CMD_INCREASE_FONT: Selector<()> = Selector::new("rustpad.cmd.font-increase");
pub const CMD_DECREASE_FONT: Selector<()> = Selector::new("rustpad.cmd.font-decrease");

pub const EDITOR_ID: WidgetId = WidgetId::reserved(1);
