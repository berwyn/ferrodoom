use std::io::Read;

use wad::WadParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();
    let path: String = args.value_from_str(["-f", "--file"]).unwrap();

    println!("Loading WAD from {:?}", path);
    let mut file = std::fs::File::open(&path)?;
    let mut bytes: Vec<u8> = Vec::new();

    file.read_to_end(&mut bytes)?;

    let wad = WadParser::default().with_data(&bytes).parse()?;
    std::dbg!(wad);

    Ok(())
}
