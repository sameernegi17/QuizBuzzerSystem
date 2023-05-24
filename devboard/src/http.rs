use core::str::FromStr;

use heapless::String;

static BODY_MATCHER_START: &str = "\r\n\r\n";
static BODY_MATCHER_START_LEN: usize = 4;
static BODY_MATCHER_END: &str = "\0";

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
