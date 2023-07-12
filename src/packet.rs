use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};
use crate::varint::{read_varint, write_varint, varint_length};
use std::io::Error;

#[derive(Debug)]
pub struct Packet {
    pub length: u32,
    pub packet_id: u32,
    pub data: Vec<u8>
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
    write_varint(&mut data_buffer, packet.packet_id);
    data_buffer.extend(packet.data);

    let mut outgoing_packet: Vec<u8> = vec![];
    write_varint(&mut outgoing_packet, data_buffer.len() as u32);
    outgoing_packet.extend(data_buffer);

    stream.write_all(&outgoing_packet).await?;

    Ok(())
}