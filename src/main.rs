use gilrs::{Button, EventType, Gilrs};
use gilrs::ev::Axis::LeftStickX;
use gilrs_core::{EvCode};
use gilrs::ev::Code;
use url::Url;

use std::net::TcpListener;
use std::thread::spawn;

use tungstenite::{accept_hdr, connect, Message};
use tungstenite::handshake::server::{Request, Response};

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
    // loop {
    //     let msg = socket.read_message().expect("Error reading message");
    //     println!("Received: {}", msg);
    // }
    // socket.close(None);


    let mut gilrs = Gilrs::new().unwrap();

    loop {
        while let Some(ev) = gilrs.next_event() {

            let info = gilrs.gamepad(ev.id);

            match ev.event {
                EventType::ButtonPressed(Button::South, _) => socket
                    .write_message(Message::Text(format!("{:?}", ev.event).into()))
                    .unwrap(),
                EventType::ButtonPressed(Button::North, _) => socket
                    .write_message(Message::Text(format!("{:?}", ev.event).into()))
                    .unwrap(),
                EventType::ButtonPressed(Button::West, _) => socket
                    .write_message(Message::Text(format!("{:?}", ev.event).into()))
                    .unwrap(),
                EventType::ButtonPressed(Button::East, _) => socket
                    .write_message(Message::Text(format!("{:?}", ev.event).into()))
                    .unwrap(),
                EventType::ButtonReleased(Button::South, _) => socket
                    .write_message(Message::Text(format!("{:?}", ev.event).into()))
                    .unwrap(),
                EventType::ButtonReleased(Button::North, _) => socket
                    .write_message(Message::Text(format!("{:?}", ev.event).into()))
                    .unwrap(),
                EventType::ButtonReleased(Button::West, _) => socket
                    .write_message(Message::Text(format!("{:?}", ev.event).into()))
                    .unwrap(),
                EventType::ButtonReleased(Button::East, _) => socket
                    .write_message(Message::Text(format!("{:?}", ev.event).into()))
                    .unwrap(),
                EventType::Connected => println!("Connected: {}", format!("{:?}", info.id())),
                EventType::Disconnected => println!("Disconnected: {}", format!("{:?}", info.id())),
                _ => (),
            }


        }
    }


}
