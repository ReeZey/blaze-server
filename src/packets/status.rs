use crate::utils::player::format_guid;
use crate::utils::varint;
use crate::utils::packet;
use crate::utils::player;

use bson::{Document, Bson};
use packet::{write_packet, read_packet, Packet};
use varint::{write_varint_string, varint_length};
use tokio::net::TcpStream;
use std::collections::HashMap;
use std::io::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use player::Player;

pub async fn handle_status(mut stream: &mut TcpStream, players: Arc<Mutex<HashMap<String, Player>>>) -> Result<(), Error> {
    let mut status: Document = Document::default();
    let mut version_status: Document = Document::default();
    version_status.insert("name", "1.20");
    version_status.insert("protocol", 763);
    status.insert("version", version_status);

    let players = players.lock().await;
    let mut players_array = vec![];
    for (_, player) in players.iter() {
        let mut player_json: Document = Document::default();
        player_json.insert("name", player.username.clone());
        player_json.insert("id", format_guid(player.guid));
        players_array.push(player_json);
    }
    drop(players);
    
    let mut players_status: Document = Document::default();
    players_status.insert("max", 69);
    players_status.insert("online", players_array.len() as u32);
    players_status.insert("sample", players_array);
    status.insert("players", players_status);

    let mut description_status: Document = Document::default();
    description_status.insert("text", "yo");
    status.insert("description", description_status);
    
    status.insert("enforcesSecureChat", false);

    let bson_min_broder: Bson = status.into();

    let mut output_buffer = vec![];
    write_varint_string(&mut output_buffer, bson_min_broder.into_relaxed_extjson().to_string());

    let packet_id = 0;

    write_packet(&mut stream, Packet::new(packet_id, output_buffer)).await?;

    let packet = read_packet(&mut stream).await?;
    write_packet(&mut stream, packet).await?;

    Ok(())
}