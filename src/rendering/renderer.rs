use nalgebra::Point2;
use svg::node::element::{Circle, Definitions, Line, Marker, Path, Rectangle, Text};
use svg::node::{Node, Text as TextNode};
use svg::Document;

static ARROW_HEAD_ID: &str = "arrow";

pub static LIGHT_BLUE: &str = "#add3ff";
pub static MEDIUM_BLUE: &str = "#62acff";

pub static LIGHT_PURPLE: &str = "#eddbff";
pub static MEDIUM_PURPLE: &str = "#ae8ccf";

pub trait Renderer {
    fn render_rect(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        fill: &str,
        fill_opacity: f32,
        stroke: &str,
        stroke_width: u32,
        r: u32,
    );
    fn render_circle(&mut self, center: Point2<u32>, r: u32, stroke_colour: &str);
    fn render_text(&mut self, text: &str, x: u32, y: u32, font_size: u8, text_anchor: &str);
    fn render_arrow(&mut self, p1: Point2<u32>, p2: Point2<u32>, dash: u8);
    fn render_line(
        &mut self,
        p1: Point2<u32>,
        p2: Point2<u32>,
        width: u8,
        dash: u8,
        stroke_colour: &str,
        marker_end: Option<&str>,
    );
    fn render_db_icon(&mut self, x: u32, y: u32, width: u32, height: u32);
    fn render_stickman(&mut self, x: u32, y: u32, width: u32, height: u32);
}

pub struct SVGRenderer {
    doc: Document,
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
}

impl Renderer for SVGRenderer {
    fn render_rect(
        &mut self,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        fill: &str,
        fill_opacity: f32,
        stroke: &str,
        stroke_width: u32,
        r: u32,
    ) {
        let rect = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("rx", r)
            .set("ry", r)
            .set("fill", fill)
            .set("fill-opacity", fill_opacity)
            .set("stroke", stroke)
            .set("stroke-width", stroke_width)
            .set("width", width)
            .set("height", height);
        self.add(rect);
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

    fn render_text(&mut self, text: &str, x: u32, y: u32, font_size: u8, text_anchor: &str) {
        let text = Text::new()
            .set("x", x)
            .set("y", y)
            .set("font-family", "Courier New")
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
}
