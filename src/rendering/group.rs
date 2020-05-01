use crate::diagram::{Event, Group, SequenceDiagram, SimpleGroup};
use crate::rendering::layout::{string_width, GridSize, SizedComponent};
use crate::rendering::renderer::{Renderer, LIGHT_PURPLE, MEDIUM_PURPLE};

use std::collections::HashSet;

impl SizedComponent for Group {
    fn height(&self) -> u32 {
        40
    }

    fn width(&self) -> u32 {
        unimplemented!()
    }
}

pub fn draw_group(
    renderer: &mut dyn Renderer,
    group: &Group,
    diagram: &SequenceDiagram,
    grid_size: &GridSize,
) {
    match group.clone() {
        Group::AltGroup(_) => (),
        Group::SimpleGroup(group) => {
            let text_y = grid_size.row_bounds[group.get_start() + 1] - 10;
            let y = text_y - 25;
            let x_pos = calculate_x_pos(&group, diagram, grid_size);
            let x = x_pos.0 - 10;
            let width = x_pos.1 - x_pos.0 + 20;
            let end_y = grid_size.row_bounds[group.get_end() + 1];
            let height = end_y - y;
            renderer.render_rect(x, y, width, height, LIGHT_PURPLE, 0.2, MEDIUM_PURPLE, 2, 5);

            // Render the label in the top left corner
            let width = string_width(group.get_label(), 20);
            renderer.render_rect(x, y, width, 35, MEDIUM_PURPLE, 1.0, MEDIUM_PURPLE, 2, 5);
            renderer.render_text(group.get_label(), x_pos.0, text_y, 20, "left");
        }
    }
}

fn calculate_x_pos(
    group: &SimpleGroup,
    diagram: &SequenceDiagram,
    grid_size: &GridSize,
) -> (u32, u32) {
    let mut participants = HashSet::new();
    for t in &diagram.get_timeline()[group.get_start()..group.get_end()] {
        for ev in t {
            match ev {
                Event::MessageSent(msg) => {
                    participants.insert(&msg.from);
                    participants.insert(&msg.to);
                }
                Event::ParticipantCreated(p) => {
                    participants.insert(p);
                }
                _ => (),
            }
        }
    }
    let cols: HashSet<usize> = participants
        .iter()
        .map(|&p| diagram.find_participant(p).unwrap().0)
        .collect();

    let min_col = grid_size.cols[cols.iter().min().unwrap() + 1];
    let max_col = grid_size.cols[cols.iter().max().unwrap() + 1];

    (min_col, max_col)
}
