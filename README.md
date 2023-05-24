# QuizBuzzerSystem

This repository holds the code for an advanced quiz buzzer system. More than
just knowing who pressed a button first, we can tell the exact reaction times
etc.

## Setup

For the `web-server` to run, you need to install a library for the audio
dependency.

```bash
sudo apt-get update && sudo apt-get install -y libasound2-dev
```

For the `devboard`, you need to configure a local-link ethernet connection with
you laptop's IP fixed to `192.168.100.2`.

## `devboard`

The `devboard` crate is built for the [STM32H745I-DISCO][board]. It contains the code
which will run on the board.

## `web-server`

The `web-server` provides both the backend with which the board communicates and
hosts the frontend that users interact with.

Running the web server

```bash
# Currently, you need to start from this path due to hardcoded paths.
cd web-server/src

# Start with default/dev config
cargo run --release

# Start with production config
RUN_MODE=prod cargo run --release
```

In the production config, you can navigate to
[http://192.168.100.1:8000/](http://192.168.100.1:8000/)
to see the frontend.

[board]: https://www.st.com/en/evaluation-tools/stm32h745i-disco.html
