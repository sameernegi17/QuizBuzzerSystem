use crate::devboard_controller::{
    DevboardButtonLed, DevboardButtonLeds, DevboardEventType, DevboardEvents,
};
use rand::Rng;
use std::{collections::HashSet, time::Duration};

const NUMBER_OF_BUTTONS: usize = 6;

pub trait QuizMode {
    /// This is the update function of the game loop. It takes new button input from the devboard
    /// and returns a vector of booleans that represent the LED states of the buttons.
    fn update(&mut self, inputs: DevboardEvents) -> DevboardButtonLeds;
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerButtonState {
    NotPressed,
    Pressed(Duration),
}

/// This is the reaction time game. Several players control a button each. At initialization, a
/// random amount of time is chosen by the game, after which the players will be informed on the
/// frontend that the buttons can now be pressed. The first player to press their button wins.
/// Players that press their button too early will be informed that they have lost.
pub struct ReactionTimeGame {
    devboard_time_since_boot: Option<usize>,
    pub delay: Duration,
    pub player_button_states: [PlayerButtonState; NUMBER_OF_BUTTONS],
    /// List of button indices that were pressed too early
    pub overeager_trigger_happy: HashSet<usize>,
    /// List of button indices that were pressed in time, sorted by reaction time
    pub winners: Vec<(usize, usize)>,
}

impl ReactionTimeGame {
    pub fn new() -> ReactionTimeGame {
        let delay = rand::thread_rng().gen_range(Duration::from_secs(4)..Duration::from_secs(15));
        ReactionTimeGame {
            devboard_time_since_boot: None,
            delay,
            player_button_states: [PlayerButtonState::NotPressed; NUMBER_OF_BUTTONS],
            overeager_trigger_happy: HashSet::new(),
            winners: Vec::new(),
        }
    }
}

impl QuizMode for ReactionTimeGame {
    fn update(&mut self, inputs: DevboardEvents) -> DevboardButtonLeds {
        // In first update after game start, save the time since boot of the devboard
        if self.devboard_time_since_boot.is_none() {
            self.devboard_time_since_boot = Some(inputs.ms_since_reset);
        }

        // Check if any player pressed their button
        for devboard_event in inputs.button_events {
            if devboard_event.event_type == DevboardEventType::Released {
                continue;
            }

            if let PlayerButtonState::Pressed(_) =
                self.player_button_states[devboard_event.button_index]
            {
                continue;
            }

            let time_since_game_start =
                devboard_event.timestamp - self.devboard_time_since_boot.unwrap();
            self.player_button_states[devboard_event.button_index] = PlayerButtonState::Pressed(
                Duration::from_millis(time_since_game_start as u64) - self.delay,
            );
        }

        // Update game state
        self.overeager_trigger_happy = self
            .player_button_states
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                if let PlayerButtonState::Pressed(d) = s {
                    if *d < self.delay {
                        return Some(i);
                    }
                }
                None
            })
            .collect();
        let mut winners = self
            .player_button_states
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                if let PlayerButtonState::Pressed(d) = s {
                    if *d >= self.delay {
                        return Some((i, d.as_millis() as usize));
                    }
                }
                None
            })
            .collect::<Vec<_>>();
        winners.sort_by(|a, b| a.1.cmp(&b.1));
        self.winners = winners;

        // Create response with LED of winning button enabled
        let mut leds = DevboardButtonLeds {
            button_leds: vec![DevboardButtonLed { enabled: false }; NUMBER_OF_BUTTONS],
        };
        if let Some(w) = self.winners.first() {
            leds.button_leds[w.0] = DevboardButtonLed { enabled: true };
        }

        leds
    }
}
