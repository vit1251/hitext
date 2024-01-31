use mio::net::UdpSocket;
use mio::{Events, Interest, Poll, Token};
use std::io::{stdout, Write};
use std::time::Duration;

use hitext::parser::{parse, Msg};

const MONITOR: Token = Token(0);

fn apply(msgs: Vec<Msg>) {

    let stdout = stdout();
    let mut stdout = stdout.lock();

    print!("{}", termion::cursor::Hide);

    for msg in msgs.iter() {
        //println!("msg = {:?}", msg);

        match msg {
            Msg::Reset => {
                print!("{}{}", termion::clear::All, termion::style::Reset);
            }
            Msg::GotoXY(x, y) => {
                print!("{}", termion::cursor::Goto(*x, *y));
            }
            Msg::SetColor(r,g,b) => {
                print!("{}", termion::color::Fg(termion::color::Rgb(*r,*g,*b)));
            }
            Msg::Write(c) => {
                //let v = x as char;
                print!("{}", c);
            }
        }
    }

    stdout.flush().unwrap();
}

fn main() {
    let addr = "0.0.0.0:5000".parse().unwrap();
    let mut monitor_sock = UdpSocket::bind(addr).unwrap();

    let mut poll = Poll::new().unwrap();
    let mut buffer = [0; 65536];

    poll.registry()
        .register(&mut monitor_sock, MONITOR, Interest::READABLE)
        .unwrap();

    let mut events = Events::with_capacity(128);
    loop {
        poll.poll(&mut events, Some(Duration::from_millis(100)))
            .unwrap();
        for event in events.iter() {
            match event.token() {
                MONITOR => {
                    let size = monitor_sock.recv(&mut buffer).unwrap();
                    let msgs = parse(&buffer[0..size]);
                    apply(msgs);
                },
                Token(_) => todo!(),
            }
        }
    }
}
