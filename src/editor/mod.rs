use druid::kurbo::Point;
use druid::text::{FontDescriptor, Selection};
use druid::widget::TextBox;
use druid::widget::prelude::*;
use druid::{Key, KeyOrValue, Lens, Selector, WidgetPod};

pub const APPLY_SELECTION: Selector<ByteRange> = Selector::new("rustpad.editor.apply-selection");
pub const REQUEST_EDITOR_FOCUS: Selector<()> = Selector::new("rustpad.editor.focus");

#[derive(Clone, Copy, Data, Debug, Default, PartialEq)]
pub struct ByteRange {
    pub start: usize,
    pub end: usize,
}

impl ByteRange {}

#[derive(Clone, Data, Debug, Default, Lens)]
pub struct EditorMetrics {
    pub line: usize,
    pub column: usize,
    pub chars: usize,
    pub selection: usize,
}

#[derive(Clone, Data, Debug, Default)]
pub struct SelectionState {
    pub char_range: CharRange,
    pub byte_range: ByteRange,
}

#[derive(Clone, Copy, Data, Debug, Default, PartialEq, Eq)]
pub struct CharRange {
    pub start: usize,
    pub end: usize,
}

impl CharRange {
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
}

pub const EDITOR_FONT_KEY: Key<FontDescriptor> = Key::new("rustpad.editor.font");

pub struct EditorWidget {
    textbox: WidgetPod<String, TextBox<String>>,
}

impl EditorWidget {
    pub fn new(wrap: bool) -> Self {
        let textbox = TextBox::multiline()
            .with_font(KeyOrValue::Key(EDITOR_FONT_KEY))
            .with_line_wrapping(wrap);
        Self {
            textbox: WidgetPod::new(textbox),
        }
    }

    fn update_selection(&mut self, data: &mut crate::app::AppState) {
        let component = self.textbox.widget().text();
        if !component.can_read() {
            return;
        }
        let handle = component.borrow();
        let selection = handle.selection();
        let byte_start = selection.min();
        let byte_end = selection.max();
        let chars_before_start = count_chars(&data.text, byte_start);
        let chars_before_end = count_chars(&data.text, byte_end);
        let char_range = CharRange {
            start: chars_before_start,
            end: chars_before_end,
        };
        data.selection = SelectionState {
            char_range,
            byte_range: ByteRange {
                start: byte_start,
                end: byte_end,
            },
        };
        let column = column_from_bytes(&data.text, byte_start);
        let line = line_from_bytes(&data.text, byte_start);
        data.metrics.line = line;
        data.metrics.column = column;
        data.metrics.chars = data.text.chars().count();
        data.metrics.selection = char_range.len();
    }
}

impl Widget<crate::app::AppState> for EditorWidget {
    fn event(
        &mut self,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut crate::app::AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(APPLY_SELECTION) => {
                if let Some(range) = cmd.get(APPLY_SELECTION) {
                    let selection = Selection::new(range.start, range.end);
                    if let Some(inval) = self
                        .textbox
                        .widget_mut()
                        .text_mut()
                        .borrow_mut()
                        .set_selection(selection)
                    {
                        ctx.invalidate_text_input(inval);
                    }
                    ctx.set_handled();
                }
            }
            Event::Command(cmd) if cmd.is(REQUEST_EDITOR_FOCUS) => {
                ctx.request_focus();
                ctx.set_handled();
            }
            _ => {}
        }

        self.textbox.event(ctx, event, &mut data.text, env);
        self.update_selection(data);
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &crate::app::AppState,
        env: &Env,
    ) {
        self.textbox.lifecycle(ctx, event, &data.text, env);
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &crate::app::AppState,
        data: &crate::app::AppState,
        env: &Env,
    ) {
        if !old_data.text.same(&data.text) {
            self.textbox.update(ctx, &data.text, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &crate::app::AppState,
        env: &Env,
    ) -> Size {
        let size = self.textbox.layout(ctx, bc, &data.text, env);
        self.textbox.set_origin(ctx, Point::ORIGIN);
        size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &crate::app::AppState, env: &Env) {
        self.textbox.paint(ctx, &data.text, env);
    }
}

fn count_chars(text: &str, byte_index: usize) -> usize {
    text[..byte_index.min(text.len())].chars().count()
}

fn line_from_bytes(text: &str, byte_index: usize) -> usize {
    text[..byte_index.min(text.len())]
        .chars()
        .filter(|&ch| ch == '\n')
        .count()
        + 1
}

fn column_from_bytes(text: &str, byte_index: usize) -> usize {
    let slice = &text[..byte_index.min(text.len())];
    slice
        .rsplit_once('\n')
        .map(|(_, tail)| tail.chars().count() + 1)
        .unwrap_or_else(|| slice.chars().count() + 1)
}
