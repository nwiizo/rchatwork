extern crate curl;

use curl::easy::{Easy,List};

// Capture output into a local `Vec`.
fn main() {
    let mut dst = Vec::new();
    let mut easy = Easy::new();
    let mut roomid:&str  = "*************";
    let mut cw_token:&str = "**********************";
    let mut data = "body=this is the body".as_bytes();
    let mut list = List::new();
        list.append("X-ChatWorkToken: *************************").unwrap();
        easy.http_headers(list).unwrap();
    easy.url("https://api.chatwork.com/v2/rooms/***************/messages").unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        dst.extend_from_slice(data);
        Ok(data.len())
    }).unwrap();
    transfer.perform().unwrap();
}
