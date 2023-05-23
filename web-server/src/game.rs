use crate::devboard_controller;

pub trait QuizMode {
    /// This is the update function of the game loop. It takes new button input from the devboard
    /// and returns a vector of booleans that represent the LED states of the buttons.
    fn update(
        &mut self,
        inputs: devboard_controller::DevboardEvents,
    ) -> devboard_controller::DevboardButtonLeds;
}

pub struct ReactionTimeGame {}

impl QuizMode for ReactionTimeGame {
    fn update(
        &mut self,
        inputs: devboard_controller::DevboardEvents,
    ) -> devboard_controller::DevboardButtonLeds {
        // for devboard_event in &inputs.button_events {
        //     if devboard_event.event_type == devboard_controller::DevboardEventType::Pressed {
        //         leds.button_leds[devboard_event.button_index as usize].enabled = true;
        //     }
        // }

        let mut leds = devboard_controller::DevboardButtonLeds {
            button_leds: vec![
                devboard_controller::DevboardButtonLed { enabled: true },
                devboard_controller::DevboardButtonLed { enabled: false },
                devboard_controller::DevboardButtonLed { enabled: true },
            ],
        };

        leds
    }
}
