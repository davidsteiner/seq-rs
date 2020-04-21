use crate::diagram::{Event, SequenceDiagram};

#[derive(Clone, Debug)]
pub struct GridSize {
    pub cols: Vec<u32>,
    pub row_bounds: Vec<u32>,
}

impl GridSize {
    pub fn get_col_center(&self, col: usize) -> u32 {
        self.cols[col + 1]
    }

    pub fn get_row_center(&self, row: usize) -> u32 {
        let start = self.row_bounds[row];
        start + (self.row_bounds[row + 1] - start) / 2
    }

    pub fn width(&self) -> u32 {
        *self.cols.last().unwrap()
    }

    pub fn height(&self) -> u32 {
        *self.row_bounds.last().unwrap()
    }
}

pub fn calculate_grid(diagram: &SequenceDiagram) -> GridSize {
    let mut row_bounds = vec![0];
    for events in diagram.get_timeline() {
        let height = events.iter().map(|ev| get_event_height(ev, diagram)).max();
        row_bounds.push(*row_bounds.last().unwrap() + height.unwrap());
    }
    row_bounds.push(row_bounds.last().unwrap() + row_bounds.get(1).unwrap());

    GridSize {
        cols: calculate_cols(diagram),
        row_bounds,
    }
}

fn get_event_height(event: &Event, diagram: &SequenceDiagram) -> u32 {
    match event {
        Event::MessageSent(msg) => msg.height(),
        Event::ParticipantCreated(p) => diagram.find_participant(p).unwrap().1.height(),
    }
}

pub trait SizedComponent {
    fn height(&self) -> u32;
    fn width(&self) -> u32;
}

fn calculate_cols(diagram: &SequenceDiagram) -> Vec<u32> {
    let mut cols = vec![0];
    let participants = diagram.get_participants();

    let mut y = 0;
    for (idx, p) in participants.iter().enumerate() {
        if idx == 0 {
            y += p.width() / 2;
        } else {
            y += (participants[idx - 1].width() + p.width()) / 2;
        }
        cols.push(y);
        if idx == participants.len() - 1 {
            cols.push(y + p.width() / 2);
        }
    }
    cols
}
