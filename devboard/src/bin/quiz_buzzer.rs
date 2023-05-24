#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::str::FromStr;
use defmt::*;
use devboard::{
    button_tasks::{button_task, button_tasks},
    DevboardButtonLed, DevboardButtonLeds, DevboardEvent, DevboardEventType, DevboardEvents, State,
    BUFFER_SIZE, DEBOUNCE_MS, NUM_BUTTONS, Q, STATE_PERIOD_MS,
};
use embassy_executor::Spawner;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::tcp::Error::ConnectionReset;
use embassy_net::{Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::eth::{Ethernet, PacketQueue};
use embassy_stm32::exti::Channel;
use embassy_stm32::gpio::Pin;
use embassy_stm32::gpio::{AnyPin, Level, Output, Speed};
use embassy_stm32::interrupt;
use embassy_stm32::peripherals::ETH;
use embassy_stm32::rng::Rng;
use embassy_time::{Duration, Instant, Timer};
use embedded_io::asynch::{Read, Write};
use embedded_nal_async::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpConnect};
use heapless::Vec;
use rand_core::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};

macro_rules! singleton {
    ($val:expr) => {{
        type T = impl Sized;
        static STATIC_CELL: StaticCell<T> = StaticCell::new();
        let (x,) = STATIC_CELL.init(($val,));
        x
    }};
}

type Device = Ethernet<'static, ETH, GenericSMI>;

button_tasks!(button1, button2, button3, button4, button5, button6);

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<Device>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    // configure LED pins
    let led_pins: [AnyPin; NUM_BUTTONS] = [
        p.PA6.degrade(),
        p.PA8.degrade(),
        p.PI8.degrade(),
        p.PB6.degrade(),
        p.PE3.degrade(),
        p.PB15.degrade(),
    ];
    let mut led_outputs: Vec<Output<AnyPin>, NUM_BUTTONS> = Vec::new();

    for pin in led_pins {
        led_outputs.push(Output::new(pin, Level::Low, Speed::Low));
    }

    // Generate random seed.
    let mut rng = Rng::new(p.RNG);
    let mut seed = [0; 8];
    rng.fill_bytes(&mut seed);
    let seed = u64::from_le_bytes(seed);

    let eth_int = interrupt::take!(ETH);
    let mac_addr = [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF];

    let device = Ethernet::new(
        singleton!(PacketQueue::<16, 16>::new()),
        p.ETH,
        eth_int,
        p.PA1,
        p.PC3,
        p.PA2,
        p.PC1,
        p.PA7,
        p.PC4,
        p.PC5,
        p.PB0,
        p.PB1,
        p.PG13,
        p.PG12,
        p.PC2,
        p.PE2,
        p.PG11,
        GenericSMI,
        mac_addr,
        1,
    );

    // let config = embassy_net::Config::Dhcp(Default::default());
    let config = embassy_net::Config::Static(embassy_net::StaticConfig {
        address: Ipv4Cidr::new(Ipv4Address::new(192, 168, 100, 5), 24),
        dns_servers: Vec::new(),
        gateway: Some(Ipv4Address::new(192, 168, 100, 1)),
    });

    // Init network stack
    let stack = &*singleton!(Stack::new(
        device,
        config,
        singleton!(StackResources::<2>::new()),
        seed
    ));

    // Launch network task
    unwrap!(_spawner.spawn(net_task(&stack)));
    info!("initialized");

    unwrap!(_spawner.spawn(button1(0, p.PG3.degrade(), p.EXTI3.degrade())));
    unwrap!(_spawner.spawn(button2(1, p.PK1.degrade(), p.EXTI1.degrade())));
    unwrap!(_spawner.spawn(button3(2, p.PE6.degrade(), p.EXTI6.degrade())));
    unwrap!(_spawner.spawn(button4(3, p.PB7.degrade(), p.EXTI7.degrade())));
    unwrap!(_spawner.spawn(button5(4, p.PH15.degrade(), p.EXTI15.degrade())));
    unwrap!(_spawner.spawn(button6(5, p.PB4.degrade(), p.EXTI4.degrade())));

    static STATE: TcpClientState<1, 1024, 1024> = TcpClientState::new();
    let client = TcpClient::new(&stack, &STATE);

    loop {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 100, 1), 8000));

        info!("connecting...!");
        let r = client.connect(addr).await;

        if let Err(e) = r {
            info!("connect error: {:?}", e);
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }
        let mut connection = r.unwrap();
        info!("connected!");

        loop {
            let mut state: State = State {
                time: Instant::now().as_millis(),
                button_presses: Default::default(),
            };

            let mut devboard_events = DevboardEvents {
                ms_since_reset: Instant::now().as_millis(),
                number_of_buttons: NUM_BUTTONS,
                button_events: Vec::new(),
            };
            while let Some(press) = Q.dequeue() {
                // let (id, time) = press;
                // let id: usize = id as usize;
                // info!("press! {:?}, {:?}", id, time);

                let dev_board_event = DevboardEvent {
                    button_index: press.0,
                    timestamp: press.1,
                    event_type: DevboardEventType::Pressed,
                };

                //state.button_presses.push(press);

                devboard_events.button_events.push(dev_board_event);

                if devboard_events.button_events.is_full() {
                    break;
                }
            }

            let serialized = serde_json_core::ser::to_string::<DevboardEvents, { BUFFER_SIZE }>(
                &devboard_events,
            )
            .unwrap();
            info!("Serialized: {:?}", serialized);

            let mut buf = [0u8; BUFFER_SIZE];
            let mut read_buf = [0u8; BUFFER_SIZE];
            let request: &str = format_no_std::show(
                &mut buf,
                format_args!("POST /devboard HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: {:?}\r\n\r\n{serialized}", serialized.len())
            ).unwrap();

            let r = connection.write_all(request.as_bytes()).await;
            if let Err(e) = r {
                info!("write error: {:?}", e);

                if e == ConnectionReset {
                    break;
                }
            }

            let body_matcher = "\r\n\r\n";
            let body_matcher_len: usize = 4;

            let body_end_matcher = "\0";

            let mut body_start: usize;
            let body_end: usize;

            let mut reading_counter = 0;
            let mut body: heapless::String<500> = heapless::String::new();
            loop {
                let resp = connection.read(&mut read_buf[reading_counter..]).await;
                match resp {
                    Ok(size) => {
                        reading_counter += size;
                    }
                    Err(_) => {
                        break;
                    }
                }

                let resp = unsafe { core::str::from_utf8_unchecked(&read_buf) };

                let mut start = resp.find(body_matcher);
                match start {
                    Some(size) => {
                        body_start = size + body_matcher_len;
                    }
                    None => {
                        continue;
                    }
                }

                let mut end = resp.find(body_end_matcher);
                match end {
                    Some(size) => {
                        body_end = size;
                    }
                    None => {
                        continue;
                    }
                }

                body = heapless::String::from_str(&resp[body_start..body_end]).unwrap();
                break;
            }

            info!("Led{:?}", body);

            let (dev_button_leds, leds) =
                serde_json_core::from_str::<DevboardButtonLeds>(&body).unwrap();

            for (id, new_state) in dev_button_leds.button_leds.iter().enumerate() {
                match new_state.enabled {
                    true => led_outputs[id].set_level(Level::High),
                    false => led_outputs[id].set_level(Level::Low),
                }
            }

            // info!("Led{:?}", dev_button_leds.button_leds);
            Timer::after(Duration::from_millis(STATE_PERIOD_MS)).await;
        }
    }
}
