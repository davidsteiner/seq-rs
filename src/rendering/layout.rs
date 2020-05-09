use crate::diagram::SequenceDiagram;
use crate::participant::get_participant_width;

static ROW_MARGIN: u32 = 20;

#[derive(Clone, Debug)]
pub struct GridSize {
    pub cols: Vec<u32>,
    row_bounds: Vec<u32>,
}

impl GridSize {
    fn new() -> GridSize {
        GridSize {
            row_bounds: vec![ROW_MARGIN],
            cols: vec![0],
        }
    }

    pub fn num_rows(&self) -> usize {
        self.row_bounds.len() / 2
    }

    pub fn get_col_center(&self, col: usize) -> u32 {
        self.cols[col + 1]
    }

    pub fn get_row_center(&self, row: usize) -> u32 {
        self.get_row_top(row) + self.get_row_height(row) / 2
    }

    pub fn get_row_height(&self, row: usize) -> u32 {
        self.get_row_bottom(row) - self.get_row_top(row)
    }

    pub fn get_row_bottom(&self, row: usize) -> u32 {
        self.row_bounds[row * 2 + 1]
    }

    pub fn get_row_top(&self, row: usize) -> u32 {
        self.row_bounds[row * 2]
    }

    pub fn width(&self) -> u32 {
        *self.cols.last().unwrap()
    }

    pub fn height(&self) -> u32 {
        *self.row_bounds.last().unwrap()
    }

    fn add_row(&mut self, height: u32) {
        let bottom = self.row_bounds.last().unwrap() + height;
        self.row_bounds.push(bottom);
        self.row_bounds.push(bottom + ROW_MARGIN);
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
    let mut grid = GridSize::new();
    for events in diagram.get_timeline() {
        let height = events.iter().map(|ev| ev.height()).max();
        grid.add_row(height.unwrap());
    }
    grid.add_row(grid.get_row_height(0));

    grid.cols = calculate_cols(diagram);
    grid
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
                    let missing_space = width as i32 - (cols[idx + 1] - cols[idx]) as i32;
                    if missing_space > 0 {
                        for col in &mut cols[idx + 1..] {
                            *col += missing_space as u32;
                        }
                    }
                }
            }
        }
    }

    cols
}
