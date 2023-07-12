use std::io::Error;

use crate::utils::varint;
use crate::utils::packet;
use crate::utils::varint::read_varint_buf;

use tokio::net::TcpStream;
use packet::read_packet;
use varint::read_varint_string_buf;

pub async fn handle_login(stream: &mut TcpStream) -> Result<(), Error> {
    let mut packet = read_packet(stream).await?;
    let username = read_varint_string_buf(&mut packet.data)?;

    let has_guid = read_varint_buf(&mut packet.data)? == 1;
    let mut guid: u128 = 0;
    if has_guid {
        guid = u128::from_be_bytes(packet.data.drain(..16).as_slice().try_into().unwrap());
    }
    
    println!("{} {:x?}", username, guid);

    Ok(())
}