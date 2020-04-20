mod layout;
mod message;
mod participant;
mod renderer;

use crate::diagram::{Event, SequenceDiagram};
use crate::rendering::layout::{calculate_grid, SizedComponent};
use crate::rendering::message::draw_message;
use crate::rendering::participant::draw_participant;
use crate::rendering::renderer::{Renderer, SVGRenderer, MEDIUM_BLUE};

use nalgebra::Point2;

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
                    draw_participant(
                        participant,
                        &mut renderer,
                        center_x,
                        grid_size.row_bounds[idx + 1] - participant.height(),
                    );
                    // render participant at the bottom
                    draw_participant(
                        participant,
                        &mut renderer,
                        center_x,
                        height - grid_size.row_bounds[1],
                    );
                }
                Event::MessageSent(msg) => {
                    draw_message(&mut renderer, msg, idx, diagram, &grid_size);
                }
            }
        }
    }
    renderer.as_string()
}
