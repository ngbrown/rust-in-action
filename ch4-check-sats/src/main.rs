#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
struct CubeSat {
    id: u64,
}

#[derive(Debug)]
struct Mailbox {
    messages: Vec<Message>,
}

#[derive(Debug)]
struct Message {
    to: u64,
    content: String,
}

#[derive(Debug)]
struct GroundStation {
    radio_freq: f64, // Mhz
}

impl GroundStation {
    fn connect(&self, sat_id: u64) -> CubeSat {
        CubeSat { id: sat_id }
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message) {
        mailbox.post(msg);
    }
}

impl CubeSat {
    fn recv(&self, mailbox: &mut Mailbox) -> Option<Message> {
        mailbox.deliver(&self)
    }
}

impl Mailbox {
    fn post(&mut self, msg: Message) {
        self.messages.push(msg);
    }

    fn deliver(&mut self, recipient: &CubeSat) -> Option<Message> {
        for i in 0..self.messages.len() {
            if self.messages[i].to == recipient.id {
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeSat) -> StatusMessage {
    StatusMessage::Ok
}

fn fetch_sat_ids() -> Vec<u64> {
    vec![1, 2, 3]
}

fn main() {
    let mut mail = Mailbox { messages: vec![] };

    let base: Rc<RefCell<GroundStation>> =
        Rc::new(RefCell::new(GroundStation { radio_freq: 87.65 }));

    println!("base: {:?}", base);

    {
        let mut base_2 = base.borrow_mut();
        base_2.radio_freq -= 12.34;
        println!("base_2: {:?}", base_2);
    }

    println!("base: {:?}", base);

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.borrow().connect(sat_id);

        let status = check_status(sat.clone());
        println!("{:?}: {:?}", sat, status.clone());

        let msg = Message {
            to: sat_id,
            content: String::from("hello"),
        };
        base.borrow().send(&mut mail, msg);
    }

    let sat_ids = fetch_sat_ids();

    for sat_id in sat_ids {
        let sat = base.borrow().connect(sat_id);

        let status = check_status(sat);
        println!("{:?}: {:?}", sat, status);

        let msg = sat.recv(&mut mail);
        println!("{:?}: {:?}", sat, msg);
    }
}
