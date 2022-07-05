use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

//Below routine reference tcp.rs.
fn main() {
    //Creating a `TcpListener` by [`bind`]ing it to a socket address and a port.
    //This demo use addr 127.0.0.1 and port 941
    let listener = TcpListener::bind("127.0.0.1:1941").unwrap();
    match listener.accept() {
        //Print the addr of the client.
        Ok((_socket, addr)) => println!("new client: {addr:?}"),
        //Print error info if some errors occurs.
        Err(e) => println!("couldn't get client: {e:?}"),
    }

    // loop {
        //iterator over the connections being received on this listener
        for stream in listener.incoming() {
            //Get the stream from the enum.
            let stream = stream.unwrap();
            //Pass the stream to sub-routine.
            handle_connection(stream);
        }
    // }
}

fn handle_connection(mut stream: TcpStream) {
    //Create a buffer to receive the data
    let mut buffer = [0; 1024];

    loop {
        //Check the read result
        match stream.read(&mut buffer) {
            //if return result is Ok and the size > 0, print the data and send back to the client
            Ok(rec_size) if rec_size > 0 => {
                //Send back to the client
                stream.write(&buffer[0..rec_size]).unwrap();
                //Print the data in local terminal for debugging
                println!("The data from client: {}", String::from_utf8_lossy(&buffer[0..rec_size]));
            }
            //If size = 0, no action.
            Ok(_) => {}
            //Handle the error.
            Err(_) => {
                //Print the error addr.
                println!(
                    "An error occurred, terminating connection with {}",
                    stream.peer_addr().unwrap()
                );
            }
        }
    }
}
