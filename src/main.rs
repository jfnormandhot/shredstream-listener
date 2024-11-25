// use solana_sdk::{transaction::Transaction, signature::Signature};
// use solana_sdk::*;
use solana_sdk::message::Message;
// use hex::FromHex;
// use hex::decode;
use std::net::UdpSocket;
use std::str;
use solana_sdk::nonce::state::Data;
use solana_entry;
use tokio;
use solana_sdk::{transaction::Transaction, packet};
use bincode::deserialize;
use solana_entry::entry::Entry;
use solana_ledger::shred::{Shred, Error as ShredError};

pub fn main() {


        // Bind to the local address and port (e.g., "127.0.0.1:34254")
        let socket = match UdpSocket::bind("127.0.0.1:2002") {

            Ok(socket) => socket,
            Err(e) => {
                eprintln!("Error binding to local address: {}", e);
                return;
            }

        };
    
        println!("Listening on port 2002...");
    
        // Buffer to store the incoming data
        let mut buf = [0; 2048];
    
        loop {
            // Receive data from the socket (blocking call)
            match socket.recv_from(&mut buf) {
                Ok((number_of_bytes, src_addr)) => {
                    // Convert the received bytes to a string and print it
                    let received_data = str::from_utf8(&buf[..number_of_bytes])
                        .unwrap_or("<invalid UTF-8>");
                    
                    println!("Received from {}: {}", src_addr, received_data);

                    let result: Result<solana_ledger::shred::Shred, solana_ledger::shred::Error> =  Shred::new_from_serialized_shred(buf.to_vec()); 
                    
                    match result {
                        Ok(shred) => {
                            
                            println!("Shred: {:?}", shred);
                            println!("Signature: {} ", shred.signature());

                            

                            println!("data_complete: {:?}", shred.data_complete());

                            let transaction  = deserialize::<Transaction>(&shred.payload());
                            let entries:Result<Entry, Box<bincode::ErrorKind>>  = deserialize::<Entry>(&shred.payload());

                            println!("Transaction: {:?}", transaction);
                            println!("Entries: {:?}", entries);

                            //solana_ledger::shred::Shredder::deshred(shreds)
                            match shred.shred_type() {
                                solana_ledger::shred::ShredType::Data => {
                                    println!("Shred Type: Data");
                                    println!("Payload: {:?}", shred.payload());

                                    
                                }
                                solana_ledger::shred::ShredType::Code => {
                                    println!("Shred Type: Code");
                                }
                                // solana_ledger::shred::ShredType::LastInSlot => {
                                //     println!("Shred Type: LastInSlot");
                                // }
                                // solana_ledger::shred::ShredType::FirstInSlot => {
                                //     println!("Shred Type: FirstInSlot");
                                // }
                                // solana_ledger::shred::ShredType::LastInFECSet => {
                                //     println!("Shred Type: LastInFECSet");
                                // }
                                // solana_ledger::shred::ShredType::IntermediateInFECSet => {
                                //     println!("Shred Type: IntermediateInFECSet");
                                // }
                                // solana_ledger::shred::ShredType::DataProof => {
                                //     println!("Shred Type: DataProof");
                                // }
                                // solana_ledger::shred::ShredType::CodingProof => {
                                //     println!("Shred Type: CodingProof");
                                // }
                                // solana_ledger::shred::ShredType::Orphan => {
                                //     println!("Shred Type: Orphan");
                                // }



                            }


                        }
                        Err(e) => {
                            eprintln!("Error parsing shred: {}", e);
                        }
                    }

                    let signature = &buf[..64]; // Equivalent to data[:64]

                    // Shred variant (u8)
                    let shred_variant = buf[64]; // Equivalent to data[64]
                
                    // Slot (u64, from bytes 65 to 72)
                    let slot = u64::from_le_bytes(buf[65..73].try_into().unwrap()); // Equivalent to int.from_bytes(data[65:73], 'little')
                
                    // Index (u32, from bytes 73 to 76)
                    let index = u32::from_le_bytes(buf[73..77].try_into().unwrap()); // Equivalent to int.from_bytes(data[73:77], 'little')
                
                    // Version (u16, from bytes 77 to 78)
                    let version = u16::from_le_bytes(buf[77..79].try_into().unwrap()); // Equivalent to int.from_bytes(data[77:79], 'little')
                
                    // FEC set index (u32, from bytes 79 to 82)
                    let fec_set_index = u32::from_le_bytes(buf[79..83].try_into().unwrap()); // Equivalent to int.from_bytes(data[79:83], 'little')

                    println!("Signature: {:?}", signature);
                    println!("Shred Variant: {}", shred_variant);
                    println!("Slot: {}", slot);
                    println!("Index: {}", index);
                    println!("Version: {}", version);
                    println!("FEC Set Index: {}", fec_set_index);
                }
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                    break; // Optionally break the loop if there's an error
                }
            }
        }
    

    //let service = ShredstreamService::default();
    //let service = ShredstreamService::default();
    //shredstream_lister::shredstream::shredstream_server::ShredstreamServer::new(Arc::new(T));

        

    //let signature = Signature::from(&signature_bytes[..]);
    //let signature = Signature::from(&signature_bytes[..]);

    //let signature = Signature::from_bytes(&signature_bytes).unwrap();

    //println!(signature);
    // shredstream_lister::packet::PacketBatch::decode(buf2);

}