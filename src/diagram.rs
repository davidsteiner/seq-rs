use crate::error::Error;
use crate::group::{AltElse, Group, GroupEnded, GroupStarted};
use crate::message::{Message, MessageSent};
use crate::note::{Note, NoteOrientation};
use crate::participant::{Participant, ParticipantCreated, ParticipantKind};
use crate::rendering::layout::{GridSize, ReservedWidth};
use crate::rendering::renderer::{LineStyle, Renderer};
use std::cell::RefCell;
use std::rc::Rc;

pub trait TimelineEvent {
    fn draw(
        &self,
        diagram: &SequenceDiagram,
        renderer: &mut dyn Renderer,
        grid: &GridSize,
        row: usize,
    );
    fn reserved_width(&self) -> Option<ReservedWidth>;
    fn height(&self) -> u32;
    fn col_range(&self) -> Option<(usize, usize)>;
}

pub struct SequenceDiagram {
    participants: Vec<Rc<RefCell<Participant>>>,
    timeline: Vec<Vec<Box<dyn TimelineEvent>>>,
}

impl SequenceDiagram {
    pub fn new() -> SequenceDiagram {
        SequenceDiagram {
            participants: vec![],
            timeline: vec![vec![]],
        }
    }

    pub fn get_participants(&self) -> &Vec<Rc<RefCell<Participant>>> {
        &self.participants
    }

    pub fn get_timeline(&self) -> &Vec<Vec<Box<dyn TimelineEvent>>> {
        &self.timeline
    }

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
            let p = Participant::new(name.to_string(), ParticipantKind::Default);
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
        let event = Box::new(Note { label, orientation });
        if new_row {
            self.timeline.push(vec![event]);
        } else {
            self.timeline.last_mut().unwrap().push(event);
        }
    }
}
