#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use core::ops::Index;
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::tcp::Error::ConnectionReset;
use embassy_net::{Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::eth::{Ethernet, PacketQueue};
use embassy_stm32::exti::AnyChannel;
use embassy_stm32::exti::Channel;
use embassy_stm32::exti::ExtiInput;
use embassy_stm32::gpio::AnyPin;
use embassy_stm32::gpio::Pin;
use embassy_stm32::gpio::{Input, Pull};
use embassy_stm32::interrupt;
use embassy_stm32::peripherals::ETH;
use embassy_stm32::rng::Rng;
use embassy_time::{Duration, Instant, Timer};
use embedded_io::asynch::{Write, Read};
use embedded_nal_async::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpConnect};
use heapless::mpmc::Q64;
use heapless::Vec;
use rand_core::RngCore;
use serde::__private::ser::serialize_tagged_newtype;
use serde::Serialize;
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

const NUM_BUTTONS: i32 = 6;
const NUM_BUTTON_PRESSES_PER_MSG: usize = 20;
const DEBOUNCE_MS: u64 = 100;
const STATE_PERIOD_MS: u64 = 1000;
const BUFFER_SIZE: usize = 100 + (NUM_BUTTON_PRESSES_PER_MSG * 20);
static Q: Q64<(usize, u64)> = Q64::new();

#[derive(Serialize, Debug)]
struct State {
    time: u64,
    button_presses: Vec<(usize, u64), NUM_BUTTON_PRESSES_PER_MSG>,
}

#[derive(Debug, Serialize)]
pub struct DevboardEvents {
    pub number_of_buttons: i32,
    pub ms_since_reset: u64,
    pub button_events: Vec<DevboardEvent,NUM_BUTTON_PRESSES_PER_MSG>,
}

#[derive(Debug, Serialize)]
pub struct DevboardEvent {
    pub button_index: usize,
    pub event_type: DevboardEventType,
    pub timestamp: u64,
}

#[derive(Debug, Serialize)]
pub enum DevboardEventType {
    Pressed,
    Released,
}

type Device = Ethernet<'static, ETH, GenericSMI>;

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<Device>) -> ! {
    stack.run().await
}

#[embassy_executor::task]
async fn async_task1(id: usize, button: AnyPin, exti_inp: AnyChannel) -> ! {
    let button = Input::new(button, Pull::Up);
    let mut button = ExtiInput::new(button, exti_inp);

    loop {
        button.wait_for_rising_edge().await;
        Q.enqueue((id, Instant::now().as_millis())).ok();
        Timer::after(Duration::from_millis(DEBOUNCE_MS)).await;
    }
}

#[embassy_executor::task]
async fn async_task2(id: usize, button: AnyPin, exti_inp: AnyChannel) -> ! {
    let button = Input::new(button, Pull::Up);
    let mut button = ExtiInput::new(button, exti_inp);

    loop {
        button.wait_for_rising_edge().await;
        Q.enqueue((id, Instant::now().as_millis())).ok();
        Timer::after(Duration::from_millis(DEBOUNCE_MS)).await;
    }
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

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

    unwrap!(_spawner.spawn(async_task1(0, p.PG3.degrade(), p.EXTI3.degrade())));
    unwrap!(_spawner.spawn(async_task2(1, p.PK1.degrade(), p.EXTI1.degrade())));

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
            let mut state: State = State { time: Instant::now().as_millis(), button_presses: Default::default() };

            let mut devboard_events = DevboardEvents {ms_since_reset: Instant::now().as_millis(), number_of_buttons : NUM_BUTTONS, button_events : Vec::new()};
            while let Some(press) = Q.dequeue() {
                // let (id, time) = press;
                // let id: usize = id as usize;
                // info!("press! {:?}, {:?}", id, time);

                let dev_board_event = DevboardEvent {button_index: press.0 , timestamp : press.1 , event_type : DevboardEventType::Pressed };

                //state.button_presses.push(press);

                devboard_events.button_events.push(dev_board_event);

                if devboard_events.button_events.is_full() { break }
            }

            let serialized = serde_json_core::ser::to_string::<DevboardEvents, { BUFFER_SIZE }>(&devboard_events).unwrap();
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
            let resp = connection.read(&mut read_buf).await;
            let resp = unsafe {core::str::from_utf8_unchecked(&read_buf)};
            info!("{:?}",resp);
            Timer::after(Duration::from_millis(STATE_PERIOD_MS)).await;
        }
    }
}
