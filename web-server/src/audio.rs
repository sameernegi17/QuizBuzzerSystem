use std::sync::mpsc;

pub(crate) const AUDIO_PATH_FAIL: &str = "./static/audio/boowomp.mp3";
pub(crate) const AUDIO_PATHS: [&str; 6] = [
    "./static/audio/duck.mp3",
    "./static/audio/icq.mp3",
    "./static/audio/mario.mp3",
    "./static/audio/mgs.mp3",
    "./static/audio/partyblower.mp3",
    "./static/audio/create.mp3",
];

/// Spawn a rusty_audio::Audio in a new thread and return a channel for the play() commands. This
pub fn spawn_audio_thread() -> Option<mpsc::Sender<String>> {
    if !std::path::Path::new(AUDIO_PATH_FAIL).exists()
        || !AUDIO_PATHS
            .iter()
            .all(|path| std::path::Path::new(path).exists())
    {
        println!("Not all audio files were found. Place them into static/audio/");
        log::warn!("Not all audio files were found. Place them into static/audio/");
        return None;
    }

    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        let mut audio = rusty_audio::Audio::new();

        audio.add(AUDIO_PATH_FAIL, AUDIO_PATH_FAIL);
        for path in AUDIO_PATHS.iter() {
            audio.add(path, path);
        }

        loop {
            let audio_identifier = rx.recv().unwrap();
            audio.play(audio_identifier);
        }
    });
    Some(tx)
}
