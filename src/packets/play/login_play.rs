use std::io::Error;
use tokio::{net::TcpStream, fs::File, io::{AsyncReadExt, AsyncWriteExt}};
use config::Config;

use crate::utils::{varint::{write_varint_string, write_varint}, packet::{Packet, write_packet}};

struct LoginPlay {
    entity_id: i32,
    is_hardcore: bool,
    gamemode: u8,
    prev_gamemode: u8,
    dimensions: Vec<String>,
    registry_codec: Vec<u8>,
    dimension_type: String,
    dimension_name: String,
    hashed_seed: i64,
    max_players: u32,
    view_distance: u32,
    simulation_distance: u32,
    reduced_debug_info: bool,
    enable_respawn_screen: bool,
    debug: bool,
    flat: bool,
    has_death: bool,
    death_dimension: String,
    death_location: [f64; 3],
    portal_cooldown: u32,
}

pub async fn handle_login(stream: &mut TcpStream) -> Result<(), Error> {
    let config = Config::builder()
        .add_source(config::File::with_name("settings"))
        .build()
        .unwrap();

    let max_players = config.get_int("max_players").unwrap() as u32;
    let view_distance = config.get_int("view_distance").unwrap() as u32;
    let simulation_distance = config.get_int("simulation_distance").unwrap() as u32;

    let dimensions = vec![
        "minecraft:overworld".to_owned(), 
        "minecraft:the_nether".to_owned(), 
        "minecraft:the_end".to_owned()
    ];

    let mut registry_codec = vec![];
    let mut handle = File::open("registry-codec.nbt").await.unwrap();
    handle.read_to_end(&mut registry_codec).await.unwrap();
    
    let play = LoginPlay {
        entity_id: 0,
        is_hardcore: false,
        gamemode: 0,
        prev_gamemode: 0,
        dimensions,
        registry_codec,
        dimension_type: "minecraft:overworld".to_owned(),
        dimension_name: "minecraft:overwarld".to_owned(),
        hashed_seed: 0,
        max_players,
        view_distance,
        simulation_distance,
        reduced_debug_info: false,
        enable_respawn_screen: false,
        debug: false,
        flat: false,
        has_death: false,
        death_dimension: "nah".to_owned(),
        death_location: [0.0, 0.0, 0.0],
        portal_cooldown: 0,
    };

    let mut buffer = vec![];
    buffer.extend(play.entity_id.to_be_bytes());
    buffer.push(play.is_hardcore as u8);
    buffer.push(play.gamemode);
    buffer.push(play.prev_gamemode as u8);
    write_varint(&mut buffer, play.dimensions.len() as u32);
    for dim in play.dimensions.clone()  {
        write_varint_string(&mut buffer, dim.clone());
    }
    buffer.extend(play.registry_codec);
    write_varint_string(&mut buffer, play.dimension_type.to_string());
    write_varint_string(&mut buffer, play.dimension_name.to_string());
    buffer.extend(play.hashed_seed.to_be_bytes());
    write_varint(&mut buffer, play.max_players);
    write_varint(&mut buffer, play.view_distance);
    write_varint(&mut buffer, play.simulation_distance);
    buffer.push(play.reduced_debug_info as u8);
    buffer.push(play.enable_respawn_screen as u8);
    buffer.push(play.debug as u8);
    buffer.push(play.flat as u8);
    buffer.push(play.has_death as u8);
    write_varint(&mut buffer, play.portal_cooldown);

    write_packet(stream, Packet::new(0x28, buffer)).await.unwrap();

    Ok(())
}