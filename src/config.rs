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

static DEFAULT_FONT_SIZE: u32 = 12;

impl Default for Config {
    fn default() -> Self {
        Config {
            group_config: GroupConfig { font_size: DEFAULT_FONT_SIZE },
            message_config: MessageConfig { font_size: DEFAULT_FONT_SIZE },
            note_config: NoteConfig { font_size: DEFAULT_FONT_SIZE },
            participant_config: ParticipantConfig { font_size: 35 },
        }
    }
}
