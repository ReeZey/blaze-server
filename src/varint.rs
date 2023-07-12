use std::io::{Error, ErrorKind};

use tokio::{net::TcpStream, io::AsyncReadExt};

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub async fn read_varint(stream: &mut TcpStream) -> Result<u32, Error> {
    let mut value: u32 = 0;
    let mut size: u32 = 0;
    let mut single_byte = vec![0u8; 1];
    stream.read_exact(&mut single_byte).await?;
    let mut current_byte = single_byte[0];

    while (current_byte & CONTINUE_BIT) == CONTINUE_BIT {
        value |= ((current_byte & SEGMENT_BITS) as u32) << (size * 7);
        size += 1;
        if size > 5 {
            return Err(Error::new(ErrorKind::InvalidData, "varint too large"));
        };
        stream.read_exact(&mut single_byte).await.unwrap();
        current_byte = single_byte[0];
    }

    return Ok((value | (((current_byte & SEGMENT_BITS) as u32) << (size * 7))).into());
}

pub fn read_varint_buf(buffer: &mut Vec<u8>) -> Result<u32, Error> {
    let mut value: u32 = 0;
    let mut size: u32 = 0;
    let mut current_byte: u8 = buffer[0];
    buffer.remove(0);

    while (current_byte & CONTINUE_BIT) == CONTINUE_BIT {
        value |= ((current_byte & SEGMENT_BITS) as u32) << (size * 7);
        size += 1;
        if size > 5 {
            return Err(Error::new(ErrorKind::InvalidData, "varint too large"));
        };
        current_byte = buffer[0];
        buffer.remove(0);
    }

    return Ok((value | (((current_byte & SEGMENT_BITS) as u32) << (size * 7))).into());
}

pub async fn _read_varint_string(stream: &mut TcpStream) -> Result<String, Error> {
    let stringlen = read_varint(stream).await?;
    let mut data: Vec<u8> = vec![0; stringlen as usize];
    stream.read_exact(&mut data).await.unwrap();
    return Ok(String::from_utf8(data).unwrap());
}

pub fn read_varint_string_buf(buffer: &mut Vec<u8>) -> Result<String, Error> {
    let stringlen = read_varint_buf(buffer).unwrap() as usize;
    if buffer.len() < stringlen {
        return Err(Error::new(ErrorKind::InvalidData, "Buffer too small?"));
    }
    return Ok(String::from_utf8(buffer.drain(0..stringlen).as_slice().to_vec()).unwrap());
}

pub fn write_varint(buffer: &mut Vec<u8>, mut number: u32) {
    loop {
        let mut byte = number as u8;

        number >>= 6;
        let done = number == 0;
        if done {
            byte &= !CONTINUE_BIT;
        } else {
            number >>= 1;
            byte |= CONTINUE_BIT;
        }

        buffer.push(byte);

        if done {
            return;
        };
    }
}

pub fn varint_length(number: u32) -> u32 {
    let mut buffer = vec![];
    write_varint(&mut buffer, number);
    return buffer.len() as u32;
}

pub fn write_varint_string(buffer: &mut Vec<u8>, string: String) {
    write_varint(buffer, string.len() as u32);
    buffer.extend_from_slice(String::into_bytes(string).as_slice())
}