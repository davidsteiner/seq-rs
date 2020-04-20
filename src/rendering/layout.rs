use crate::diagram::{Event, Message, Participant, ParticipantKind, SequenceDiagram};

pub const PARTICIPANT_WIDTH: u32 = 300;
pub const PARTICIPANT_HEIGHT: u32 = 100;
pub const PARTICIPANT_SPACE: u32 = 150;
pub const ACTOR_HEIGHT: u32 = 160;

#[derive(Clone, Debug)]
pub struct GridSize {
    pub col_bounds: Vec<u32>,
    pub row_bounds: Vec<u32>,
}

impl GridSize {
    pub fn get_col_center(&self, col: usize) -> u32 {
        let start = self.col_bounds[col];
        start + (self.col_bounds[col + 1] - start) / 2
    }

    pub fn get_row_center(&self, row: usize) -> u32 {
        let start = self.row_bounds[row];
        start + (self.row_bounds[row + 1] - start) / 2
    }

    pub fn width(&self) -> u32 {
        *self.col_bounds.last().unwrap()
    }

    pub fn height(&self) -> u32 {
        *self.row_bounds.last().unwrap()
    }
}

pub fn calculate_grid(diagram: &SequenceDiagram) -> GridSize {
    let mut col_bounds = vec![0];
    for (idx, _) in diagram.get_participants().iter().enumerate() {
        // TODO: column widths should be dynamically calculated based on messages and participants
        col_bounds.push((PARTICIPANT_WIDTH + PARTICIPANT_SPACE) * (idx + 1) as u32);
    }

    let mut row_bounds = vec![0];
    for events in diagram.get_timeline() {
        let height = events.iter().map(|ev| get_event_height(ev, diagram)).max();
        row_bounds.push(*row_bounds.last().unwrap() + height.unwrap());
    }
    row_bounds.push(row_bounds.last().unwrap() + row_bounds.get(1).unwrap());

    GridSize {
        col_bounds,
        row_bounds,
    }
}


fn get_event_height(event: &Event, diagram: &SequenceDiagram) -> u32 {
    match event {
        Event::MessageSent(msg) => msg.height(),
        Event::ParticipantCreated(p) => diagram.find_participant(p).unwrap().1.height(),
    }
}

impl Draw for Participant {
    fn height(&self) -> u32 {
        match self.get_kind() {
            ParticipantKind::Default => PARTICIPANT_HEIGHT,
            ParticipantKind::Actor => ACTOR_HEIGHT,
            ParticipantKind::Database => ACTOR_HEIGHT,
        }
    }
}

impl Draw for Message {
    fn height(&self) -> u32 {
        75
    }
}

pub trait Draw {
    fn height(&self) -> u32;
}
