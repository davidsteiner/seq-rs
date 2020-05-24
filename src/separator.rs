use crate::diagram::{SequenceDiagram, TimelineEvent};
use crate::rendering::layout::{string_width, GridSize, ReservedWidth};
use crate::rendering::renderer::{RectParams, Renderer, LIGHT_PURPLE, MEDIUM_PURPLE};
use nalgebra::Point2;

pub struct Separator {
    label: String,
    config: SeparatorConfig,
}

impl Separator {
    pub(crate) fn new(label: String, config: SeparatorConfig) -> Separator {
        Separator { label, config }
    }

    pub fn width(&self) -> u32 {
        string_width(&self.label, self.config.font_size) * 12 / 10
    }
}

impl TimelineEvent for Separator {
    fn draw(&self, _: &SequenceDiagram, renderer: &mut dyn Renderer, grid: &GridSize, row: usize) {
        let height = grid.get_row_height(row);
        let bottom = grid.get_row_bottom(row);
        let top = grid.get_row_top(row);

        // Draw the two horizontal lines
        for y in &[bottom - height / 3, bottom - height * 2 / 3] {
            renderer.render_line(
                Point2::new(0, *y),
                Point2::new(grid.width(), *y),
                1,
                0,
                MEDIUM_PURPLE,
                None,
            );
        }

        // Draw the box around the label
        let params = RectParams {
            stroke: MEDIUM_PURPLE,
            fill: LIGHT_PURPLE,
            ..Default::default()
        };
        let x = (grid.width() - self.width()) / 2;
        renderer.render_rect(x, top, self.width(), self.height(), params);

        // Draw the label
        renderer.render_text(
            &self.label,
            grid.width() / 2,
            top,
            self.config.font_size,
            "middle",
        );
    }

    fn reserved_width(&self) -> Option<ReservedWidth> {
        Some(ReservedWidth::new(0, usize::max_value(), self.width() + 10))
    }

    fn height(&self) -> u32 {
        self.config.font_size * 12 / 10
    }

    fn col_range(&self) -> Option<(usize, usize)> {
        None
    }
}

#[derive(Clone, Copy)]
pub struct SeparatorConfig {
    pub font_size: u32,
}
