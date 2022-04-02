use std::{
    fs::{self, read_dir},
    path::PathBuf,
};

fn main() -> anyhow::Result<()> {
    let example_dir = fs::canonicalize(PathBuf::from("./examples"))?;
    println!("Examples directory: {}", &example_dir.to_string_lossy());
    let x = read_dir(example_dir)?;
    x.into_iter().for_each(|f| println!("{:?}", f));

    println!("Hello, example world!");
    Ok(())
}
