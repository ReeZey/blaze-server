use tokio::{net::TcpStream, io::AsyncReadExt};

const SEGMENT_BITS: u8 = 0x7F;
const CONTINUE_BIT: u8 = 0x80;

pub async fn read_varint(stream: &mut TcpStream) -> Option<u32> {
    let mut value: u32 = 0;
    let mut size: u32 = 0;
    let mut single_byte = vec![0u8; 1];
    match stream.read_exact(&mut single_byte).await {
        Ok(_) => {}
        Err(_) => {
            return None;
        }
    };
    let mut current_byte = single_byte[0];

    while (current_byte & CONTINUE_BIT) == CONTINUE_BIT {
        value |= ((current_byte & SEGMENT_BITS) as u32) << (size * 7);
        size += 1;
        if size > 5 {
            return None;
        };
        stream.read_exact(&mut single_byte).await.unwrap();
        current_byte = single_byte[0];
    }

    return Some((value | (((current_byte & SEGMENT_BITS) as u32) << (size * 7))).into());
}

pub fn read_varint_buf(buffer: &mut Vec<u8>) -> Option<u32> {
    let mut value: u32 = 0;
    let mut size: u32 = 0;
    let mut current_byte: u8 = buffer[0];
    buffer.remove(0);

    while (current_byte & CONTINUE_BIT) == CONTINUE_BIT {
        value |= ((current_byte & SEGMENT_BITS) as u32) << (size * 7);
        size += 1;
        if size > 5 {
            return None;
        };
        current_byte = buffer[0];
        buffer.remove(0);
    }

    return Some((value | (((current_byte & SEGMENT_BITS) as u32) << (size * 7))).into());
}

pub async fn _read_varint_string(stream: &mut TcpStream) -> Option<String> {
    let stringlen = read_varint(stream).await? as usize;
    let mut data: Vec<u8> = vec![0; stringlen];
    stream.read_exact(&mut data).await.unwrap();
    return Some(String::from_utf8(data).unwrap());
}

pub fn read_varint_string_buf(buffer: &mut Vec<u8>) -> Option<String> {
    let stringlen = read_varint_buf(buffer).unwrap() as usize;
    if buffer.len() < stringlen {
        return None;
    }
    return Some(String::from_utf8(buffer.drain(0..stringlen).as_slice().to_vec()).unwrap());
}
pub fn write_varint(buffer: &mut Vec<u8>, number: u32) {
    let mut val: i32 = number as i32;
    loop {
        let mut byte = val as u8;

        val >>= 6;
        let done = val == 0 || val == -1;
        if done {
            byte &= !CONTINUE_BIT;
        } else {
            val >>= 1;
            byte |= CONTINUE_BIT;
        }

        buffer.push(byte);

        if done {
            return;
        };
    }
}

pub fn write_varint_string(buffer: &mut Vec<u8>, string: String) {
    write_varint(buffer, string.len() as u32);
    buffer.extend_from_slice(String::into_bytes(string).as_slice())
}