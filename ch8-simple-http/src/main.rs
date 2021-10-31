use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    //let url = "http://www.rustinaction.com/";
    let url = "http://localhost:5000/";
    let mut response = reqwest::get(url)?;

    let content = response.text()?;
    println!("{}", content);

    Ok(())
}
