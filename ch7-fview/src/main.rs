use std::env;
use std::fs::File;
use std::io::Read;

const BYTES_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
    let arg1 = env::args().nth(1);

    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = f.read_exact(&mut buffer) {
        print!("[0x{:08x}] ", pos);
        for byte in &buffer {
            print!("{:02x} ", byte);
        }
        print!("  ");
        for byte in &buffer {
            match *byte {
                0x00 => print!("0"),
                0x20..=0x7E => print!("{}", *byte as char),
                0xff => print!("#"),
                _ => print!("."),
            }
        }

        println!();
        pos += BYTES_PER_LINE;
    }

    Ok(())
}
