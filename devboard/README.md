# Setup

1. Obtain an
   [STM32H745I-DISCO](https://www.st.com/en/evaluation-tools/stm32h745i-disco.html)
   discovery board
2. Switch to the directory of this README: `cd devboard`
3. Connect the board via its `STLK` (`CN14`) Micro USB socket
4. Make sure the jumper `JP8` (next to that socket) is set to `STLK`

# Examples

## Blinky

Run `cargo run --bin blinky`

You should see the red and green LEDs and the LCD blinking.
The output should look similar to the following:

```
cargo run --bin blinky
    Finished dev [optimized + debuginfo] target(s) in 0.07s
     Running `probe-rs-cli run --chip STM32H745XIHx target/thumbv7em-none-eabihf/debug/blinky`
     Erasing sectors ✔ [00:00:01] [##################################################################################################################################################] 128.00 KiB/128.00 KiB @ 65.36 KiB/s (eta 0s )
 Programming pages   ✔ [00:00:00] [####################################################################################################################################################] 24.00 KiB/24.00 KiB @ 43.93 KiB/s (eta 0s )    Finished in 2.518s
0.000000 INFO Hello World!
└─ src/bin/blinky.rs:14
0.000061 INFO high
└─ src/bin/blinky.rs:21
0.100128 INFO low
└─ src/bin/blinky.rs:27
0.200195 INFO high
└─ src/bin/blinky.rs:21
0.300262 INFO low
└─ src/bin/blinky.rs:27
```

## Button / External Interrupt

Run `cargo run --bin button_exti`

Pressing the blue button should result in output similar to the following:

```
cargo run --bin button_exti
Finished dev [optimized + debuginfo] target(s) in 0.07s
     Running `probe-rs-cli run --chip STM32H745XIHx target/thumbv7em-none-eabihf/debug/button_exti`
    Finished in 2.485s
0.000000 INFO Hello World!
└─ src/bin/button_exti.rs:14
0.000030 INFO Press the USER button...
└─ src/bin/button_exti.rs:19
1.956268 INFO Pressed!
└─ src/bin/button_exti.rs:23
2.084197 INFO Released!
└─ src/bin/button_exti.rs:25
```

## Ethernet

1. Connect an Ethernet cable between the board and your PC.
2. Configure the respective Ethernet interface on your PC with the following static IP
   address: `192.168.100.1/24`
3. Start listening for incoming connections from the board: `nc -v -l 192.168.100.1 8000`
4. In a second terminal, run: `cargo run --bin eth_client`

The output (second terminal) should look similar to the following:

```
cargo run --bin eth_client
    Finished dev [optimized + debuginfo] target(s) in 0.07s
     Running `probe-rs-cli run --chip STM32H745XIHx target/thumbv7em-none-eabihf/debug/eth_client`
    Finished in 4.462s
0.000000 INFO Hello World!
└─ src/bin/eth_client.rs:47
0.001495 DEBUG Acquired IP configuration:
0.001525 DEBUG    IP address:      192.168.100.5/24
0.001556 DEBUG    Default gateway: 192.168.100.1
0.001586 INFO Network task initialized
└─ src/bin/eth_client.rs:94
0.001586 INFO connecting...
```

The output (first terminal) should look similar to the following:

```
nc -v -l 192.168.100.1 8000
Listening on <HOSTNAME> 8000
Connection received on 192.168.100.5 <SOURCE PORT>
Hello
Hello
Hello
Hello
Hello
Hello
Hello
```

If it does not work, check the following:

- Firewall settings
    - For testing, you can disable the firewall with `sudo ufw disable`
- IP configuration
    - Make sure you are using and configuring the correct Ethernet interface
    - Use NetworkManager or manual configuration, but not both

# Debugging

## Installation

Debugging requires `gdb-multiarch` / `arm-none-eabi-gdb` and `openocd`.
For information on how to install them, have a look at
[this](https://docs.rust-embedded.org/book/intro/install/linux.html#packages)
of the Embedded Rust Book.

## Start GDB Server

Connect the board and run the following commands
([source](https://docs.rust-embedded.org/book/start/hardware.html#debugging)):

```sh
cd devboard
openocd
```

Example output:

```
Open On-Chip Debugger 0.11.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
Info : Listening on port 6666 for tcl connections
Info : Listening on port 4444 for telnet connections
Info : clock speed 1800 kHz
Info : STLINK V3J10M3 (API v3) VID:PID 0483:374E
Info : Target voltage: 3.276494
Info : stm32h7x.cpu0: hardware has 8 breakpoints, 4 watchpoints
Info : starting gdb server for stm32h7x.cpu0 on 3333
Info : Listening on port 3333 for gdb connections
```

## Connect a GDB Client

In a second terminal run the following command.
Adapt the path to the binary you want to debug accordingly. 

```sh
cd devboard
gdb-multiarch -x openocd.gdb target/thumbv7em-none-eabihf/debug/blinky [--tui]
```
