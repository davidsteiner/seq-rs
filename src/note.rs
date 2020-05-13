use crate::diagram::{SequenceDiagram, TimelineEvent};
use crate::participant::Participant;
use crate::rendering::layout::{string_width, GridSize, ReservedWidth};
use crate::rendering::renderer::{Renderer, DARK_GREY, LIGHT_GREY};
use std::cell::RefCell;
use std::rc::Rc;

static FONT_SIZE: u8 = 24;
static PARTICIPANT_MARGIN: u32 = 10;

pub struct Note {
    pub orientation: NoteOrientation,
    pub label: String,
}

pub enum NoteOrientation {
    LeftOf(Rc<RefCell<Participant>>),
    RightOf(Rc<RefCell<Participant>>),
}

impl Note {
    fn width(&self) -> u32 {
        let longest = &self.label.split("\\n").max_by_key(|t| t.len()).unwrap();
        string_width(longest, FONT_SIZE)
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
        renderer.render_text(&self.label, x, y, FONT_SIZE, "left");
    }

    fn reserved_width(&self) -> Option<ReservedWidth> {
        let cols = match &self.orientation {
            NoteOrientation::LeftOf(p) => (p.borrow().get_idx(), p.borrow().get_idx() + 1),
            NoteOrientation::RightOf(p) => (p.borrow().get_idx() + 1, p.borrow().get_idx() + 2),
        };
        Some(ReservedWidth::new(
            cols.0,
            cols.1,
            self.width() + PARTICIPANT_MARGIN * 2,
        ))
    }

    fn height(&self) -> u32 {
        (FONT_SIZE as usize * self.label.split("\\n").count()) as u32 * 11 / 10
            + FONT_SIZE as u32 / 3
    }

    fn col_range(&self) -> Option<(usize, usize)> {
        None
    }
}
