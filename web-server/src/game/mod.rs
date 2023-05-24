use crate::devboard_controller::{DevboardButtonLeds, DevboardEvents};

pub mod quiz_game;
pub mod reaction_time_game;

const NUMBER_OF_BUTTONS: usize = 6;

pub trait GameMode: Send {
    /// This is the update function of the game loop. It takes new button input from the devboard
    /// and returns a vector of booleans that represent the LED states of the buttons.
    fn update(&mut self, inputs: DevboardEvents) -> DevboardButtonLeds;
}

pub use quiz_game::QuizGame;
pub use reaction_time_game::ReactionTimeGame;
