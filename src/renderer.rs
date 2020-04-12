use crate::diagram::{Event, Participant, SequenceDiagram, LineStyle};

use nalgebra::Point2;
use svg::node::element::{Definitions, Line, Marker, Path, Rectangle, Text};
use svg::node::{Node, Text as TextNode};
use svg::Document;

const PARTICIPANT_WIDTH: u32 = 300;
const PARTICIPANT_HEIGHT: u32 = 100;
const PARTICIPANT_SPACE: u32 = 150;

static ARROW_HEAD_ID: &str = "arrow";

struct GridSize {
    col_bounds: Vec<u32>,
    row_bounds: Vec<u32>,
}

impl GridSize {
    fn get_col_center(&self, col: usize) -> u32 {
        let start = self.col_bounds[col];
        start + (self.col_bounds[col + 1] - start) / 2
    }

    fn get_row_center(&self, row: usize) -> u32 {
        let start = self.row_bounds[row];
        start + (self.row_bounds[row + 1] - start) / 2
    }

    fn height(&self) -> u32 {
        *self.row_bounds.last().unwrap()
    }
}

pub fn render(diagram: &SequenceDiagram) -> String {
    let width = (PARTICIPANT_WIDTH + PARTICIPANT_SPACE) * diagram.get_participants().len() as u32;
    let grid_size = calculate_grid(diagram);
    let height = grid_size.height();
    let mut renderer = SVGRenderer::new(width, height);

    for (idx, row) in diagram.get_timeline().iter().enumerate() {
        for event in row {
            match event {
                Event::ParticipantCreated(participant_id) => {
                    let (col, participant) = diagram.find_participant(participant_id).unwrap();
                    let center_x = grid_size.get_col_center(col);

                    // render lifeline
                    renderer.render_line(
                        Point2::new(center_x, grid_size.row_bounds[idx]),
                        Point2::new(center_x, height - PARTICIPANT_HEIGHT),
                        15,
                        None,
                    );

                    // render participant box
                    renderer.render_participant(participant, center_x, 0);
                    renderer.render_participant(participant, center_x, height - PARTICIPANT_HEIGHT);
                }
                Event::MessageSent(msg) => {
                    let y = grid_size.get_row_center(idx);
                    let (src_idx, _) = diagram.find_participant(&msg.from).unwrap();
                    let (dest_idx, _) = diagram.find_participant(&msg.to).unwrap();
                    let src_x = grid_size.get_col_center(src_idx);
                    let dest_x = grid_size.get_col_center(dest_idx);
                    let dash = match &msg.style {
                        LineStyle::Plain => 0,
                        LineStyle::Dashed => 10,
                    };

                    renderer.render_arrow(Point2::new(src_x, y), Point2::new(dest_x, y), dash);

                    let text_bounds = if src_x < dest_x { (src_x, dest_x) } else { (dest_x, src_x) };
                    let text_x = (text_bounds.1 - text_bounds.0) / 2 + text_bounds.0;
                    renderer.render_text(&msg.label, text_x, y - 5, 25);
                }
            }
        }
    }
    renderer.as_string()
}

fn calculate_grid(diagram: &SequenceDiagram) -> GridSize {
    let mut col_bounds = vec![0];
    for (idx, _) in diagram.get_participants().iter().enumerate() {
        // TODO: column widths should be dynamically calculated based on messages and participants
        col_bounds.push((PARTICIPANT_WIDTH + PARTICIPANT_SPACE) * (idx + 1) as u32);
    }

    let mut row_bounds = vec![0];
    for (idx, _) in diagram.get_timeline().iter().enumerate() {
        row_bounds.push(PARTICIPANT_HEIGHT * (idx + 1) as u32);
    }
    row_bounds.push(row_bounds.last().unwrap() + PARTICIPANT_HEIGHT);

    GridSize {
        col_bounds,
        row_bounds,
    }
}

struct SVGRenderer {
    doc: Document,
    participant_width: u32,
}

impl SVGRenderer {
    pub fn new(width: u32, height: u32) -> SVGRenderer {
        let path = Path::new().set("d", "M0,0 L0,6 L9,3 z");
        let marker = Marker::new()
            .set("id", ARROW_HEAD_ID)
            .set("markerWidth", 10)
            .set("markerHeight", 10)
            .set("refX", 9)
            .set("refY", 3)
            .set("orient", "auto")
            .add(path);

        SVGRenderer {
            doc: Document::new()
                .set("viewBox", (0, 0, width, height))
                .add(Definitions::new().add(marker)),
            participant_width: PARTICIPANT_WIDTH,
        }
    }

    pub fn as_string(&self) -> String {
        self.doc.to_string()
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
        self.render_text(&participant.get_label(), x, y + PARTICIPANT_HEIGHT / 3 * 2, 50);
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

    fn render_text(&mut self, text: &str, x: u32, y: u32, font_size: u8) {
        let text = Text::new()
            .set("x", x)
            .set("y", y)
            .set("font-size", font_size)
            .set("text-anchor", "middle")
            .add(TextNode::new(text));
        self.add(text);
    }

    fn render_arrow(&mut self, p1: Point2<u32>, p2: Point2<u32>, dash: u8) {
        self.render_line(p1, p2, dash, Some(ARROW_HEAD_ID));
    }

    fn render_line(
        &mut self,
        p1: Point2<u32>,
        p2: Point2<u32>,
        dash: u8,
        marker_end: Option<&str>,
    ) {
        let mut line = Line::new()
            .set("x1", p1.x)
            .set("y1", p1.y)
            .set("x2", p2.x)
            .set("y2", p2.y)
            .set("stroke", "black")
            .set("stroke-width", 5)
            .set("stroke-dasharray", dash);
        if let Some(m) = marker_end {
            line = line.set("marker-end", format!("url(#{})", m));
        }
        self.add(line);
    }
}
