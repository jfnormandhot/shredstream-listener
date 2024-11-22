use std::net::{SocketAddr};
//use std::time::Duration;
use solana_streamer::{
    sendmmsg::{batch_send, SendPktsError, },
    streamer,
    streamer::StreamerReceiveStats,
};

use shredstream_lister::socket::SocketAddrSpace;
use shredstream_lister::recvmmsg::{recv_mmsg, NUM_RCVMMSGS};

use {
    // crate::{
    //     recvmmsg::{recv_mmsg, NUM_RCVMMSGS},
    //     socket::SocketAddrSpace,
    // },
    std::{
        io::Result,
        net::UdpSocket,
        time::{Duration, Instant},
    },
};


pub use {
    solana_perf::packet::{
        to_packet_batches, PacketBatch, PacketBatchRecycler, NUM_PACKETS, PACKETS_PER_BATCH,
    },
    solana_sdk::packet::{Meta, Packet, PACKET_DATA_SIZE},
};

//use solana_sdk::packet::{Meta, Packet, PACKET_DATA_SIZE};

// pub mod shred {
//     include!("../src/trace_shred.rs");
// }

pub fn recv_from(batch: &mut PacketBatch, socket: &UdpSocket, max_wait: Duration) -> Result<usize> {
    let mut i = 0;
    //DOCUMENTED SIDE-EFFECT
    //Performance out of the IO without poll
    //  * block on the socket until it's readable
    //  * set the socket to non blocking
    //  * read until it fails
    //  * set it back to blocking before returning
    socket.set_nonblocking(false)?;
    println!("receiving on {}", socket.local_addr().unwrap());
    let start = Instant::now();
    loop {
        batch.resize(
            std::cmp::min(i + NUM_RCVMMSGS, PACKETS_PER_BATCH),
            Packet::default(),
        );
        match recv_mmsg(socket, &mut batch[i..]) {
            Err(_) if i > 0 => {
                if start.elapsed() > max_wait {
                    break;
                }
            }
            Err(e) => {
                println!("recv_from err {:?}", e);
                return Err(e);
            }
            Ok(npkts) => {
                if i == 0 {
                    socket.set_nonblocking(true)?;
                }
                println!("got 111 {} packets", npkts);
                println!("{:?}", npkts);
                i += npkts;
                // Try to batch into big enough buffers
                // will cause less re-shuffling later on.
                if start.elapsed() > max_wait || i >= PACKETS_PER_BATCH {
                    break;
                }
            }
        }
    }
    batch.truncate(i);
    Ok(i)
}

fn main() -> std::io::Result<()> {


    let recv_socket = UdpSocket::bind("127.0.0.1:2002").expect("bind");
    let addr = recv_socket.local_addr().unwrap();
    let send_socket = UdpSocket::bind("127.0.0.1:0").expect("bind");
    let saddr = send_socket.local_addr().unwrap();

    let packet_batch_size = 30;
    let mut batch = PacketBatch::with_capacity(packet_batch_size);
    batch.resize(packet_batch_size, Packet::default());

    for m in batch.iter_mut() {
        m.meta_mut().set_socket_addr(&addr);
        m.meta_mut().size = PACKET_DATA_SIZE;
    }
    //send_to(&batch, &send_socket, &SocketAddrSpace::Unspecified).unwrap();

    batch
        .iter_mut()
        .for_each(|pkt| *pkt.meta_mut() = Meta::default());
    let recvd = recv_from(
        &mut batch,
        &recv_socket,
        Duration::from_millis(1000000), // max_wait
    )
    .unwrap();

    
    assert_eq!(recvd, batch.len());


    // let address: SocketAddr = "127.0.0.1:2002".parse().unwrap();
    // let socket = UdpSocket::bind(address)?;

    // // Set socket to non-blocking mode
    // socket.set_nonblocking(true)?;

    // let mut buf = [0u8; 1024];

    // loop {
    //     match socket.recv_from(&mut buf) {
    //         Ok((amt, src)) => {
    //             println!("Received {} bytes from {}: {:?}", amt, src, &buf[..amt]);
    //         }

            

    //         // shred::TraceShred::decode(&mut buf) => {
    //         //     println!("Received a TraceShred: {:?}", shred::TraceShred::decode(&mut buf));
    //         // }

    //         // let mut packets = vec![Packet::default(); 32];
    //         // let recv = recv_mmsg(reader, &mut packets[..]).await.unwrap();

    //         Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
    //             // No data available yet
    //             println!("No data yet...");
    //             std::thread::sleep(Duration::from_secs(1));
    //         }
    //         Err(e) => {
    //             println!("Error: {}", e);
    //             break;
    //         }
    //     }
    // }

    Ok(())

    
}

pub fn send_to(
    batch: &PacketBatch,
    socket: &UdpSocket,
    socket_addr_space: &SocketAddrSpace,
) -> Result<()> {
    for p in batch.iter() {
        let addr = p.meta().socket_addr();
        if socket_addr_space.check(&addr) {
            if let Some(data) = p.data(..) {
                socket.send_to(data, addr)?;
            }
        }
    }
    Ok(())
}
