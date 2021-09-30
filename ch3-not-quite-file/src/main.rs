use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}

#[derive(Debug)]
struct File {
    name: String,
    data: Vec<u8>,
}

// Overloading sorta... https://stackoverflow.com/a/67064869/25182
enum NewFileParam<'a> {
    Name(&'a str),
    NameAndData(&'a str, &'a Vec<u8>),
}
impl<'a> From<&'a str> for NewFileParam<'a> {
    fn from(n: &'a str) -> Self {
        NewFileParam::Name(n)
    }
}
impl<'a> From<(&'a str, &'a Vec<u8>)> for NewFileParam<'a> {
    fn from(p: (&'a str, &'a Vec<u8>)) -> Self {
        NewFileParam::NameAndData(p.0, p.1)
    }
}

impl File {
    fn new<'a, T: Into<NewFileParam<'a>>>(t: T) -> File {
        use NewFileParam::*;

        let f = match t.into() {
            Name(n) => File {
                name: String::from(n),
                data: Vec::new(),
            },
            NameAndData(n, d) => File {
                name: String::from(n),
                data: d.clone(),
            },
        };

        f
    }

    fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        let mut tmp = self.data.clone();
        let read_length = tmp.len();

        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    Ok(f)
}

fn close(f: File) -> Result<File, String> {
    if one_in(100_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg);
    }
    Ok(f)
}

fn main() {
    let f4_data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f4 = File::new(("f4.txt", &f4_data));
    //let mut f4 = File::new("f4.txt");

    let mut buffer: Vec<u8> = vec![];

    f4 = open(f4).unwrap();
    let f4_length = f4.read(&mut buffer).unwrap();
    f4 = close(f4).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f4);
    println!("{} is {} bytes long", &f4.name, f4_length);
    println!("{}", text);
}
