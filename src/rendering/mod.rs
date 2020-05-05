pub mod layout;
pub mod renderer;

use crate::diagram::SequenceDiagram;
use crate::rendering::layout::{calculate_grid, GridSize};
use crate::rendering::renderer::{Renderer, SVGRenderer};

use nalgebra::Point2;

pub fn render(diagram: &SequenceDiagram, show_debug_lines: bool) -> String {
    let grid_size = calculate_grid(diagram);
    let width = grid_size.width();
    let height = grid_size.height();
    let mut renderer = SVGRenderer::new(width, height);

    for (row_idx, row) in diagram.get_timeline().iter().enumerate() {
        for event in row {
            event.draw(diagram, &mut renderer, &grid_size, row_idx);
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

    for row in 0..grid.num_rows() {
        //for y in (grid.get_row_top(row), grid.get_row_bottom(row)) {
        let mut render_line = |y: u32| {
            renderer.render_line(
                Point2::new(0, y),
                Point2::new(grid.width(), y),
                1,
                10,
                "#fd5600",
                None,
            )
        };
        render_line(grid.get_row_top(row));
        render_line(grid.get_row_bottom(row));
    }
}
