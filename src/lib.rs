extern crate pest;
#[macro_use]
extern crate pest_derive;

mod config;
mod diagram;
mod error;
mod group;
mod message;
mod note;
mod parser;
mod participant;
mod rendering;

/// Parses the supplied diagram string into SVG string.
///
/// # Arguments
///
/// * `content` - A string representing the diagram in the diagram DSL
/// * `show_debug_lines` - A boolean to enable debug lines for the layout in the SVG
pub fn parse(content: &str, show_debug_lines: bool) -> Result<String, error::Error> {
    let config = config::Config {
        ..Default::default()
    };
    let diagram = parser::create_diagram(content, config)?;
    Ok(rendering::render(&diagram, show_debug_lines))
}
