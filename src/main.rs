use std::env::args_os;
use std::fs::File;
use std::os::windows::prelude::*;

fn main() -> anyhow::Result<()> {
    let input = args_os()
        .skip(1)
        .next()
        .expect("Expected a number describing the size of the wanted file in GiBs!");

    let buffer = File::create("BIG")?;
    buffer.seek_write(
        &[0],
        input
            .to_str()
            .expect("Expected valid utf-8 in the first argument")
            .parse::<u64>()?
            * 1024
            * 1024
            * 1024,
    )?;
    Ok(())
}
