use gilrs::{Button, EventType, Gilrs, Event};
use gilrs::ev::Axis::LeftStickX;
use gilrs_core::{EvCode};
use gilrs::ev::Code;
use url::Url;

use tungstenite::{connect, Message, WebSocket};
use std::thread;
use std::time::Duration;
use tungstenite::client::AutoStream;
use std::thread::JoinHandle;

fn main() {

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:8080").unwrap()).expect("Can't connect");

    println!("Connected to the server");
    println!("Response HTTP code: {}", response.status());
    println!("Response contains the following headers:");
    for (ref header, _value) in response.headers() {
        println!("* {}", header);
    }
    println!("---------------------------");


    socket
        .write_message(Message::Text("---gamepad_input.rs connected---".into()))
        .unwrap();

    // socket.close(None);


    let mut gilrs = Gilrs::new().unwrap();

    // start_message_listener(&mut socket);

    loop {

        while let Some(ev) = gilrs.next_event() {

            let info = gilrs.gamepad(ev.id);

            match ev.event {
                // EventType::ButtonPressed(Button::South, _) =>
                //     send_message(Option::from(ev.event), &mut socket, None),
                // EventType::ButtonPressed(Button::North, _) =>
                //     send_message(Option::from(ev.event), &mut socket, None),
                EventType::ButtonPressed(Button::West, _) =>
                    send_message(None, &mut socket, Option::from("ROTATE_RIGHT")),
                EventType::ButtonPressed(Button::East, _) =>
                    send_message(None, &mut socket, Option::from("ROTATE_LEFT")),
                // EventType::ButtonReleased(Button::South, _) =>
                //     send_message(Option::from(ev.event), &mut socket, None),
                // EventType::ButtonReleased(Button::North, _) =>
                //     send_message(Option::from(ev.event), &mut socket, None),
                // EventType::ButtonReleased(Button::West, _) =>
                //     send_message(Option::from(ev.event), &mut socket, None),
                // EventType::ButtonReleased(Button::East, _) =>
                //     send_message(Option::from(ev.event), &mut socket, None),
                EventType::Connected => println!("Connected: {}", format!("{:?}", info.id())),
                EventType::Disconnected => println!("Disconnected: {}", format!("{:?}", info.id())),
                _ => (),
            }


        }
    }


}

fn send_message(ev: Option<EventType>, sock: &mut WebSocket<AutoStream>, message: Option<&str>) {

    if ev.is_some() {
        sock
            .write_message(Message::Text(format!("{:?}", ev.expect("EventType was not valid!")).into()))
            .unwrap();
        println!("Sent: {}", format!("{:?}", ev.expect("EventType was not valid!")))
    } else if ev.is_none() && message.is_some() {
        sock
            .write_message(Message::Text(format!("{}", message.expect("Message was not valid!"))))
            .unwrap();
        println!("Sent: {}", message.expect("Message was not valid!"))
    }


}

//WIP separate thread for receiving and logging things
fn start_message_listener(sock: &'static mut WebSocket<AutoStream>) {
    thread::spawn(move || {
        loop {
            let msg = sock.read_message().expect("Error reading message");
            println!("Received: {}", msg);
            send_message(None, sock, Option::from("I got your message too!"));
            thread::sleep(Duration::from_millis(3000));
        }
    });
}
