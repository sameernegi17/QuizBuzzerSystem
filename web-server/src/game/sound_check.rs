use crate::{
    audio::AUDIO_PATHS,
    devboard_controller::{DevboardButtonLed, DevboardButtonLeds, DevboardEvents},
    game,
};
use std::{
    collections::HashSet,
    sync::mpsc,
    time::{Duration, Instant},
};

/// This is the sound check. Button presses create sounds. The sound effect of each button is
/// changed every few seconds.
pub struct SoundCheck {
    /// The id of the first player to press their button
    pub first_buzz: Option<(usize, Duration)>,
    // Audio channel for fun sound effects (if available)
    audio: Option<mpsc::Sender<String>>,
    start: Instant,
}

impl SoundCheck {
    pub fn new(audio: Option<mpsc::Sender<String>>) -> Self {
        SoundCheck {
            first_buzz: None,
            audio,
            start: Instant::now(),
        }
    }
}

impl game::GameMode for SoundCheck {
    fn update(&mut self, inputs: DevboardEvents) -> DevboardButtonLeds {
        let button_ids = inputs
            .button_events
            .iter()
            .map(|ev| ev.button_index)
            .collect::<HashSet<_>>();

        let t = Instant::now().duration_since(self.start).as_secs() as f32;

        let sounds = button_ids
            .iter()
            .map(|id| {
                let id = (id * 263) / AUDIO_PATHS.len();
                let step = (t + (t / 5.0 * id as f32).sin() * 2.0) as usize / 5;
                let offset = (id + step * 7919) % AUDIO_PATHS.len();
                offset
            })
            .collect::<Vec<_>>();

        // Play player button audio for first buzz. We assume the button events arrive in increasing order of time.
        #[cfg(feature = "audio")]
        if let Some(audio) = &self.audio {
            for sound in sounds {
                audio.send(AUDIO_PATHS[sound].to_string()).unwrap();
            }
        }

        // Create response with LED of winning button enabled
        let mut leds = DevboardButtonLeds {
            button_leds: vec![DevboardButtonLed { enabled: false }; game::NUMBER_OF_BUTTONS],
        };
        for i in button_ids {
            leds.button_leds[i].enabled = true;
        }

        leds
    }
}
