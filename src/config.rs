use crate::group::GroupConfig;
use crate::message::MessageConfig;
use crate::note::NoteConfig;
use crate::participant::ParticipantConfig;

#[derive(Clone, Copy)]
pub struct Config {
    pub group_config: GroupConfig,
    pub message_config: MessageConfig,
    pub note_config: NoteConfig,
    pub participant_config: ParticipantConfig,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            group_config: GroupConfig { font_size: 20 },
            message_config: MessageConfig { font_size: 24 },
            note_config: NoteConfig { font_size: 24 },
            participant_config: ParticipantConfig { font_size: 35 },
        }
    }
}
