use crate::devboard_controller::{DevboardButtonLeds, DevboardEvents};

pub mod quiz_game;
pub mod reaction_time_game;
pub mod sound_check;

const NUMBER_OF_BUTTONS: usize = 6;

pub trait GameMode: Send {
    /// This is the update function of the game loop. It takes new button input from the devboard
    /// and returns a vector of booleans that represent the LED states of the buttons.
    fn update(&mut self, inputs: DevboardEvents) -> DevboardButtonLeds;

    fn serialize(&self) -> String {
        String::from(
            r#"{
                "game_over": false, "game_mode": "sound_check", "game_state": {"first_buzz": null}

    }"#,
        )
    }
}

pub use quiz_game::QuizGame;
pub use reaction_time_game::ReactionTimeGame;
pub use sound_check::SoundCheck;
