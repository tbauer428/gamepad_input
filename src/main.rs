use gilrs::{Button, EventType, Gilrs, Event};
use gilrs::ev::Axis::LeftStickX;
use gilrs_core::{EvCode};
use gilrs::ev::Code;
use url::Url;

use std::net::TcpListener;
use std::thread::spawn;

use tungstenite::{accept_hdr, connect, Message, WebSocket};
use tungstenite::handshake::server::{Request, Response};
use std::thread;
use std::time::Duration;
use std::borrow::{BorrowMut, Borrow};
use tungstenite::client::AutoStream;

fn main() {

    let (mut socket, response) =
        connect(Url::parse("ws://192.168.1.56:8080").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }

    socket
        .write_message(Message::Text("---gamepad_input connected---".into()))
        .unwrap();

    // socket.close(None);


    let mut gilrs = Gilrs::new().unwrap();

    loop {
        while let Some(ev) = gilrs.next_event() {

            let info = gilrs.gamepad(ev.id);

            match ev.event {
                EventType::ButtonPressed(Button::South, _) => send_message(ev.event, &mut socket),
                EventType::ButtonPressed(Button::North, _) => send_message(ev.event, &mut socket),
                EventType::ButtonPressed(Button::West, _) => send_message(ev.event, &mut socket),
                EventType::ButtonPressed(Button::East, _) => send_message(ev.event, &mut socket),
                EventType::ButtonReleased(Button::South, _) => send_message(ev.event, &mut socket),
                EventType::ButtonReleased(Button::North, _) => send_message(ev.event, &mut socket),
                EventType::ButtonReleased(Button::West, _) => send_message(ev.event, &mut socket),
                EventType::ButtonReleased(Button::East, _) => send_message(ev.event, &mut socket),
                EventType::Connected => println!("Connected: {}", format!("{:?}", info.id())),
                EventType::Disconnected => println!("Disconnected: {}", format!("{:?}", info.id())),
                _ => (),
            }


        }
    }


}

fn send_message(ev: EventType, sock: &mut WebSocket<AutoStream>) {
    sock
        .write_message(Message::Text(format!("{:?}", ev).into()))
        .unwrap();
}

fn start_message_listener(sock: &'static mut WebSocket<AutoStream>) {
    thread::spawn(move || {
        loop {
            let msg = sock.read_message().expect("Error reading message");
            println!("Received: {}", msg);
            thread::sleep(Duration::from_millis(1));
        }
    });

}
