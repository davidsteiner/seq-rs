use crate::diagram::{
    Participant, ParticipantCreated, ParticipantKind, SequenceDiagram, TimelineEvent,
};
use crate::rendering::layout::{string_width, GridSize, ReservedWidth};
use crate::rendering::renderer::{Renderer, LIGHT_BLUE, MEDIUM_BLUE};
use nalgebra::Point2;

pub const PARTICIPANT_HEIGHT: u32 = 100;
pub const PARTICIPANT_SPACE: u32 = 150;
pub const ACTOR_HEIGHT: u32 = 160;
const FONT_SIZE: u8 = 35;

impl TimelineEvent for ParticipantCreated {
    fn draw(
        &self,
        _diagram: &SequenceDiagram,
        renderer: &mut dyn Renderer,
        grid: &GridSize,
        row: usize,
    ) {
        let participant = self.participant.borrow();
        let height = grid.height();
        let center_x = grid.get_col_center(participant.get_idx());
        let y = grid.row_bounds[row + 1] - self.height();

        // render lifeline
        renderer.render_line(
            Point2::new(center_x, grid.row_bounds[row + 1]),
            Point2::new(center_x, height - self.height()),
            3,
            0,
            MEDIUM_BLUE,
            None,
        );

        // render participant at the top
        draw_participant(&participant, renderer, center_x, y);

        // render participant at the bottom
        draw_participant(
            &participant,
            renderer,
            center_x,
            height - grid.row_bounds[1],
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
    renderer.render_rect(
        x - width / 2,
        y,
        width,
        PARTICIPANT_HEIGHT,
        LIGHT_BLUE,
        1.0,
        MEDIUM_BLUE,
        5,
        20,
    );
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
