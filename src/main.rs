use std::collections::HashMap;
use std::sync::Arc;

use packets::play::login_play;
use packets::{status, login_success};
use tokio::net::{TcpListener, TcpStream};

mod utils;
mod packets;

use tokio::sync::Mutex;
use utils::packet::read_packet;
use utils::varint::{read_varint_buf, read_varint_string_buf};
use tokio::io::Error;
use utils::player::Player;

#[tokio::main()]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:51413").await.unwrap();

    let players: Arc<Mutex<HashMap<String, Player>>> = Arc::new(Mutex::new(HashMap::default()));

    loop {
        let client_players = players.clone();
        let (stream, _) = listener.accept().await.unwrap();

        //println!("New connection from {}", stream.peer_addr().unwrap());

        tokio::spawn(async move {
            match handle_client(stream, client_players).await {
                Ok(_) => {}
                Err(_) => {}
            };
        });
    }
}

async fn handle_client(mut stream: TcpStream, players: Arc<Mutex<HashMap<String, Player>>>) -> Result<(), Error> {
    let mut packet = read_packet(&mut stream).await?;

    match packet.packet_id {
        0 => {
            let _protocol_id = read_varint_buf(&mut packet.data).unwrap();
            let _host = read_varint_string_buf(&mut packet.data).unwrap();
            let _port_parts: Vec<u8> = packet.data.drain(..2).collect();
            let _port = u16::from_be_bytes([_port_parts[0], _port_parts[1]]);
            let next_state = read_varint_buf(&mut packet.data).unwrap();

            match next_state {
                1 => {
                    //why mojang
                    read_packet(&mut stream).await.unwrap();

                    status::handle_status(&mut stream, players).await?;
                },
                2 => {
                    login_success::handle_login(&mut stream, players).await.unwrap();
                    login_play::handle_login(&mut stream).await.unwrap();
                    
                    loop {

                    }
                }
                _ => {}
            }
        }
        _ => {}
    }

    Ok(())
}
