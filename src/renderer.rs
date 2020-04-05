use crate::diagram::{Event, Participant, SequenceDiagram};

use nalgebra::Point2;
use svg::node::element::{Line, Rectangle, Text};
use svg::node::{Node, Text as TextNode};
use svg::Document;

const PARTICIPANT_WIDTH: u32 = 300;
const PARTICIPANT_HEIGHT: u32 = 100;
const PARTICIPANT_SPACE: u32 = 150;

struct GridSize {
    col_bounds: Vec<u32>,
}

impl GridSize {
    fn get_center(&self, col: usize) -> u32 {
        let start = self.col_bounds[col];
        start + (self.col_bounds[col + 1] - start) / 2
    }
}

pub fn render(diagram: &SequenceDiagram) {
    let width = (PARTICIPANT_WIDTH + PARTICIPANT_SPACE) * diagram.get_participants().len() as u32;
    let height = 500;
    let mut renderer = SVGRenderer::new(width, height);
    let grid_size = calculate_grid(diagram);

    for tick in diagram.get_timeline() {
        for event in tick {
            match event {
                Event::ParticipantCreated(participant_id) => {
                    let (col, participant) = diagram.find_participant(participant_id).unwrap();
                    let center_x = grid_size.get_center(col);

                    // render lifeline
                    renderer.render_line(
                        Point2::new(center_x, 0),
                        Point2::new(center_x, height - PARTICIPANT_HEIGHT),
                    );

                    // render participant box
                    renderer.render_participant(participant, center_x, 0);
                    renderer.render_participant(participant, center_x, height - PARTICIPANT_HEIGHT);
                }
                Event::MessageSent(_msg) => {}
            }
        }
    }
    renderer.save();
}

fn calculate_grid(diagram: &SequenceDiagram) -> GridSize {
    let mut col_bounds = vec![0];
    for (idx, _p) in diagram.get_participants().iter().enumerate() {
        // TODO: column widths should be dynamically calculated based on messages and participants
        col_bounds.push((PARTICIPANT_WIDTH + PARTICIPANT_SPACE) * (idx + 1) as u32);
    }
    GridSize { col_bounds }
}

struct SVGRenderer {
    doc: Document,
    participant_width: u32,
}

impl SVGRenderer {
    pub fn new(width: u32, height: u32) -> SVGRenderer {
        SVGRenderer {
            doc: Document::new()
                .set("viewBox", (0, 0, width, height))
                .set("width", width)
                .set("height", height),
            participant_width: PARTICIPANT_WIDTH,
        }
    }

    pub fn save(&self) {
        svg::save("diagram.svg", &self.doc).unwrap();
    }

    fn add<T>(&mut self, node: T)
    where
        T: Node,
    {
        self.doc = self.doc.clone().add(node);
    }

    pub fn render_participant(&mut self, participant: &Participant, x: u32, y: u32) {
        self.render_rect(
            x - self.participant_width / 2,
            y,
            self.participant_width,
            PARTICIPANT_HEIGHT,
        );
        self.render_text(&participant.get_label(), x, y + PARTICIPANT_HEIGHT / 3 * 2);
    }

    fn render_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        let rect = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("fill", "grey")
            .set("width", width)
            .set("height", height);
        self.add(rect);
    }

    fn render_text(&mut self, text: &str, x: u32, y: u32) {
        let text = Text::new()
            .set("x", x)
            .set("y", y)
            .set("font-size", 50)
            .set("text-anchor", "middle")
            .add(TextNode::new(text));
        self.add(text);
    }

    fn render_line(&mut self, p1: Point2<u32>, p2: Point2<u32>) {
        let line = Line::new()
            .set("x1", p1.x)
            .set("y1", p1.y)
            .set("x2", p2.x)
            .set("y2", p2.y)
            .set("stroke", "black")
            .set("stroke-width", 5)
            .set("stroke-dasharray", 15);
        self.add(line);
    }
}
