use crate::config::Config;
use crate::error::Error;
use crate::group::{AltElse, Group, GroupEnded, GroupStarted};
use crate::message::{Message, MessageSent};
use crate::note::{Note, NoteOrientation};
use crate::participant::{Participant, ParticipantCreated, ParticipantKind};
use crate::rendering::layout::{GridSize, ReservedWidth};
use crate::rendering::renderer::{LineStyle, Renderer};
use std::cell::RefCell;
use std::rc::Rc;

/// Trait for events that the diagram's timeline consists of.
/// Timeline events know how to draw themselves on a renderer and how much space needs
/// to be reserved for them (both vertically and horizontally) in the diagram's layout grid.
pub trait TimelineEvent {
    /// Draws the event on the renderer
    fn draw(
        &self,
        diagram: &SequenceDiagram,
        renderer: &mut dyn Renderer,
        grid: &GridSize,
        row: usize,
    );

    /// Returns the width this event requires to be drawn correctly.
    /// This is used in the layout code to calculate the grid size.
    /// If the event does not have an opinion on the layout, it returns None.
    fn reserved_width(&self) -> Option<ReservedWidth> {
        None
    }

    /// The height of the event in the diagram, used to determine the row heights.
    fn height(&self) -> u32;

    /// The column indices the event relates to. It's used in groups to determine which
    /// columns the group needs to wrap.
    fn col_range(&self) -> Option<(usize, usize)>;
}

pub struct SequenceDiagram {
    participants: Vec<Rc<RefCell<Participant>>>,
    timeline: Vec<Vec<Box<dyn TimelineEvent>>>,
    config: Config,
}

impl SequenceDiagram {
    pub fn new(config: Config) -> SequenceDiagram {
        SequenceDiagram {
            participants: vec![],
            timeline: vec![vec![]],
            config,
        }
    }

    pub fn get_config(&self) -> &Config {
        &self.config
    }

    /// Returns the list of all participants in the sequence diagram.
    pub fn get_participants(&self) -> &Vec<Rc<RefCell<Participant>>> {
        &self.participants
    }

    pub fn get_timeline(&self) -> &Vec<Vec<Box<dyn TimelineEvent>>> {
        &self.timeline
    }

    /// Returns the participant for the supplied participant ID or returns None if
    /// there isn't a participant with the ID.
    pub fn find_participant_by_name(&self, id: &str) -> Option<Rc<RefCell<Participant>>> {
        self.participants
            .iter()
            .find(|&p| p.borrow().name.as_str() == id)
            .cloned()
    }

    pub fn add_participant(&mut self, mut participant: Participant) -> Rc<RefCell<Participant>> {
        participant.idx = self.participants.len();
        let rc_participant = Rc::new(RefCell::new(participant));
        self.timeline[0].push(Box::new(ParticipantCreated {
            participant: rc_participant.clone(),
        }));
        self.participants.push(rc_participant.clone());
        rc_participant
    }

    fn get_or_create_participant(&mut self, name: &str) -> Rc<RefCell<Participant>> {
        self.find_participant_by_name(&name).unwrap_or_else(|| {
            let p = Participant::new(
                name.to_string(),
                ParticipantKind::Default,
                self.config.participant_config,
            );
            self.add_participant(p)
        })
    }

    pub fn add_message(
        &mut self,
        from: &str,
        to: &str,
        label: String,
        style: LineStyle,
    ) -> Message {
        let from_participant = self.get_or_create_participant(&from);
        let to_participant = self.get_or_create_participant(&to);

        let message = Message {
            from: from_participant,
            to: to_participant,
            label,
            style,
            config: self.config.message_config,
        };
        self.timeline.push(vec![Box::new(MessageSent {
            message: message.clone(),
        })]);
        message
    }

    pub fn activate(&mut self, participant_name: &str, start: Option<usize>) {
        let participant = self.get_or_create_participant(&participant_name);
        participant.borrow_mut().activate(start);
    }

    pub fn deactivate(&mut self, participant_name: &str) -> Result<(), Error> {
        match self.find_participant_by_name(&participant_name) {
            Some(participant) => {
                if !participant
                    .borrow_mut()
                    .deactivate(self.get_timeline().len() - 1)
                {
                    return Err(Error::new(format!(
                        "Attempting to deactivate participant with no activation: {}",
                        participant_name
                    )));
                }
            }
            None => {
                return Err(Error::new(format!(
                    "Missing participant for deactivate: {}",
                    participant_name
                )))
            }
        };
        Ok(())
    }

    pub fn start_group(&mut self, group: Rc<RefCell<Group>>) {
        self.timeline.push(vec![Box::new(GroupStarted { group })]);
    }

    pub fn end_group(&mut self, group: Rc<RefCell<Group>>) {
        group.borrow_mut().end(self.timeline.len());
        self.timeline.push(vec![Box::new(GroupEnded)]);
    }

    pub fn add_alt_case(&mut self) {
        self.timeline.push(vec![Box::new(AltElse)]);
    }

    pub fn add_note(&mut self, label: String, orientation: NoteOrientation, new_row: bool) {
        let event = Box::new(Note {
            label,
            orientation,
            config: self.config.note_config,
        });
        if new_row {
            self.timeline.push(vec![event]);
        } else {
            self.timeline.last_mut().unwrap().push(event);
        }
    }
}
