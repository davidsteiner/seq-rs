use crate::diagram::{Group, SequenceDiagram};
use crate::rendering::layout::{GridSize, SizedComponent};
use crate::rendering::renderer::{Renderer, LIGHT_PURPLE, MEDIUM_PURPLE};

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
    _diagram: &SequenceDiagram,
    grid_size: &GridSize,
) {
    match group.clone() {
        Group::AltGroup(_) => (),
        Group::SimpleGroup(group) => {
            let text_y = grid_size.row_bounds[group.get_start() + 1] - 10;
            let y = text_y - 25;
            let x = 10;
            let width = grid_size.width() - 20;
            let end_y = grid_size.row_bounds[group.get_end() + 1];
            let height = end_y - y;
            renderer.render_rect(x, y, width, height, LIGHT_PURPLE, 0.2, MEDIUM_PURPLE, 2, 5);
            renderer.render_rect(x, y, 160, 35, MEDIUM_PURPLE, 1.0, MEDIUM_PURPLE, 2, 5);
            renderer.render_text(group.get_label(), x + 10, text_y, 20, "left");
        }
    }
}
