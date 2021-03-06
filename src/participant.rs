use crate::diagram::{SequenceDiagram, TimelineEvent};
use crate::message::ARROW_DISTANCE_FROM_BOTTOM;
use crate::rendering::layout::{string_width, GridSize};
use crate::rendering::renderer::{RectParams, Renderer, MEDIUM_BLUE};
use nalgebra::Point2;
use std::cell::RefCell;
use std::cmp::Ordering;
use std::rc::Rc;

pub const PARTICIPANT_SPACE: u32 = 20;
pub const ACTIVATION_WIDTH: u32 = 10;
pub const ACTIVATION_NESTING_OFFSET: u32 = 3;

#[derive(Debug, Clone)]
pub struct Participant {
    pub name: String,
    label: String,
    kind: ParticipantKind,
    pub idx: usize,
    activations: Vec<Activation>,
    config: ParticipantConfig,
}

impl Participant {
    pub fn new(name: String, kind: ParticipantKind, config: ParticipantConfig) -> Participant {
        let label = name.clone();
        Participant::with_label(name, kind, label, config)
    }

    pub fn with_label(
        name: String,
        kind: ParticipantKind,
        label: String,
        config: ParticipantConfig,
    ) -> Participant {
        Participant {
            name,
            label,
            kind,
            idx: 0,
            activations: vec![],
            config,
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

    pub fn activate(&mut self, start: Option<usize>) {
        let nesting = self.activations.iter().filter(|&a| a.end.is_none()).count();
        self.activations
            .push(Activation::new(start, nesting as u32));
    }

    pub fn deactivate(&mut self, end: usize) -> bool {
        let activation = self.activations.iter_mut().rev().find(|a| a.end.is_none());
        match activation {
            Some(a) => {
                a.end(end);
                true
            }
            None => false,
        }
    }

    fn count_activations_at(&self, row: usize) -> usize {
        self.activations.iter().filter(|&a| a.contains(row)).count()
    }

    pub fn lifeline_offset(&self, row: usize) -> (i32, i32) {
        let count = self.count_activations_at(row);
        if count > 0 {
            (
                -((ACTIVATION_WIDTH / 2) as i32),
                (ACTIVATION_WIDTH / 2) as i32
                    + (count - 1) as i32 * ACTIVATION_NESTING_OFFSET as i32,
            )
        } else {
            (0, 0)
        }
    }
}

impl Ord for Participant {
    fn cmp(&self, other: &Self) -> Ordering {
        self.idx.cmp(&other.idx)
    }
}

impl PartialOrd for Participant {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Participant {
    fn eq(&self, other: &Self) -> bool {
        self.idx == other.idx
    }
}

impl Eq for Participant {}

#[derive(PartialEq, Debug, Clone)]
pub struct Activation {
    start: Option<usize>,
    end: Option<usize>,
    nesting: u32,
}

impl Activation {
    fn new(start: Option<usize>, nesting: u32) -> Activation {
        Activation {
            start,
            end: None,
            nesting,
        }
    }

    fn end(&mut self, end: usize) {
        self.end = Some(end);
    }

    fn contains(&self, row: usize) -> bool {
        let starts_before = match self.start {
            Some(s) => s <= row,
            None => true,
        };
        let ends_after = match self.end {
            Some(s) => s >= row,
            None => true,
        };
        starts_before && ends_after
    }
}

#[derive(Debug, Clone)]
pub enum ParticipantKind {
    Default,
    Actor,
    Database,
}

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

        // render activation boxes
        for activation in &participant.activations {
            let x =
                center_x - ACTIVATION_WIDTH / 2 + activation.nesting * ACTIVATION_NESTING_OFFSET;
            let start_y = match activation.start {
                Some(row) => grid.get_row_bottom(row) - ARROW_DISTANCE_FROM_BOTTOM,
                None => grid.get_row_top(1),
            };
            let end_y = match activation.end {
                Some(row) => grid.get_row_bottom(row) - ARROW_DISTANCE_FROM_BOTTOM,
                None => grid.get_row_bottom(grid.num_rows() - 2),
            };
            let params = RectParams {
                ..Default::default()
            };
            renderer.render_rect(x, start_y, ACTIVATION_WIDTH, end_y - start_y, params);
        }

        // render participant at the top
        draw_participant(
            &participant,
            renderer,
            center_x,
            grid.get_row_bottom(row) - self.height(),
            self.height(),
        );

        // render participant at the bottom
        draw_participant(
            &participant,
            renderer,
            center_x,
            grid.get_row_top(grid.num_rows() - 1),
            self.height(),
        );
    }

    fn height(&self) -> u32 {
        let font_size = self.participant.borrow().config.font_size;
        match self.participant.borrow().get_kind() {
            ParticipantKind::Default => font_size * 2,
            ParticipantKind::Actor => font_size * 4,
            ParticipantKind::Database => font_size * 3,
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
    string_width(participant.get_label(), participant.config.font_size) + 50
}

pub fn draw_participant(
    participant: &Participant,
    renderer: &mut dyn Renderer,
    x: u32,
    y: u32,
    height: u32,
) {
    match participant.get_kind() {
        ParticipantKind::Default => draw_default_participant(renderer, participant, x, y, height),
        ParticipantKind::Actor => draw_actor(renderer, participant, x, y, height),
        ParticipantKind::Database => draw_database(renderer, participant, x, y, height),
    }
}

fn draw_default_participant(
    renderer: &mut dyn Renderer,
    participant: &Participant,
    x: u32,
    y: u32,
    height: u32,
) {
    let width = get_rendered_width(participant);
    let font_size = participant.config.font_size;
    let rect_params = RectParams {
        r: font_size / 4,
        ..Default::default()
    };
    renderer.render_rect(x - width / 2, y, width, height, rect_params);
    renderer.render_text(
        &participant.get_label(),
        x,
        y + (height - font_size) / 2,
        font_size,
        "middle",
    );
}

pub fn draw_actor(
    renderer: &mut dyn Renderer,
    participant: &Participant,
    x: u32,
    y: u32,
    height: u32,
) {
    let stickman_height = height * 2 / 3;
    let stickman_width = stickman_height * 2 / 3;
    renderer.render_stickman(x, y + stickman_height, stickman_width, stickman_height);
    renderer.render_text(
        &participant.get_label(),
        x,
        y + stickman_height,
        participant.config.font_size,
        "middle",
    );
}

fn draw_database(
    renderer: &mut dyn Renderer,
    participant: &Participant,
    x: u32,
    y: u32,
    height: u32,
) {
    let font_size = participant.config.font_size;
    let width = string_width(&participant.get_label(), font_size);

    renderer.render_db_icon(x, y + height, width * 3 / 2, height);
    renderer.render_text(
        &participant.get_label(),
        x,
        y + height - font_size * 11 / 6,
        font_size,
        "middle",
    );
}

#[derive(Clone, Copy, Debug)]
pub struct ParticipantConfig {
    pub font_size: u32,
}
