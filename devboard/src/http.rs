use core::str::FromStr;
use defmt::info;
use embassy_net::tcp::client::{TcpClient, TcpConnection};
use embassy_stm32::{
    eth::{generic_smi::GenericSMI, Ethernet},
    peripherals::ETH,
};
use embedded_io::asynch::Read;
use embedded_io::asynch::Write;
use embedded_nal_async::SocketAddr;
use embedded_nal_async::TcpConnect;
use heapless::String;

use crate::BUFFER_SIZE;

static BODY_MATCHER_START: &str = "\r\n\r\n";
static BODY_MATCHER_START_LEN: usize = 4;
static BODY_MATCHER_END: &str = "\0";

pub struct TinyHttpClient<'conn> {
    conn: TcpConnection<'conn, 1, 1024, 1024>,
    buffer: [u8; BUFFER_SIZE],
}

impl<'conn> TinyHttpClient<'conn> {
    pub async fn new<'client, 'eth>(
        tcp_client: &'client TcpClient<'client, Ethernet<'eth, ETH, GenericSMI>, 1, 1024, 1024>,
        addr: SocketAddr,
    ) -> Result<TinyHttpClient<'conn>, ()>
    where
        'client: 'conn,
    {
        let conn = tcp_client.connect(addr).await;
        if let Ok(conn) = conn {
            return Ok(Self {
                conn,
                buffer: [0u8; BUFFER_SIZE],
            });
        } else {
            return Err(());
        }
    }

    pub async fn get_req(
        &mut self,
        payload: &str,
        response_buffer: &mut [u8; BUFFER_SIZE],
    ) -> String<500> {
        let request: &str = format_no_std::show(
            &mut self.buffer,
            format_args!("POST /devboard HTTP/1.1\r\nContent-Type: application/json\r\nContent-Length: {:?}\r\n\r\n{payload}", payload.len())
        ).unwrap();

        let _r = self.conn.write_all(request.as_bytes()).await;

        // Reset response buffer
        *response_buffer = [0u8; BUFFER_SIZE];

        // An async read does not guarantee to read all bytes in one go. We loop
        // until we read zero bytes or run into an error.
        let mut cursor_pos = 0;
        let mut retries = 0;
        loop {
            match self.conn.read(&mut response_buffer[cursor_pos..]).await {
                Ok(bytes_read) => {
                    if retries >= 100 {
                        return String::new();
                    }
                    if bytes_read == 0 {
                        retries += 1;
                        continue;
                    }
                    cursor_pos += bytes_read;
                    if let Some(response) =
                        extract_payload(&unsafe { core::str::from_utf8_unchecked(response_buffer) })
                    {
                        return response;
                    }
                }
                Err(_) => return String::new(),
            }
        }
    }
}

pub fn extract_payload<const N: usize>(response: &str) -> Option<String<N>> {
    let start_pos = response.find(BODY_MATCHER_START);
    let end_pos = response.find(BODY_MATCHER_END);

    match (start_pos, end_pos) {
        (Some(start_pos), Some(end_pos)) => {
            let start_pos = start_pos + BODY_MATCHER_START_LEN;
            let end_pos = end_pos;

            return Some(heapless::String::from_str(&response[start_pos..end_pos]).unwrap());
        }
        _ => return None,
    }
}
