use tokio::{net::TcpStream, io::{AsyncReadExt, AsyncWriteExt}};
use crate::varint::{read_varint, write_varint};

#[derive(Debug)]
pub struct Packet {
    pub length: u32,
    pub packet_id: u32,
    pub data: Vec<u8>
}

pub async fn read_packet(mut stream: &mut TcpStream) -> Option<Packet> {
    let length = match read_varint(&mut stream).await {
        Some(length) => length,
        None => { 
            return None;
        }
    };
    let packet_id = read_varint(&mut stream).await.unwrap();

    //println!("{}", length);

    let mut data = vec![];
    stream.read_buf(&mut data).await.unwrap();

    //println!("{:x?}", data);

    Some(Packet {
        length,
        packet_id,
        data
    })
}

pub async fn write_packet(stream: &mut TcpStream, packet: Packet) {
    let mut data_buffer: Vec<u8> = vec![];
    write_varint(&mut data_buffer, packet.packet_id);
    data_buffer.extend(packet.data);

    let mut outgoing_packet: Vec<u8> = vec![];
    write_varint(&mut outgoing_packet, data_buffer.len() as u32);
    outgoing_packet.extend(data_buffer);

    match stream.write_all(&outgoing_packet).await {
        Ok(()) => {},
        Err(_) => {}
    };
}