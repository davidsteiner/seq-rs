use crate::diagram::{LineStyle, Message, Participant, SequenceDiagram};

use pest::error::Error;
use pest::Parser;

#[derive(Parser)]
#[grammar = "planty.pest"]
pub struct PParser;

#[derive(PartialEq, Debug, Clone)]
enum AstNode {
    Participant(Participant),
    Message(Message),
}

pub fn create_diagram(source: &str) -> Result<SequenceDiagram, Error<Rule>> {
    let mut diagram = SequenceDiagram::new();
    let ast = parse(source)?;

    for node in ast {
        match node {
            AstNode::Participant(p) => diagram.add_participant(p),
            AstNode::Message(m) => diagram.add_message(m),
        }
    }

    Ok(diagram)
}

fn parse(source: &str) -> Result<Vec<AstNode>, Error<Rule>> {
    let mut ast = vec![];

    let pairs = PParser::parse(Rule::program, source)?;
    for pair in pairs {
        if let Rule::stmt = pair.as_rule() {
            let inner = pair.into_inner().next().unwrap();
            ast.push(build_ast_from_stmt(inner));
        }
    }

    Ok(ast)
}

fn build_ast_from_stmt(pair: pest::iterators::Pair<Rule>) -> AstNode {
    match pair.as_rule() {
        Rule::participant => {
            let mut pair = pair.into_inner();
            let name = pair.next().unwrap();
            AstNode::Participant(Participant::new(String::from(name.as_str())))
        }
        Rule::message => {
            let mut pair = pair.into_inner();
            let left_participant = pair.next().unwrap();
            let arrow = pair.next().unwrap();
            let right_participant = pair.next().unwrap();
            let line_style = match arrow.as_str() {
                "<-" | "->" => LineStyle::Plain,
                "<--" | "-->" => LineStyle::Dashed,
                _ => panic!("unexpected arrow type received")
            };
            let from;
            let to;
            if arrow.as_str().starts_with('<') {
                from = right_participant.as_str();
                to = left_participant.as_str();
            } else {
                from = left_participant.as_str();
                to = right_participant.as_str();
            };

            AstNode::Message(Message {
                from: String::from(from),
                to: String::from(to),
                style: line_style,
            })
        }
        unknown_expr => panic!("Unexpected expression: {:?}", unknown_expr),
    }
}
