#![allow(unused_variables)]

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File) -> bool {
    true
}

fn close(f: &mut File) -> bool {
    true
}

#[allow(dead_code)]
fn read(f: &mut File, save_to: &mut Vec<u8>) -> ! {
    unimplemented!()
}

fn main() {
    let mut f1 = File {
        name: String::from("f1.txt"),
        data: Vec::new(),
    };
    open(&mut f1);

    let f1_name = &f1.name;
    let f1_length = &f1.data.len();

    //read(&mut f1, &mut vec![]);
    close(&mut f1);

    println!("{:?}", f1);
    println!("{} is {} bytes long", f1_name, f1_length);
}
