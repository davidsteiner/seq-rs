use crate::diagram::SequenceDiagram;
use crate::rendering::participant::get_participant_width;

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

pub struct ReservedWidth {
    left_col: usize,
    right_col: usize,
    width: u32,
}

impl ReservedWidth {
    pub fn new(col1: usize, col2: usize, width: u32) -> ReservedWidth {
        if col1 < col2 {
            ReservedWidth {
                left_col: col1,
                right_col: col2,
                width,
            }
        } else {
            ReservedWidth {
                left_col: col2,
                right_col: col1,
                width,
            }
        }
    }

    pub fn col_distance(&self) -> usize {
        self.right_col - self.left_col
    }
}

pub fn calculate_grid(diagram: &SequenceDiagram) -> GridSize {
    let mut row_bounds = vec![0];
    for events in diagram.get_timeline() {
        let height = events.iter().map(|ev| ev.height()).max();
        row_bounds.push(*row_bounds.last().unwrap() + height.unwrap());
    }
    row_bounds.push(row_bounds.last().unwrap() + row_bounds.get(1).unwrap());

    GridSize {
        cols: calculate_cols(diagram),
        row_bounds,
    }
}

pub fn string_width(s: &str, font_size: u8) -> u32 {
    s.len() as u32 * font_size as u32 * 9 / 14
}

fn calculate_cols(diagram: &SequenceDiagram) -> Vec<u32> {
    let mut cols = vec![0];
    let participants = diagram.get_participants();

    let mut y = 0;
    for (idx, p) in participants.iter().enumerate() {
        let participant = p.borrow();
        if idx == 0 {
            y += get_participant_width(&participant) / 2;
        } else {
            y += (get_participant_width(&participants[idx - 1].borrow())
                + get_participant_width(&participant))
                / 2;
        }
        cols.push(y);
        if idx == participants.len() - 1 {
            cols.push(y + get_participant_width(&participant) / 2);
        }
    }

    for row in diagram.get_timeline() {
        for event in row {
            if let Some(reserved_width) = event.reserved_width() {
                if reserved_width.col_distance() == 1 {
                    let width = reserved_width.width;
                    let idx = reserved_width.left_col;
                    let missing_space = width as i32 - (cols[idx + 2] - cols[idx + 1]) as i32;
                    if missing_space > 0 {
                        for col in &mut cols[idx + 2..] {
                            *col += missing_space as u32;
                        }
                    }
                }
            }
        }
    }

    cols
}
