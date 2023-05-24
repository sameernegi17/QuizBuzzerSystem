use crate::{
    audio::{AUDIO_PATHS, AUDIO_PATH_FAIL},
    devboard_controller::{
        DevboardButtonLed, DevboardButtonLeds, DevboardEventType, DevboardEvents,
    },
    game,
};
use std::{sync::mpsc, time::Duration};

/// This is the quiz game. Several players control a button each. A quiz question is
/// read out or shown. The first player to press their button will be able to answer. Also their buzzer sound is played and the LED of their button is lit. If the answer is correct, the player wins. If the answer is incorrect, the player loses. If no player answers in time, the question is skipped.
/// Players that press their button too early will be informed that they have lost.
pub struct QuizGame {
    devboard_time_since_boot: Option<usize>,
    /// The id of the first player to press their button
    pub first_buzz: Option<(usize, Duration)>,
    // Audio channel for fun sound effects (if available)
    audio: Option<mpsc::Sender<String>>,
}

impl QuizGame {
    pub fn new(audio: Option<mpsc::Sender<String>>) -> Self {
        QuizGame {
            devboard_time_since_boot: None,
            first_buzz: None,
            audio,
        }
    }
}

impl game::GameMode for QuizGame {
    fn update(&mut self, inputs: DevboardEvents) -> DevboardButtonLeds {
        // In first update after game start, save the time since boot of the devboard
        if self.devboard_time_since_boot.is_none() {
            self.devboard_time_since_boot = Some(inputs.ms_since_reset);
        } else if inputs.ms_since_reset < self.devboard_time_since_boot.unwrap() {
            println!("Devboard reset detected, resetting timer");
            self.devboard_time_since_boot = Some(inputs.ms_since_reset);
        }

        // Check if any player is the first to press their button
        if let Some(devboard_event) = inputs
            .button_events
            .iter()
            .find(|ev| ev.event_type == DevboardEventType::Pressed)
        {
            if self.first_buzz.is_none() {
                // first press, save the player id and the time since game start
                let time_since_game_start =
                    devboard_event.timestamp - self.devboard_time_since_boot.unwrap();

                self.first_buzz = Some((
                    devboard_event.button_index,
                    Duration::from_millis(time_since_game_start as u64),
                ));

                // Play player button audio for first buzz. We assume the button events arrive in increasing order of time.
                #[cfg(feature = "audio")]
                if let Some(audio) = &self.audio {
                    audio
                        .send(AUDIO_PATHS[devboard_event.button_index].to_string())
                        .unwrap();
                    // audio.send(AUDIO_PATH_FAIL.to_string()).unwrap();
                }
            } else {
                // Play fail audio for late buzz.
                #[cfg(feature = "audio")]
                if let Some(audio) = &self.audio {
                    audio.send(AUDIO_PATH_FAIL.to_string()).unwrap();
                }
            }
        }

        // Create response with LED of winning button enabled
        let mut leds = DevboardButtonLeds {
            button_leds: vec![DevboardButtonLed { enabled: false }; game::NUMBER_OF_BUTTONS],
        };
        if let Some((id, _)) = self.first_buzz {
            leds.button_leds[id] = DevboardButtonLed { enabled: true };
        }

        leds
    }
}
