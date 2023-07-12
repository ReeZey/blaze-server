use std::io::Error;
use std::io::ErrorKind;
use std::sync::Arc;

use crate::utils::packet::Packet;
use crate::utils::varint;
use crate::utils::packet;
use crate::utils::player;
use crate::utils::varint::write_varint;

use bson::Bson;
use bson::Document;
use tokio::net::TcpStream;
use packet::{read_packet, write_packet};
use tokio::sync::Mutex;
use varint::{read_varint_string_buf, read_varint_buf, write_varint_string};
use player::Player;
use std::collections::HashMap;

pub async fn handle_login(stream: &mut TcpStream, players: Arc<Mutex<HashMap<String, Player>>>) -> Result<(), Error> {
    let mut packet = read_packet(stream).await?;
    let username = read_varint_string_buf(&mut packet.data)?;

    let has_guid = read_varint_buf(&mut packet.data)? == 1;
    let mut guid: u128 = 0;
    if has_guid {
        guid = u128::from_be_bytes(packet.data.drain(..16).as_slice().try_into().unwrap());
    }
    
    let mut players = players.lock().await;

    if players.contains_key(&username){
        let mut response: Document = Document::default();
        response.insert("text", "du får inte vara med");
        response.insert("bold", true);
    
        let bson_min_broder: Bson = response.into();
    
        let mut output_buffer = vec![];
        write_varint_string(&mut output_buffer, bson_min_broder.into_relaxed_extjson().to_string());
    
        write_packet(stream, Packet::new(0, output_buffer)).await?;
        return Err(Error::new(ErrorKind::Other, "Player already connected"));
    }

    println!("New player authenticated: {} {:x?}", username, guid);
    players.insert(username.clone(), Player{
        username: username.clone(),
        guid
    });
    //println!("{:#?}", players);
    drop(players);

    let mut output_buffer = vec![];
    output_buffer.extend(guid.to_be_bytes());
    output_buffer.extend(username.as_bytes());
    write_varint(&mut output_buffer, 0);

    write_packet(stream, Packet::new(2, output_buffer)).await?;

    Ok(())
}