use std::env;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

const BYTES_PER_LINE: usize = 16;

fn main() -> std::io::Result<()> {
    let arg1 = env::args().nth(1);

    let fname = arg1.expect("usage: fview FILENAME");

    let mut f = File::open(&fname).expect("Unable to open file.");
    let mut pos: u64 = 0;
    let mut buffer = [0; BYTES_PER_LINE];

    while let Ok(_) = f.read_exact(&mut buffer) {
        print_hex_dump_line(pos, &buffer);

        pos += BYTES_PER_LINE as u64;
    }

    f.seek(SeekFrom::Start(pos)).unwrap();
    let mut buffer_vec: Vec<u8> = vec![];
    let bytes_to_end = f.read_to_end(&mut buffer_vec).unwrap();
    if bytes_to_end > 0 {
        print_hex_dump_line(pos, &buffer_vec);
        // pos += bytes_to_end;
    }

    Ok(())
}

fn print_hex_dump_line(pos: u64, buffer: &[u8]) {
    print!("[0x{:08x}] ", pos);
    for byte in buffer {
        print!("{:02x} ", byte);
    }
    for _ in buffer.len()..BYTES_PER_LINE {
        print!("   ");
    }
    print!("  ");
    for byte in buffer {
        match *byte {
            0x0A => print!("↵"),
            0x0D => print!("⇽"),
            0x20..=0x7E => print!("{}", *byte as char),
            _ => print!("∙"),
        }
    }

    println!();
}
