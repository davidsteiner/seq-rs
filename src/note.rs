use crate::diagram::{SequenceDiagram, TimelineEvent};
use crate::participant::Participant;
use crate::rendering::layout::{string_width, GridSize, ReservedWidth};
use crate::rendering::renderer::{Renderer, DARK_GREY, LIGHT_GREY};
use std::cell::RefCell;
use std::rc::Rc;

static PARTICIPANT_MARGIN: u32 = 10;

pub struct Note {
    pub orientation: NoteOrientation,
    pub label: String,
    pub config: NoteConfig,
}

pub enum NoteOrientation {
    LeftOf(Rc<RefCell<Participant>>),
    RightOf(Rc<RefCell<Participant>>),
    Over(Vec<Rc<RefCell<Participant>>>),
}

impl Note {
    fn width(&self) -> u32 {
        let longest = &self.label.split("\\n").max_by_key(|t| t.len()).unwrap();
        string_width(longest, self.config.font_size)
    }
}

impl TimelineEvent for Note {
    fn draw(
        &self,
        _diagram: &SequenceDiagram,
        renderer: &mut dyn Renderer,
        grid: &GridSize,
        row: usize,
    ) {
        let x = match &self.orientation {
            NoteOrientation::LeftOf(p) => {
                grid.get_col_center(p.borrow().get_idx()) - self.width() - PARTICIPANT_MARGIN
            }
            NoteOrientation::RightOf(p) => {
                grid.get_col_center(p.borrow().get_idx()) + PARTICIPANT_MARGIN
            }
            NoteOrientation::Over(participants) => {
                let left_participant = participants.iter().min().unwrap();
                let left_idx = left_participant.borrow().idx;
                let right_participant = participants.iter().max().unwrap();
                let right_idx = right_participant.borrow().idx;

                let center = (grid.get_col_center(right_idx) + grid.get_col_center(left_idx)) / 2;
                let unadjusted = (center as i32 - (self.width() / 2) as i32)
                    .max(0)
                    .min((grid.width() - self.width()) as i32)
                    as u32;
                unadjusted + PARTICIPANT_MARGIN / 2
            }
        };
        let y = grid.get_row_top(row);
        let box_x = x - PARTICIPANT_MARGIN / 2;
        renderer.render_note_box(
            box_x,
            y,
            self.width() + PARTICIPANT_MARGIN,
            self.height(),
            LIGHT_GREY,
            DARK_GREY,
        );
        renderer.render_text(&self.label, x, y, self.config.font_size, "left");
    }

    fn reserved_width(&self) -> Option<ReservedWidth> {
        let cols = match &self.orientation {
            NoteOrientation::LeftOf(p) => (0, p.borrow().get_idx() + 1),
            NoteOrientation::RightOf(p) => (p.borrow().get_idx() + 1, usize::max_value()),
            NoteOrientation::Over(_) => (0, usize::max_value()),
        };
        Some(ReservedWidth::new(
            cols.0,
            cols.1,
            self.width() + PARTICIPANT_MARGIN * 2,
        ))
    }

    fn height(&self) -> u32 {
        let font_size = self.config.font_size;
        (font_size as usize * self.label.split("\\n").count()) as u32 * 11 / 10
            + font_size as u32 / 3
    }

    fn col_range(&self) -> Option<(usize, usize)> {
        None
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NoteConfig {
    pub font_size: u32,
}
