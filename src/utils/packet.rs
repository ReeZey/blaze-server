use super::varint;

use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};
use varint::{read_varint, write_varint, varint_length};
use std::io::Error;

#[derive(Debug)]
pub struct Packet {
    pub length: u32,
    pub packet_id: u32,
    pub data: Vec<u8>
}

impl Packet {
    pub fn new(packet_id: u32, data: Vec<u8>) -> Packet {
        return Packet {
            length: varint_length(packet_id) + data.len() as u32,
            packet_id,
            data
        }
    }
}

pub async fn read_packet(mut stream: &mut TcpStream) -> Result<Packet, Error> {
    let length = read_varint(&mut stream).await?;
    let packet_id = read_varint(&mut stream).await?;

    //println!("{:?}", packet_id);

    let packet_length = length - varint_length(packet_id);
    let mut data = vec![0; packet_length as usize];
    stream.read_exact(&mut data).await?;

    Ok(Packet {
        length,
        packet_id,
        data
    })
}

pub async fn write_packet(stream: &mut TcpStream, packet: Packet) -> Result<(), Error> {
    let mut data_buffer: Vec<u8> = vec![];
    write_varint(&mut data_buffer, varint_length(packet.packet_id) + packet.data.len() as u32);
    write_varint(&mut data_buffer, packet.packet_id);
    data_buffer.extend(packet.data);

    stream.write_all(&data_buffer).await?;

    Ok(())
}