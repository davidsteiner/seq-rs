use crate::diagram::{LineStyle, Message, SequenceDiagram};
use crate::rendering::layout::{GridSize, SizedComponent};
use crate::rendering::renderer::Renderer;
use nalgebra::Point2;

const WIDTH_PER_CHAR: u32 = 25;

impl SizedComponent for Message {
    fn height(&self) -> u32 {
        if self.from != self.to && self.label.is_empty() {
            // Regular messages with no label don't need as much vertical space
            20
        } else {
            50
        }
    }

    fn width(&self) -> u32 {
        self.label.len() as u32 * WIDTH_PER_CHAR
    }
}

pub fn draw_message(
    renderer: &mut dyn Renderer,
    msg: &Message,
    row: usize,
    diagram: &SequenceDiagram,
    grid_size: &GridSize,
) {
    if msg.from == msg.to {
        draw_self_message(renderer, msg, row, diagram, grid_size);
    } else {
        draw_regular_message(renderer, msg, row, diagram, grid_size);
    }
}

fn draw_regular_message(
    renderer: &mut dyn Renderer,
    msg: &Message,
    row: usize,
    diagram: &SequenceDiagram,
    grid_size: &GridSize,
) {
    let y = grid_size.row_bounds[row + 1] - 10;

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

fn draw_self_message(
    renderer: &mut dyn Renderer,
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
