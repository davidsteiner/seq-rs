use crate::diagram::{SequenceDiagram, TimelineEvent};
use crate::rendering::layout::{string_width, GridSize, ReservedWidth};
use crate::rendering::renderer::{RectParams, Renderer, MEDIUM_BLUE};
use nalgebra::Point2;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct Participant {
    pub name: String,
    label: String,
    kind: ParticipantKind,
    pub idx: usize,
}

impl Participant {
    pub fn new(name: String, kind: ParticipantKind) -> Participant {
        let label = name.clone();
        Participant::with_label(name, kind, label)
    }

    pub fn with_label(name: String, kind: ParticipantKind, label: String) -> Participant {
        Participant {
            name,
            label,
            kind,
            idx: 0,
        }
    }

    pub fn get_label(&self) -> &String {
        &self.label
    }

    pub fn get_kind(&self) -> &ParticipantKind {
        &self.kind
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum ParticipantKind {
    Default,
    Actor,
    Database,
}

pub const PARTICIPANT_HEIGHT: u32 = 100;
pub const PARTICIPANT_SPACE: u32 = 150;
pub const ACTOR_HEIGHT: u32 = 160;
const FONT_SIZE: u8 = 35;

pub struct ParticipantCreated {
    pub(crate) participant: Rc<RefCell<Participant>>,
}

impl TimelineEvent for ParticipantCreated {
    fn draw(
        &self,
        _diagram: &SequenceDiagram,
        renderer: &mut dyn Renderer,
        grid: &GridSize,
        row: usize,
    ) {
        let participant = self.participant.borrow();
        let center_x = grid.get_col_center(participant.get_idx());

        // render lifeline
        renderer.render_line(
            Point2::new(center_x, grid.get_row_bottom(row)),
            Point2::new(center_x, grid.get_row_top(grid.num_rows() - 1)),
            3,
            0,
            MEDIUM_BLUE,
            None,
        );

        // render participant at the top
        draw_participant(&participant, renderer, center_x, grid.get_row_top(row));

        // render participant at the bottom
        draw_participant(
            &participant,
            renderer,
            center_x,
            grid.get_row_top(grid.num_rows() - 1),
        );
    }

    fn reserved_width(&self) -> Option<ReservedWidth> {
        let col = self.participant.borrow().get_idx();
        let width = get_participant_width(&self.participant.borrow());
        Some(ReservedWidth::new(col, col, width))
    }

    fn height(&self) -> u32 {
        match self.participant.borrow().get_kind() {
            ParticipantKind::Default => PARTICIPANT_HEIGHT,
            ParticipantKind::Actor => ACTOR_HEIGHT,
            ParticipantKind::Database => ACTOR_HEIGHT,
        }
    }

    fn col_range(&self) -> Option<(usize, usize)> {
        let col = self.participant.borrow().get_idx();
        Some((col, col))
    }
}

pub fn get_participant_width(participant: &Participant) -> u32 {
    get_rendered_width(participant) + PARTICIPANT_SPACE
}

fn get_rendered_width(participant: &Participant) -> u32 {
    string_width(participant.get_label(), FONT_SIZE) + 50
}

pub fn draw_participant(participant: &Participant, renderer: &mut dyn Renderer, x: u32, y: u32) {
    match participant.get_kind() {
        ParticipantKind::Default => draw_default_participant(renderer, participant, x, y),
        ParticipantKind::Actor => draw_actor(renderer, participant, x, y),
        ParticipantKind::Database => draw_database(renderer, participant, x, y),
    }
}

fn draw_default_participant(
    renderer: &mut dyn Renderer,
    participant: &Participant,
    x: u32,
    y: u32,
) {
    let width = get_rendered_width(participant);
    let rect_params = RectParams {
        r: 10,
        ..Default::default()
    };
    renderer.render_rect(x - width / 2, y, width, PARTICIPANT_HEIGHT, rect_params);
    renderer.render_text(
        &participant.get_label(),
        x,
        y + PARTICIPANT_HEIGHT / 3 * 2,
        FONT_SIZE,
        "middle",
    );
}

pub fn draw_actor(renderer: &mut dyn Renderer, participant: &Participant, x: u32, y: u32) {
    renderer.render_stickman(x, y + ACTOR_HEIGHT - 70, 70, ACTOR_HEIGHT - 70);
    renderer.render_text(
        &participant.get_label(),
        x,
        y + ACTOR_HEIGHT - 20,
        FONT_SIZE,
        "middle",
    );
}

fn draw_database(renderer: &mut dyn Renderer, participant: &Participant, x: u32, y: u32) {
    renderer.render_db_icon(x, y + ACTOR_HEIGHT - 70, 70, ACTOR_HEIGHT - 70);
    renderer.render_text(
        &participant.get_label(),
        x,
        y + ACTOR_HEIGHT - 20,
        FONT_SIZE,
        "middle",
    );
}
