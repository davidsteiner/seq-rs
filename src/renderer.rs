use crate::diagram::{Participant, SequenceDiagram};

use svg::node::element::Rectangle;
use svg::node::element::Text;
use svg::node::Text as TextNode;
use svg::Document;

const PARTICIPANT_WIDTH: u32 = 300;
const PARTICIPANT_SPACE: u32 = 150;

pub fn render(diagram: &SequenceDiagram) {
    let width = PARTICIPANT_WIDTH * diagram.get_participants().len() as u32
        + PARTICIPANT_SPACE * (diagram.get_participants().len() - 1) as u32;
    let height = 500;
    let mut renderer = SVGRenderer::new(width, height);
    for (idx, participant) in diagram.get_participants().iter().enumerate() {
        let x = (PARTICIPANT_WIDTH + PARTICIPANT_SPACE) * idx as u32;
        renderer.render_participant(participant, x, 0);
    }
    renderer.save();
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

    pub fn render_participant(&mut self, participant: &Participant, x: u32, y: u32) {
        self.render_rect(x, y, self.participant_width, 100);
        self.render_text(
            &participant.get_label(),
            x + self.participant_width / 2,
            y + 50,
        );
    }

    fn render_rect(&mut self, x: u32, y: u32, width: u32, height: u32) {
        let rect = Rectangle::new()
            .set("x", x)
            .set("y", y)
            .set("fill", "grey")
            .set("width", width)
            .set("height", height);
        self.doc = self.doc.clone().add(rect);
    }

    fn render_text(&mut self, text: &str, x: u32, y: u32) {
        let text = Text::new()
            .set("x", x)
            .set("y", y)
            .set("font-size", 50)
            .set("text-anchor", "middle")
            .add(TextNode::new(text));
        self.doc = self.doc.clone().add(text);
    }
}
