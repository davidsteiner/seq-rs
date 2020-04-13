use self::Event::*;

type ID = String;

#[derive(PartialEq, Debug, Clone)]
pub struct Participant {
    name: ID,
    label: String,
}

impl Participant {
    pub fn new(name: String) -> Participant {
        let label = name.clone();
        Participant { name, label }
    }

    pub fn with_label(name: String, label: String) -> Participant {
        Participant { name, label }
    }

    pub fn get_label(&self) -> &String {
        &self.label
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Message {
    pub from: ID,
    pub to: ID,
    pub label: String,
    pub style: LineStyle,
}

#[derive(PartialEq, Debug, Clone)]
pub enum LineStyle {
    Plain,
    Dashed,
}

pub enum Event {
    ParticipantCreated(ID),
    MessageSent(Message),
}

pub struct SequenceDiagram {
    participants: Vec<Participant>,
    timeline: Vec<Vec<Event>>,
}

impl SequenceDiagram {
    pub fn new() -> SequenceDiagram {
        SequenceDiagram {
            participants: vec![],
            timeline: vec![vec![]],
        }
    }

    pub fn get_participants(&self) -> &Vec<Participant> {
        &self.participants
    }

    pub fn get_timeline(&self) -> &Vec<Vec<Event>> {
        &self.timeline
    }

    pub fn find_participant(&self, id: &str) -> Option<(usize, &Participant)> {
        self.participants
            .iter()
            .enumerate()
            .find(|(_idx, p)| p.name.as_str() == id)
    }

    pub fn add_participant(&mut self, participant: Participant) {
        let existing = self.find_participant(&participant.name);
        if existing.is_none() {
            self.timeline[0].push(ParticipantCreated(participant.name.clone()));
            self.participants.push(participant)
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.add_participant(Participant::new(message.from.clone()));
        self.add_participant(Participant::new(message.to.clone()));
        self.timeline.push(vec![MessageSent(message)]);
    }
}
