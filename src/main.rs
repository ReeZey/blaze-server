use packet::read_packet;
use tokio::net::TcpListener;
use varint::{read_varint_buf, read_varint_string_buf};
use bson::{Document, Bson};

mod packet;
mod varint;

use packet::{write_packet, Packet};
use varint::write_varint_string;

#[tokio::main()]
async fn main() {
    let listener = TcpListener::bind("0.0.0.0:51413").await.unwrap();

    loop {
        let (mut stream, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            let mut packet = match read_packet(&mut stream).await {
                Some(packet) => packet,
                None => {
                    return;
                }
            };

            println!("New connection from {}", stream.peer_addr().unwrap());

            //let version = read_varint_buf(&mut packet.data).unwrap();

            match packet.packet_id {
                0 => {
                    //println!("{:#?}", packet);
                    //fs::write("test.bin", &packet.data).await.unwrap();

                    let _protocol_id = read_varint_buf(&mut packet.data).unwrap();
                    let _host = read_varint_string_buf(&mut packet.data).unwrap();
                    let _port_parts: Vec<u8> = packet.data.drain(..2).collect();
                    let _port = u16::from_be_bytes([_port_parts[0], _port_parts[1]]);
                    let next_state = read_varint_buf(&mut packet.data).unwrap();

                    //println!("{} {} {} {}", _protocol_id, _host, _port, next_state);

                    match next_state {
                        1 => {
                            let mut status: Document = Document::default();
                            let mut version_status: Document = Document::default();
                            version_status.insert("name", "1.20.1");
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
                            let json = bson_min_broder.into_relaxed_extjson();

                            let mut output_buffer = vec![];
                            write_varint_string(&mut output_buffer, json.to_string());

                            //println!("{:?}", output_buffer);
                            //fs::write("test2.bin", &output_buffer).await.unwrap();
                            
                            let outgoing_packet = Packet {
                                packet_id: 0,
                                length: output_buffer.len() as u32 + 1,
                                data: output_buffer
                            };

                            
                            //println!("{:?}", outgoing_packet);

                            write_packet(&mut stream, outgoing_packet).await;

                            let packet = match read_packet(&mut stream).await {
                                Some(packet) => packet,
                                None => {
                                    return;
                                }
                            };
                            write_packet(&mut stream, packet).await;
                        },
                        2 => {
                            //TODO: this get flodded sometimes?
                            let mut packet = read_packet(&mut stream).await.unwrap();

                            let username = read_varint_string_buf(&mut packet.data).unwrap();

                            println!("{}", username);
                        }
                        _ => {

                        }
                    }
                }
                _ => {}
            }

            //println!("{}", version);
        });
    }
}
