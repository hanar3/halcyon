use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 100];
    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                return ();
            }

            //   let command = &data[0..2];
            //   match command {
            //     _ => {
            //       println!("Packet unhandled {:x?}...Shutting down", data);
            //       stream.shutdown(Shutdown::Both).unwrap();
            //     }
            //   }

            true
        }
        Err(_) => {
            println!(
                "An error ocurred, terminating connection with {}",
                stream.peer_addr().unwrap()
            );
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:6121").unwrap();
    println!("Server listening on port 6121");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}
