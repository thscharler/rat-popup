//! Popup acts as a single widget, and takes part of the focus.

use rat_event::{ct_event, HandleEvent, Popup, Regular};
use rat_focus::{
    ContainerFlag, Focus, FocusAdapter, FocusBuilder, FocusFlag, HasFocus, HasFocusFlag, Navigation,
};
use rat_popup::event::PopupOutcome;
use rat_popup::{Placement, PopupCore, PopupCoreState};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Style, Stylize};
use ratatui::text::Span;
use ratatui::widgets::{Block, StatefulWidget, Widget};
use std::cmp::max;

#[derive(Debug)]
pub struct PopFocusBlue;

impl StatefulWidget for PopFocusBlue {
    type State = PopFocusBlueState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if state.popup.is_active() {
            PopupCore::new()
                .placement(state.placement)
                .block(Block::bordered().style(Style::new().black().on_yellow()))
                .render(area, buf, &mut state.popup);

            state.area = state.popup.area;

            buf.set_style(state.popup.widget_area, Style::new().black().on_yellow());

            let mut a1 = state.popup.widget_area;
            a1.height = 1;
            Span::from(" p-o-p-u-p ").render(a1, buf);

            let mut a2 = state.popup.widget_area;
            a2.y += 1;
            a2.height = 1;
            Span::from(state.cc.to_string()).render(a2, buf);
        } else {
            state.popup.clear_areas();
            state.area = Rect::default();
        }
    }
}

#[derive(Debug, Default)]
pub struct PopFocusBlueState {
    pub area: Rect,

    pub cc: char,

    /// Where to place the popup
    pub placement: Placement,
    pub popup: PopupCoreState,

    pub focus: FocusFlag,
}

impl PopFocusBlueState {
    pub fn new() -> Self {
        Self {
            area: Default::default(),
            cc: ' ',
            placement: Default::default(),
            popup: PopupCoreState::named("foc-popup"),
            focus: FocusFlag::named("foc"),
        }
    }

    pub fn is_active(&self) -> bool {
        self.popup.is_active()
    }

    pub fn show(&mut self, placement: Placement, focus: &mut Focus) {
        self.placement = placement;
        focus.focus(&FocusAdapter {
            focus: self.focus.clone(),
            ..Default::default()
        });
    }

    pub fn hide(&mut self, focus: &mut Focus) {
        if self.focus.is_focused() {
            focus.next();
        }
    }
}

impl HasFocus for PopFocusBlueState {
    fn build(&self, builder: &mut FocusBuilder) {
        // build the focus on the fly.
        // don't want to expose HasFocusFlag, as using
        // it would be wrong. This is not a simple widget.
        builder.widget(&FocusAdapter {
            focus: self.focus.clone(),
            area: self.area,
            navigation: Navigation::Leave,
            ..Default::default()
        });
    }

    fn container(&self) -> Option<ContainerFlag> {
        Some(self.popup.active.clone())
    }

    fn area(&self) -> Rect {
        self.popup.active.area()
    }
}

impl HandleEvent<crossterm::event::Event, Regular, PopupOutcome> for PopFocusBlueState {
    fn handle(&mut self, event: &crossterm::event::Event, _qualifier: Regular) -> PopupOutcome {
        let r0 = self.popup.handle(event, Popup);

        let r1 = if self.is_active() {
            match event {
                ct_event!(key press c) => {
                    self.cc = *c;
                    PopupOutcome::Changed
                }
                _ => PopupOutcome::Continue,
            }
        } else {
            PopupOutcome::Continue
        };

        max(r0, r1)
    }
}