use crate::diagram::{SequenceDiagram, TimelineEvent};
use crate::participant::Participant;
use crate::rendering::layout::{string_width, GridSize, ReservedWidth};
use crate::rendering::renderer::{LineStyle, Renderer};
use nalgebra::Point2;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Debug, Clone)]
pub struct Message {
    pub from: Rc<RefCell<Participant>>,
    pub to: Rc<RefCell<Participant>>,
    pub label: String,
    pub style: LineStyle,
}

pub struct MessageSent {
    pub(crate) message: Message,
}

const MESSAGE_FONT_SIZE: u8 = 24;
pub const ARROW_DISTANCE_FROM_BOTTOM: u32 = 10;

impl TimelineEvent for MessageSent {
    fn draw(
        &self,
        _diagram: &SequenceDiagram,
        renderer: &mut dyn Renderer,
        grid: &GridSize,
        row: usize,
    ) {
        draw_message(renderer, &self.message, row, grid);
    }

    fn reserved_width(&self) -> Option<ReservedWidth> {
        let from_idx = self.message.from.borrow().get_idx();
        let mut to_idx = self.message.to.borrow().get_idx();
        if from_idx == to_idx {
            to_idx = to_idx + 1;
        }

        // The width of the message depends on its label plus some constant margin
        let width = string_width(&self.message.label, MESSAGE_FONT_SIZE) + 40;
        Some(ReservedWidth::new(from_idx + 1, to_idx + 1, width))
    }

    fn height(&self) -> u32 {
        if self.message.from != self.message.to {
            if self.message.label.is_empty() {
                // Regular messages with no label don't need as much vertical space
                20
            } else {
                40
            }
        } else {
            55
        }
    }

    fn col_range(&self) -> Option<(usize, usize)> {
        let from_idx = self.message.from.borrow().get_idx();
        let to_idx = self.message.to.borrow().get_idx();
        Some(if from_idx < to_idx {
            (from_idx, to_idx)
        } else {
            (to_idx, from_idx)
        })
    }
}

pub fn draw_message(renderer: &mut dyn Renderer, msg: &Message, row: usize, grid_size: &GridSize) {
    if msg.from == msg.to {
        draw_self_message(renderer, msg, row, grid_size);
    } else {
        draw_regular_message(renderer, msg, row, grid_size);
    }
}

fn draw_regular_message(
    renderer: &mut dyn Renderer,
    msg: &Message,
    row: usize,
    grid_size: &GridSize,
) {
    let y = grid_size.get_row_bottom(row) - ARROW_DISTANCE_FROM_BOTTOM;

    let src_idx = msg.from.borrow().get_idx();
    let dest_idx = msg.to.borrow().get_idx();
    let (src_offset, dest_offset) = if src_idx < dest_idx {
        (
            msg.from.borrow().lifeline_offset(row).1,
            msg.to.borrow().lifeline_offset(row).0,
        )
    } else {
        (
            msg.from.borrow().lifeline_offset(row).0,
            msg.to.borrow().lifeline_offset(row).1,
        )
    };
    let src_x = (grid_size.get_col_center(src_idx) as i32 + src_offset) as u32;
    let dest_x = (grid_size.get_col_center(dest_idx) as i32 + dest_offset) as u32;
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
    renderer.render_text(
        &msg.label,
        text_x,
        y - MESSAGE_FONT_SIZE as u32 - 5,
        MESSAGE_FONT_SIZE,
        "middle",
    );
}

fn draw_self_message(renderer: &mut dyn Renderer, msg: &Message, row: usize, grid_size: &GridSize) {
    let y = grid_size.get_row_center(row);
    let y_start = y - 20;
    let y_end = grid_size.get_row_bottom(row) - ARROW_DISTANCE_FROM_BOTTOM;
    let idx = msg.from.borrow().get_idx();
    let x = grid_size.get_col_center(idx) + msg.from.borrow().lifeline_offset(row).1 as u32;
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

    renderer.render_text(
        &msg.label,
        x_offset + 10,
        y_start,
        MESSAGE_FONT_SIZE,
        "start",
    );
}
