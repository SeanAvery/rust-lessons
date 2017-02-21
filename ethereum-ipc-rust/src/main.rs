extern crate nanomsg;

use nanomsg::{Socket, Protocol, Error};

fn create_socket() -> Result<(), Error> {
    let mut socket = try!(Socket::new(Protocol::Pull));

    let mut endpoint = try!(socket.bind("ipc:///tmp/pipeline.ipc"));

    let mut msg = String::new();
    loop {
        try!(socket.read_to_string(&mut msg));
        println!("We got a message: {}", &*msg);
        msg.clear();
    }

    Ok(())
}



fn pusher() -> Result<(), Error> {
    let mut socket = try!(Socket::new(Protocol::Push));
    let mut endpoint = try!(Socket.connect("ipc:///tmp/pipeline.ipc"));

    socket.write(b"messag in a bottle...");

    endpoint.shutdown();
    Ok(())
}

fn main() {
    println!("Hello, world!");
}
