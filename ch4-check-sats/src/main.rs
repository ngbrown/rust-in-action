#![allow(unused_variables)]

#[derive(Debug)]
struct CubeSat {
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

type Message = String;

struct GroundStation;

impl GroundStation {
    fn send(&self, to: &mut CubeSat, msg: Message) {
        to.mailbox.messages.push(msg);
    }
}

impl CubeSat {
    fn recv(&mut self) -> Option<Message> {
        self.mailbox.messages.pop()
    }
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: &CubeSat) {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
}

fn main() {
    let base = GroundStation {};

    let mut sat_a = CubeSat {
        id: 0,
        mailbox: Mailbox { messages: vec![] },
    };
    let sat_b = CubeSat {
        id: 1,
        mailbox: Mailbox { messages: vec![] },
    };
    let sat_c = CubeSat {
        id: 2,
        mailbox: Mailbox { messages: vec![] },
    };

    check_status(&sat_a);

    base.send(&mut sat_a, Message::from("hello there!"));

    check_status(&sat_a);

    let msg = sat_a.recv();
    check_status(&sat_a);

    println!("msg: {:?}", msg);
}
