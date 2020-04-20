use crate::diagram::{Event, LineStyle, Message, Participant, ParticipantKind, SequenceDiagram};
use crate::rendering::layout::{
    calculate_grid, Draw, GridSize, ACTOR_HEIGHT, PARTICIPANT_HEIGHT, PARTICIPANT_WIDTH,
};

use nalgebra::Point2;
use svg::node::element::{Circle, Definitions, Line, Marker, Path, Rectangle, Text};
use svg::node::{Node, Text as TextNode};
use svg::Document;

static ARROW_HEAD_ID: &str = "arrow";

static LIGHT_BLUE: &str = "#add3ff";
static MEDIUM_BLUE: &str = "#62acff";

pub fn render(diagram: &SequenceDiagram) -> String {
    let grid_size = calculate_grid(diagram);
    let width = grid_size.width();
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
                        Point2::new(center_x, grid_size.row_bounds[idx + 1]),
                        Point2::new(center_x, height - participant.height()),
                        3,
                        0,
                        MEDIUM_BLUE,
                        None,
                    );

                    // render participant at the top
                    renderer.render_participant(
                        participant,
                        center_x,
                        grid_size.row_bounds[idx + 1] - participant.height(),
                    );
                    // render participant at the bottom
                    renderer.render_participant(
                        participant,
                        center_x,
                        height - grid_size.row_bounds[1],
                    );
                }
                Event::MessageSent(msg) => {
                    render_message(&mut renderer, msg, idx, diagram, &grid_size);
                }
            }
        }
    }
    renderer.as_string()
}

fn render_message(
    renderer: &mut SVGRenderer,
    msg: &Message,
    row: usize,
    diagram: &SequenceDiagram,
    grid_size: &GridSize,
) {
    if msg.from == msg.to {
        render_self_message(renderer, msg, row, diagram, grid_size);
    } else {
        render_regular_message(renderer, msg, row, diagram, grid_size);
    }
}

fn render_regular_message(
    renderer: &mut SVGRenderer,
    msg: &Message,
    row: usize,
    diagram: &SequenceDiagram,
    grid_size: &GridSize,
) {
    let y = grid_size.get_row_center(row);

    let (src_idx, _) = diagram.find_participant(&msg.from).unwrap();
    let (dest_idx, _) = diagram.find_participant(&msg.to).unwrap();
    let src_x = grid_size.get_col_center(src_idx);
    let dest_x = grid_size.get_col_center(dest_idx);
    let dash = match &msg.style {
        LineStyle::Plain => 0,
        LineStyle::Dashed => 10,
    };

    renderer.render_arrow(Point2::new(src_x, y), Point2::new(dest_x, y), dash);

    let text_bounds = if src_x < dest_x {
        (src_x, dest_x)
    } else {
        (dest_x, src_x)
    };
    let text_x = (text_bounds.1 - text_bounds.0) / 2 + text_bounds.0;
    renderer.render_text(&msg.label, text_x, y - 5, 35, "middle");
}

fn render_self_message(
    renderer: &mut SVGRenderer,
    msg: &Message,
    row: usize,
    diagram: &SequenceDiagram,
    grid_size: &GridSize,
) {
    let y = grid_size.get_row_center(row);
    let y_start = y - 20;
    let y_end = y + 20;
    let (idx, _) = diagram.find_participant(&msg.from).unwrap();
    let x = grid_size.get_col_center(idx);
    let x_offset = x + 35;

    let dash = match &msg.style {
        LineStyle::Plain => 0,
        LineStyle::Dashed => 10,
    };

    renderer.render_line(
        Point2::new(x, y_start),
        Point2::new(x_offset, y_start),
        1,
        dash,
        "black",
        None,
    );
    renderer.render_line(
        Point2::new(x_offset, y_start),
        Point2::new(x_offset, y_end),
        1,
        dash,
        "black",
        None,
    );
    renderer.render_arrow(Point2::new(x_offset, y_end), Point2::new(x, y_end), dash);

    renderer.render_text(&msg.label, x_offset + 10, y + 10, 35, "start");
}

struct SVGRenderer {
    doc: Document,
    participant_width: u32,
}

impl SVGRenderer {
    pub fn new(width: u32, height: u32) -> SVGRenderer {
        let path = Path::new().set("d", "M0,0 L0,12 L18,6 z");
        let marker = Marker::new()
            .set("id", ARROW_HEAD_ID)
            .set("markerWidth", 20)
            .set("markerHeight", 20)
            .set("markerUnits", "userSpaceOnUse")
            .set("refX", 18)
            .set("refY", 6)
            .set("orient", "auto")
            .add(path);

        SVGRenderer {
            doc: Document::new()
                .set("viewBox", (-5, -5, width + 10, height + 10))
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

    fn render_participant(&mut self, participant: &Participant, x: u32, y: u32) {
        match participant.get_kind() {
            ParticipantKind::Default => self.render_default_participant(participant, x, y),
            ParticipantKind::Actor => self.render_actor(participant, x, y),
            ParticipantKind::Database => self.render_database(participant, x, y),
        }
    }

    fn render_default_participant(&mut self, participant: &Participant, x: u32, y: u32) {
        self.render_rect(
            x - self.participant_width / 2,
            y,
            self.participant_width,
            PARTICIPANT_HEIGHT,
            20,
        );
        self.render_text(
            &participant.get_label(),
            x,
            y + PARTICIPANT_HEIGHT / 3 * 2,
            50,
            "middle",
        );
    }

    fn render_database(&mut self, participant: &Participant, x: u32, y: u32) {
        self.render_db_icon(x, y + ACTOR_HEIGHT - 70, 70, ACTOR_HEIGHT - 70);
        self.render_text(
            &participant.get_label(),
            x,
            y + ACTOR_HEIGHT - 20,
            50,
            "middle",
        );
    }

    fn render_db_icon(&mut self, x: u32, y: u32, width: u32, height: u32) {
        let x = x as i32;
        let y = y as i32;
        let width = width as i32;
        let height = height as i32;
        let left_x = x - width / 2;
        let vu = height / 6;
        // let d = format!("M 0 250 c 0 50 200 50 200 0 v -200 c 0 -50 -200 -50 -200 0 v 200 m 0 -200 c 0 50 200 50 200 0",
        let d = format!(
            "M {} {} c {} {} {} {} {} {} v {} c {} {} {} {} {} {} v {} m {} {} c {} {} {} {} {} {}",
            left_x,
            y - vu,
            0,
            vu,
            width,
            vu,
            width,
            0,
            -(4 * vu),
            0,
            -vu,
            -width,
            -vu,
            -width,
            0,
            4 * vu,
            0,
            -4 * vu,
            0,
            vu,
            width,
            vu,
            width,
            0
        );
        let path = Path::new()
            .set("d", d)
            .set("stroke", MEDIUM_BLUE)
            .set("stroke-width", 5)
            .set("fill", LIGHT_BLUE);
        self.add(path);
    }

    fn render_actor(&mut self, participant: &Participant, x: u32, y: u32) {
        self.render_stickman(x, y + ACTOR_HEIGHT - 70, 70, ACTOR_HEIGHT - 70);
        self.render_text(
            &participant.get_label(),
            x,
            y + ACTOR_HEIGHT - 20,
            50,
            "middle",
        );
    }

    fn render_stickman(&mut self, x: u32, y: u32, width: u32, height: u32) {
        let x_offset = width / 2;
        let third_height = height / 3;

        let lines = vec![
            (
                Point2::new(x - x_offset, y),
                Point2::new(x, y - third_height),
            ), // left leg
            (
                Point2::new(x + x_offset, y),
                Point2::new(x, y - third_height),
            ), // right leg
            (
                Point2::new(x, y - third_height),
                Point2::new(x, y - third_height * 2),
            ), // torso
            (
                Point2::new(x - x_offset, y - height / 8 * 5),
                Point2::new(x, y - height / 2),
            ), // left arm
            (
                Point2::new(x + x_offset, y - height / 8 * 5),
                Point2::new(x, y - height / 2),
            ), // right arm
        ];
        for line in lines {
            self.render_line(line.0, line.1, 5, 0, MEDIUM_BLUE, None);
        }
        self.render_circle(
            Point2::new(x, y - height / 6 * 5),
            third_height / 2,
            MEDIUM_BLUE,
        );
    }

    fn render_rect(&mut self, x: u32, y: u32, width: u32, height: u32, r: u32) {
        let rect = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("rx", r)
            .set("ry", r)
            .set("fill", LIGHT_BLUE)
            .set("stroke", MEDIUM_BLUE)
            .set("stroke-width", 5)
            .set("width", width)
            .set("height", height);
        self.add(rect);
    }

    fn render_text(&mut self, text: &str, x: u32, y: u32, font_size: u8, text_anchor: &str) {
        let text = Text::new()
            .set("x", x)
            .set("y", y)
            .set("font-size", font_size)
            .set("text-anchor", text_anchor)
            .add(TextNode::new(text));
        self.add(text);
    }

    fn render_arrow(&mut self, p1: Point2<u32>, p2: Point2<u32>, dash: u8) {
        self.render_line(p1, p2, 1, dash, "black", Some(ARROW_HEAD_ID));
    }

    fn render_line(
        &mut self,
        p1: Point2<u32>,
        p2: Point2<u32>,
        width: u8,
        dash: u8,
        stroke_colour: &str,
        marker_end: Option<&str>,
    ) {
        let mut line = Line::new()
            .set("x1", p1.x)
            .set("y1", p1.y)
            .set("x2", p2.x)
            .set("y2", p2.y)
            .set("stroke", stroke_colour)
            .set("stroke-width", width)
            .set("stroke-dasharray", dash);
        if let Some(m) = marker_end {
            line = line.set("marker-end", format!("url(#{})", m));
        }
        self.add(line);
    }

    fn render_circle(&mut self, center: Point2<u32>, r: u32, stroke_colour: &str) {
        let circle = Circle::new()
            .set("cx", center.x)
            .set("cy", center.y)
            .set("r", r)
            .set("fill", stroke_colour)
            .set("stroke", stroke_colour);
        self.add(circle);
    }
}
