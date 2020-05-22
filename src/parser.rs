use crate::config::Config;
use crate::diagram::SequenceDiagram;
use crate::error::Error;
use crate::group::{AltGroup, Group, SimpleGroup};
use crate::message::Message;
use crate::note::NoteOrientation;
use crate::participant::{Participant, ParticipantKind};
use crate::rendering::renderer::LineStyle;

use crate::parser::AstNode::ParticipantDefinition;
use pest::iterators::Pair;
use pest::Parser;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

#[derive(Parser)]
#[grammar = "seq-rs.pest"]
pub struct PParser;

enum AstNode {
    ParticipantDefinition {
        name: String,
        label: String,
        kind: ParticipantKind,
    },
    Message {
        from: String,
        to: String,
        label: String,
        style: LineStyle,
        activation_modifier: Option<ActivationModifier>,
    },
    GroupStart(String, String),
    AltElse(String),
    GroupEnd,
    Activate(String),
    Deactivate(String),
    MessageNote {
        label: String,
        direction: Direction,
    },
}

enum ActivationModifier {
    Activate,
    Deactivate,
}

enum Direction {
    Left,
    Right,
}

pub fn create_diagram(source: &str, config: Config) -> Result<SequenceDiagram, Error> {
    let mut diagram = SequenceDiagram::new(config);
    let mut active_groups: VecDeque<Rc<RefCell<Group>>> = VecDeque::new();
    let mut last_message: Option<(usize, Message)> = None;
    let ast = parse(source)?;

    for node in ast {
        match node {
            AstNode::ParticipantDefinition { name, label, kind } => {
                let p = Participant::with_label(
                    name,
                    kind,
                    label,
                    diagram.get_config().participant_config,
                );
                diagram.add_participant(p);
            }
            AstNode::Message {
                from,
                to,
                label,
                style,
                activation_modifier,
            } => {
                let row = diagram.get_timeline().len();
                let msg = diagram.add_message(&from, &to, label, style);
                if let Some(modifier) = activation_modifier {
                    match modifier {
                        ActivationModifier::Activate => diagram.activate(&to, Some(row)),
                        ActivationModifier::Deactivate => diagram.deactivate(&from)?,
                    }
                }
                last_message = Some((row, msg));
            }
            AstNode::GroupStart(group_type, header) => {
                let timeline_pos = diagram.get_timeline().len();
                let config = diagram.get_config().group_config;
                let group = match group_type.as_str() {
                    "group" => Group::SimpleGroup(SimpleGroup::new(
                        timeline_pos,
                        header,
                        "".to_string(),
                        config,
                    )),
                    "alt" => Group::AltGroup(AltGroup::new(timeline_pos, header, config)),
                    _ => return Err(Error::new("Unexpected group type".to_string())),
                };
                let rc_group = Rc::new(RefCell::new(group));
                active_groups.push_back(rc_group.clone());
                diagram.start_group(rc_group);
            }
            AstNode::AltElse(label) => match active_groups.back_mut() {
                Some(rc_group) => {
                    match *rc_group.borrow_mut() {
                        Group::AltGroup(ref mut group) => {
                            let row = diagram.get_timeline().len();
                            group.add_case(label, row);
                        }
                        _ => {
                            return Err(Error::new(
                                "else when active group is not an 'alt' group".to_string(),
                            ))
                        }
                    }
                    diagram.add_alt_case(rc_group.clone());
                }
                None => return Err(Error::new("else without active alt group".to_string())),
            },
            AstNode::GroupEnd => match active_groups.pop_back() {
                Some(group) => diagram.end_group(group),
                None => return Err(Error::new("Found end without active group".to_string())),
            },
            AstNode::Activate(participant_name) => {
                diagram.activate(&participant_name, last_message.as_ref().map(|p| p.0));
            }
            AstNode::Deactivate(participant_name) => {
                diagram.deactivate(&participant_name)?;
            }
            AstNode::MessageNote { label, direction } => match last_message.as_ref() {
                Some((_, msg)) => {
                    let orientation = match direction {
                        Direction::Left => {
                            if msg.from < msg.to {
                                NoteOrientation::LeftOf(msg.from.clone())
                            } else {
                                NoteOrientation::LeftOf(msg.to.clone())
                            }
                        }
                        Direction::Right => {
                            if msg.from < msg.to {
                                NoteOrientation::RightOf(msg.to.clone())
                            } else {
                                NoteOrientation::RightOf(msg.from.clone())
                            }
                        }
                    };
                    diagram.add_note(label, orientation, false);
                }
                None => {
                    return Err(Error::new(
                        "Adding note for message before defining any messages".to_string(),
                    ))
                }
            },
        }
    }

    match active_groups.pop_back() {
        None => Ok(diagram),
        Some(_) => Err(Error::new("Group with no closing end keyword".to_string())),
    }
}

fn parse(source: &str) -> Result<Vec<AstNode>, Error> {
    let mut ast = vec![];

    let pairs = PParser::parse(Rule::program, source)?;
    for pair in pairs {
        if let Rule::stmt = pair.as_rule() {
            let inner = pair.into_inner().next().unwrap();
            ast.push(build_ast_from_stmt(inner)?);
        }
    }

    Ok(ast)
}

fn build_ast_from_stmt(pair: Pair<Rule>) -> Result<AstNode, Error> {
    Ok(match pair.as_rule() {
        Rule::participant => parse_participant(pair),
        Rule::message => parse_message(pair)?,
        Rule::group_start => parse_group_start(pair),
        Rule::group_end => AstNode::GroupEnd,
        Rule::alt_else => parse_alt_else(pair),
        Rule::activate => parse_activate(pair),
        Rule::deactivate => parse_deactivate(pair),
        Rule::message_note => parse_message_note(pair)?,
        unknown_expr => panic!("Unexpected expression: {:?}", unknown_expr),
    })
}

fn parse_group_start(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    let group_type = pair.next().unwrap().as_str().to_string();
    let header = match pair.next() {
        Some(h) => h.as_str().to_string(),
        None => "".to_string(),
    };
    AstNode::GroupStart(group_type, header)
}

fn parse_alt_else(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    let label = pair.next().unwrap().as_str().to_string();
    AstNode::AltElse(label)
}

fn parse_activate(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    let label = pair.next().unwrap().as_str().to_string();
    AstNode::Activate(label)
}

fn parse_deactivate(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    let label = pair.next().unwrap().as_str().to_string();
    AstNode::Deactivate(label)
}

fn parse_participant(pair: Pair<Rule>) -> AstNode {
    let mut pair = pair.into_inner();
    let kind = match pair.next().unwrap().as_str() {
        "participant" => ParticipantKind::Default,
        "actor" => ParticipantKind::Actor,
        "database" => ParticipantKind::Database,
        unknown => panic!("Unexpected participant type: {:?}", unknown),
    };
    let label_pair = pair.next().unwrap();
    let label = match label_pair.as_rule() {
        Rule::ident => label_pair.as_str(),
        Rule::string => {
            // Strip the leading and trailing "
            let str = label_pair.as_str();
            &str[1..str.len() - 1]
        }
        unknown_expr => panic!(
            "Unexpected expression in participant label: {:?}",
            unknown_expr
        ),
    };

    let name = match pair.next() {
        Some(inner) => inner.into_inner().next().unwrap().as_str(),
        None => label,
    };

    ParticipantDefinition {
        name: String::from(name),
        label: String::from(label),
        kind,
    }
}

fn parse_message(pair: Pair<Rule>) -> Result<AstNode, Error> {
    let mut pair = pair.into_inner();
    let left_participant = pair.next().unwrap();
    let arrow = pair.next().unwrap();
    let right_participant = pair.next().unwrap();
    let line_style = match arrow.as_str() {
        "<-" | "->" => LineStyle::Plain,
        "<--" | "-->" => LineStyle::Dashed,
        _ => panic!("unexpected arrow type received"),
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

    let mut activation_modifier = None;
    let mut label = "";

    for p in pair {
        match p.as_rule() {
            Rule::activation_modifier => {
                if p.as_str() == "++" {
                    activation_modifier = Some(ActivationModifier::Activate);
                } else {
                    activation_modifier = Some(ActivationModifier::Deactivate);
                }
            }
            Rule::message_label => {
                label = p.into_inner().next().unwrap().as_str();
            }
            _ => {
                return Err(Error::new(
                    "Unexpected rule when parsing message".to_string(),
                ))
            }
        }
    }

    Ok(AstNode::Message {
        from: String::from(from),
        to: String::from(to),
        label: String::from(label),
        style: line_style,
        activation_modifier,
    })
}

fn parse_message_note(pair: Pair<Rule>) -> Result<AstNode, Error> {
    let mut pairs = pair.into_inner();
    let direction = match pairs.next().unwrap().as_str() {
        "left" => Direction::Left,
        "right" => Direction::Right,
        unexpected => {
            return Err(Error::new(format!(
                "Unexpected note orientation: {}",
                unexpected
            )))
        }
    };

    let label = pairs.next().unwrap().into_inner().next().unwrap().as_str();

    Ok(AstNode::MessageNote {
        label: label.to_string(),
        direction,
    })
}
