mod group;
mod layout;
mod message;
mod participant;
mod renderer;

use crate::diagram::{Event, SequenceDiagram};
use crate::rendering::group::draw_group;
use crate::rendering::layout::{calculate_grid, GridSize, SizedComponent};
use crate::rendering::message::draw_message;
use crate::rendering::participant::draw_participant;
use crate::rendering::renderer::{Renderer, SVGRenderer, MEDIUM_BLUE};

use nalgebra::Point2;

pub fn render(diagram: &SequenceDiagram, show_debug_lines: bool) -> String {
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
                Event::GroupStarted(group) => {
                    draw_group(&mut renderer, &*group.borrow(), diagram, &grid_size);
                }
                Event::GroupEnded(_) => (),
                Event::AltElse { .. } => (),
            }
        }
    }

    if show_debug_lines {
        render_debug_lines(&mut renderer, &grid_size);
    }

    renderer.as_string()
}

fn render_debug_lines(renderer: &mut dyn Renderer, grid: &GridSize) {
    for col in &grid.cols {
        renderer.render_line(
            Point2::new(*col, 0),
            Point2::new(*col, grid.height()),
            1,
            10,
            "#fd5600",
            None,
        );
    }
    for row in &grid.row_bounds {
        renderer.render_line(
            Point2::new(0, *row),
            Point2::new(grid.width(), *row),
            1,
            10,
            "#fd5600",
            None,
        );
    }
}
