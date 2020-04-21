use crate::diagram::{Participant, ParticipantKind};
use crate::rendering::layout::SizedComponent;
use crate::rendering::renderer::Renderer;

pub const PARTICIPANT_HEIGHT: u32 = 100;
pub const PARTICIPANT_SPACE: u32 = 150;
pub const ACTOR_HEIGHT: u32 = 160;
const WIDTH_PER_CHAR: u32 = 25;

impl SizedComponent for Participant {
    fn height(&self) -> u32 {
        match self.get_kind() {
            ParticipantKind::Default => PARTICIPANT_HEIGHT,
            ParticipantKind::Actor => ACTOR_HEIGHT,
            ParticipantKind::Database => ACTOR_HEIGHT,
        }
    }

    fn width(&self) -> u32 {
        get_rendered_width(self) + PARTICIPANT_SPACE
    }
}

fn get_rendered_width(participant: &Participant) -> u32 {
    participant.get_label().len() as u32 * WIDTH_PER_CHAR + 50
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
    renderer.render_rect(x - width / 2, y, width, PARTICIPANT_HEIGHT, 20);
    renderer.render_text(
        &participant.get_label(),
        x,
        y + PARTICIPANT_HEIGHT / 3 * 2,
        50,
        "middle",
    );
}

pub fn draw_actor(renderer: &mut dyn Renderer, participant: &Participant, x: u32, y: u32) {
    renderer.render_stickman(x, y + ACTOR_HEIGHT - 70, 70, ACTOR_HEIGHT - 70);
    renderer.render_text(
        &participant.get_label(),
        x,
        y + ACTOR_HEIGHT - 20,
        50,
        "middle",
    );
}

fn draw_database(renderer: &mut dyn Renderer, participant: &Participant, x: u32, y: u32) {
    renderer.render_db_icon(x, y + ACTOR_HEIGHT - 70, 70, ACTOR_HEIGHT - 70);
    renderer.render_text(
        &participant.get_label(),
        x,
        y + ACTOR_HEIGHT - 20,
        50,
        "middle",
    );
}
