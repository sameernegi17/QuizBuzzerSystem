#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::client::{TcpClient, TcpClientState};
use embassy_net::{Ipv4Address, Ipv4Cidr, Stack, StackResources};
use embassy_net::tcp::Error::ConnectionReset;
use embassy_stm32::eth::generic_smi::GenericSMI;
use embassy_stm32::eth::{Ethernet, PacketQueue};
use embassy_stm32::peripherals::ETH;
use embassy_stm32::rng::Rng;
use embassy_stm32::time::mhz;
use embassy_stm32::{interrupt, Config};
use embassy_time::{Duration, Timer};
use embedded_io::asynch::Write;
use embedded_nal_async::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpConnect};
use heapless::Vec;
use rand_core::RngCore;
use static_cell::StaticCell;
use {defmt_rtt as _, panic_probe as _};
use serde::{Serialize, Deserialize};

macro_rules! singleton {
    ($val:expr) => {{
        type T = impl Sized;
        static STATIC_CELL: StaticCell<T> = StaticCell::new();
        let (x,) = STATIC_CELL.init(($val,));
        x
    }};
}

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}


type Device = Ethernet<'static, ETH, GenericSMI>;

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<Device>) -> ! {
    stack.run().await
}
#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    let mut config = Config::default();
    config.rcc.sys_ck = Some(mhz(400));
    config.rcc.hclk = Some(mhz(200));
    config.rcc.pll1.q_ck = Some(mhz(100));
    let p = embassy_stm32::init(config);
    info!("Hello World!");

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
    let stack = &*singleton!(Stack::new(device, config, singleton!(StackResources::<2>::new()), seed));

    // Launch network task
    unwrap!(spawner.spawn(net_task(&stack)));

    info!("Network task initialized");

    // To ensure DHCP configuration before trying connect
    // Timer::after(Duration::from_secs(20)).await;

    static STATE: TcpClientState<1, 1024, 1024> = TcpClientState::new();
    let client = TcpClient::new(&stack, &STATE);

    loop {
        let addr = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(192, 168, 100, 1), 8000));

        
        let mut counter = 0;
        info!("connecting...");
        let r = client.connect(addr).await;
        if let Err(e) = r {
            info!("connect error: {:?}", e);
            Timer::after(Duration::from_secs(1)).await;
            continue;
        }
        let mut connection = r.unwrap();
        info!("connected!");
        let mut point = Point { x: 1, y: 2 };
        loop {

            let serialized = serde_json_core::ser::to_string::<Point,200>(&point).unwrap();

            // Prints serialized = {"x":1,"y":2}

            let mut buf = [0u8; 64];

            let s: &str = format_no_std::show(
                &mut buf,
                format_args!("GET /show/{serialized} HTTP/1.1\r\n\r\n"),
            ).unwrap();


            info!("serialized = {:?}", s);
            
            let r = connection.write_all(s.as_bytes()).await;
            if let Err(e) = r {
                info!("write error: {:?}", e);

                if e == ConnectionReset {
                    break;
                }

                continue;
            }
            point.x +=1;
            point.y +=5;
            Timer::after(Duration::from_secs(1)).await;

            
        }
    }
}
