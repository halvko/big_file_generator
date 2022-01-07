use std::fs::OpenOptions;

const FILE_NAME: &str = "BIG";

const GI_B: u64 = 1024 * 1024 * 1024;

fn main() -> anyhow::Result<()> {
    let buffer = OpenOptions::new()
        .create(true)
        .write(true)
        .open(FILE_NAME)
        .unwrap();
    let size = buffer.metadata().unwrap().len();
    let new_size = GI_B.max(size * 2);
    buffer.set_len(new_size).unwrap();
    Ok(())
}
