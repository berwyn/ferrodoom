use std::io::Read;

use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use wad::WadParser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = pico_args::Arguments::from_env();
    let path: String = args.value_from_str(["-f", "--file"]).unwrap();
    let name: String = args.value_from_str(["-n", "--name"]).unwrap();

    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Couldn't setup tracing!");

    println!("Loading WAD from {:?}", path);
    let mut file = std::fs::File::open(&path)?;
    let mut bytes: Vec<u8> = Vec::new();

    file.read_to_end(&mut bytes)?;

    let wad = WadParser::default().with_data(&bytes).parse()?;
    match wad.find(&name) {
        Some(lump) => {
            std::dbg!(lump);
        }
        None => {
            println!("{} couldn't be found!", name);
            std::process::exit(1);
        }
    }

    Ok(())
}
