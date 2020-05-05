use crate::diagram::{SequenceDiagram, TimelineEvent};
use crate::rendering::layout::{string_width, GridSize, ReservedWidth};
use crate::rendering::renderer::{Renderer, LIGHT_PURPLE, MEDIUM_PURPLE};

use nalgebra::Point2;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct Case {
    pub row: usize,
    pub label: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct SimpleGroup {
    start: usize,
    end: usize,
    label: String,
    header: String,
}

#[derive(PartialEq, Debug, Clone)]
pub struct AltGroup {
    group: SimpleGroup,
    cases: Vec<Case>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Group {
    SimpleGroup(SimpleGroup),
    AltGroup(AltGroup),
}

impl Group {
    pub fn end(&mut self, end: usize) {
        match self {
            Group::SimpleGroup(g) => g.end(end),
            Group::AltGroup(g) => g.end(end),
        }
    }
}

impl SimpleGroup {
    pub fn new(start: usize, label: String, header: String) -> SimpleGroup {
        SimpleGroup {
            start,
            end: start,
            label,
            header,
        }
    }

    pub fn end(&mut self, end: usize) {
        self.end = end;
    }

    pub fn get_start(&self) -> usize {
        self.start
    }

    pub fn get_end(&self) -> usize {
        self.end
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn get_header(&self) -> &str {
        &self.header
    }
}

impl AltGroup {
    pub fn new(start: usize, header: String) -> AltGroup {
        let group = SimpleGroup {
            start,
            end: 0,
            label: "alt".to_string(),
            header,
        };
        AltGroup {
            group,
            cases: vec![],
        }
    }

    pub fn add_case(&mut self, label: String, row: usize) -> usize {
        let idx = self.cases.len();
        self.cases.push(Case { label, row });
        idx
    }

    pub fn end(&mut self, end: usize) {
        self.group.end(end);
    }

    pub fn get_simple_group(&self) -> &SimpleGroup {
        &self.group
    }

    pub fn get_cases(&self) -> &Vec<Case> {
        &self.cases
    }
}

pub struct GroupStarted {
    pub(crate) group: Rc<RefCell<Group>>,
}

pub struct GroupEnded;

pub struct AltElse;

impl TimelineEvent for GroupStarted {
    fn draw(
        &self,
        diagram: &SequenceDiagram,
        renderer: &mut dyn Renderer,
        grid: &GridSize,
        _row: usize,
    ) {
        draw_group(renderer, &self.group.borrow(), diagram, grid);
    }

    fn reserved_width(&self) -> Option<ReservedWidth> {
        None
    }

    fn height(&self) -> u32 {
        40
    }

    fn col_range(&self) -> Option<(usize, usize)> {
        None
    }
}

impl TimelineEvent for GroupEnded {
    fn draw(
        &self,
        _diagram: &SequenceDiagram,
        _renderer: &mut dyn Renderer,
        _grid: &GridSize,
        _row: usize,
    ) {
    }

    fn reserved_width(&self) -> Option<ReservedWidth> {
        None
    }

    fn height(&self) -> u32 {
        5
    }

    fn col_range(&self) -> Option<(usize, usize)> {
        None
    }
}

impl TimelineEvent for AltElse {
    fn draw(
        &self,
        _diagram: &SequenceDiagram,
        _renderer: &mut dyn Renderer,
        _grid: &GridSize,
        _row: usize,
    ) {
    }

    fn reserved_width(&self) -> Option<ReservedWidth> {
        None
    }

    fn height(&self) -> u32 {
        25
    }

    fn col_range(&self) -> Option<(usize, usize)> {
        None
    }
}

pub fn draw_group(
    renderer: &mut dyn Renderer,
    group: &Group,
    diagram: &SequenceDiagram,
    grid_size: &GridSize,
) {
    let simple_group = match group {
        Group::AltGroup(alt_group) => alt_group.get_simple_group(),
        Group::SimpleGroup(ref group) => group,
    };
    let text_y = grid_size.row_bounds[simple_group.get_start() + 1] - 10;
    let y = text_y - 25;
    let x_pos = calculate_x_pos(simple_group, diagram, grid_size);
    let x = x_pos.0 - 10;
    let width = x_pos.1 - x_pos.0 + 20;
    let end_y = grid_size.row_bounds[simple_group.get_end() + 1];
    let height = end_y - y;
    renderer.render_rect(x, y, width, height, LIGHT_PURPLE, 0.2, MEDIUM_PURPLE, 2, 5);

    // Render the label in the top left corner
    let label_width = string_width(simple_group.get_label(), 20) + 20;
    renderer.render_rect(
        x,
        y,
        label_width,
        35,
        MEDIUM_PURPLE,
        1.0,
        MEDIUM_PURPLE,
        2,
        5,
    );
    renderer.render_text(simple_group.get_label(), x_pos.0, text_y, 20, "left");

    // Render header to the right of the label
    let header = simple_group.get_header();
    if !header.is_empty() {
        let header = format!("[{}]", header);
        renderer.render_text(&header, x + label_width + 10, text_y, 20, "left");
    }

    // If this is an alt group, also render the else blocks
    if let Group::AltGroup(alt_group) = group {
        for case in alt_group.get_cases() {
            let text_y = grid_size.row_bounds[case.row + 1];
            let y = text_y - 20;
            renderer.render_line(
                Point2::new(x, y),
                Point2::new(x + width, y),
                2,
                10,
                MEDIUM_PURPLE,
                None,
            );
            renderer.render_text(&format!("[{}]", &case.label), x_pos.0, text_y, 20, "left");
        }
    }
}

fn calculate_x_pos(
    group: &SimpleGroup,
    diagram: &SequenceDiagram,
    grid_size: &GridSize,
) -> (u32, u32) {
    let mut cols = HashSet::new();
    for t in &diagram.get_timeline()[group.get_start()..group.get_end()] {
        for ev in t {
            if let Some((col1, col2)) = ev.col_range() {
                cols.insert(col1);
                cols.insert(col2);
            }
        }
    }

    let min_col = grid_size.cols[cols.iter().min().unwrap() + 1];
    let max_col = grid_size.cols[cols.iter().max().unwrap() + 1];

    (min_col, max_col)
}
