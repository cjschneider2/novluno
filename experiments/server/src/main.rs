#![allow(dead_code, unused_variables)]

mod crypto;

use std::io::BufReader;
use std::io::prelude::*;
use std::net::Shutdown;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

// const RM_PORT: u16 = 10101;
const CLIENT_LISTEN_ADDR: &'static str = "192.168.56.1:10101";
const SERVER_ADDR: &'static str = "198.24.149.46:10101";
const MAX_MSG_SIZE: usize = 2048;

enum MessageType {
    ReqVersion,
    RspVersion,
}

fn main() {
    let msg = format!("Client listen address `{}` could not be bound", CLIENT_LISTEN_ADDR);
    let listener = TcpListener::bind(CLIENT_LISTEN_ADDR).expect(&msg);

    // NOTE: This iterator will not yield a `None` value so is equivalent to a loop
    println!("listening for connections on `{}`", CLIENT_LISTEN_ADDR);
    for maybe_stream in listener.incoming() {
        match maybe_stream {
            Ok(client_stream) => handle_client(client_stream),
            Err(error) => println!("Client Connection Listener failed with: `{}`", error),
        }
    }
}

fn handle_client(client_stream: TcpStream) {

    println!("got connection from: `{:?}`", client_stream);

    thread::spawn( move || {
        // open up a connection to the (actual) RM server
        println!("trying to connect to server: {:?}", SERVER_ADDR);
        let server_stream = TcpStream::connect(SERVER_ADDR).unwrap();
        server_stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
        let mut server_read = BufReader::new(server_stream.try_clone().unwrap());
        // let mut server_write = BufWriter::new(server_stream.try_clone().unwrap());
        let mut server_write = server_stream.try_clone().unwrap();

        println!("cloning client connection: {:?}", SERVER_ADDR);
        let client_stream = client_stream.try_clone().unwrap();
        client_stream.set_read_timeout(Some(Duration::from_millis(100))).unwrap();
        let mut client_read = BufReader::new(client_stream.try_clone().unwrap());
        // let mut client_write = BufWriter::new(client_stream.try_clone().unwrap());
        let mut client_write = client_stream.try_clone().unwrap();

        let mut client_msg: Vec<u8> = [0u8; MAX_MSG_SIZE].to_vec();
        let mut server_msg: Vec<u8> = [0u8; MAX_MSG_SIZE].to_vec();

        loop {

            // listen to messages from client
            if let Ok(bytes) = client_read.read(&mut client_msg) {
                println!("got client messages");
                if bytes > 0 {
                    assert!(bytes < MAX_MSG_SIZE);
                    println!("got {} bytes", bytes);
                    unsafe { client_msg.set_len(bytes); }
                    println!("client->server   : {:?}", &client_msg);
                    let decrypted = crypto::decrypt(&client_msg);
                    println!("`-> decrypted    : {:?}", decrypted);
                    println!("`-> as string    : {:?}", String::from_utf8_lossy(&decrypted));
                    // println!("`-> as utf16(+0) : {:?}", String::from_utf16_lossy(&decrypted[..]));
                    // println!("`-> as utf16(+1) : {:?}", String::from_utf16_lossy(&decrypted[1..]));
                    if let Some(message) = parse(&decrypted) {
                        println!("`-> as message   : {:?}", message);
                    }

                    // send client message to server
                    server_write.write(&mut client_msg).unwrap();

                    unsafe { client_msg.set_len(MAX_MSG_SIZE); }
                } else {
                    println!("read of 0: client shutdown?");
                    cleanup_streams(client_stream, server_stream);
                    break;
                }
                println!();
            }

            // listen to messages from server
            if let Ok(bytes) = server_read.read(&mut server_msg) {
                println!("got server messages");
                if bytes > 0  {
                    println!("got {} bytes", bytes);
                    unsafe{ server_msg.set_len(bytes); }
                    println!("server->client   : {:?}", &server_msg);
                    let decrypted = crypto::decrypt(&server_msg);
                    println!("`-> decrypted    : {:?}", decrypted);
                    println!("`-> as utf8      : {:?}", String::from_utf8_lossy(&decrypted));
                    // println!("`-> as utf16(+0) : {:?}", String::from_utf16_lossy(&decrypted));
                    // println!("`-> as utf16(+1) : {:?}", String::from_utf16_lossy(&decrypted[1..]));

                    // send server messages to client
                    client_write.write(&mut server_msg).unwrap();

                    unsafe{ server_msg.set_len(MAX_MSG_SIZE); }
                } else {
                    println!("read of 0: server shutdown?");
                    cleanup_streams(client_stream, server_stream);
                    break;
                }
                println!();
            }

            if check_stream_errors(&client_stream, &server_stream).is_some() {
                cleanup_streams(client_stream, server_stream);
                break;
            }
        }

        println!("Ending client connection thread");
    });
}

fn parse(bytes: &[u8]) -> Option<()> {
    None
}

fn check_stream_errors (client: &TcpStream, server: &TcpStream) -> Option<()> {
    if let Err(error) = client.take_error() {
        println!("client error: `{:?}`", error);
        return Some(());
    }
    if let Err(error) = server.take_error() {
        println!("server error: `{:?}`", error);
        return Some(());
    }
    None
}

fn cleanup_streams (client: TcpStream, server: TcpStream) {
    client.shutdown(Shutdown::Both).unwrap();
    server.shutdown(Shutdown::Both).unwrap();
}
