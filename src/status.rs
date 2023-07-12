use crate::varint;
use crate::packet;
use crate::varint::varint_length;

use bson::{Document, Bson};
use packet::{write_packet, read_packet, Packet};
use tokio::net::TcpStream;
use varint::write_varint_string;
use std::io::Error;

pub async fn handle_status(mut stream: &mut TcpStream) -> Result<(), Error> {
    let mut status: Document = Document::default();
    let mut version_status: Document = Document::default();
    version_status.insert("name", "1.20");
    version_status.insert("protocol", 763);
    status.insert("version", version_status);

    let mut players = vec![];
    for _ in 0..1 {
        let mut player: Document = Document::default();
        player.insert("name", "ReeZey");
        player.insert("id", "2a350988-50ac-41df-b274-1b5eb6e633c1");
        players.push(player);
    }
    
    let mut players_status: Document = Document::default();
    players_status.insert("max", 69);
    players_status.insert("online", 1);
    players_status.insert("sample", players);
    status.insert("players", players_status);

    let mut description_status: Document = Document::default();
    description_status.insert("text", "yo");
    status.insert("description", description_status);
    
    status.insert("enforcesSecureChat", false);

    let bson_min_broder: Bson = status.into();

    let mut output_buffer = vec![];
    write_varint_string(&mut output_buffer, bson_min_broder.into_relaxed_extjson().to_string());

    let packet_id = 0;

    write_packet(&mut stream, Packet {
        packet_id,
        length: output_buffer.len() as u32 + varint_length(packet_id),
        data: output_buffer
    }).await?;

    let packet = read_packet(&mut stream).await?;
    write_packet(&mut stream, packet).await?;

    Ok(())
}