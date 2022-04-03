use std::{
    ffi::{OsStr, OsString},
    fs::{self, read_dir},
    os::unix::prelude::CommandExt,
    path::{Path, PathBuf},
    process::Command,
};

fn read_examples() -> anyhow::Result<()> {
    Ok(())
}

fn main() -> anyhow::Result<()> {
    let example_dir = fs::canonicalize(PathBuf::from("./examples"))?;
    println!("Examples directory: {}", &example_dir.to_string_lossy());
    let x = read_dir(example_dir)?;
    // x.into_iter().for_each(|f| println!("{:?}", f));
    let z = x
        .into_iter()
        .map(|f| {
            let file = f.unwrap().file_name();
            file
        })
        .collect::<Vec<OsString>>();

    // let cmd = Command::new("cargo").arg("run --example").exec();
    println!("{:?}", z[0]);
    let cmd = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg(
            z[1].to_string_lossy()
                .to_string()
                .strip_suffix(".rs")
                .unwrap(),
        )
        .exec();

    println!("Hello, example world!");
    Ok(())
}
