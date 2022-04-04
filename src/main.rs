use anyhow::anyhow;
use skim::{
    prelude::{SkimItemReader, SkimOptionsBuilder},
    Skim,
};
use std::{
    fs::{self, read_dir},
    io::Cursor,
    os::unix::prelude::CommandExt,
    path::PathBuf,
    process::Command,
};

fn read_examples() -> anyhow::Result<()> {
    Ok(())
}

pub fn fuzzy_find(examples: Vec<String>) -> anyhow::Result<String> {
    let skim_options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .map_err(|e| anyhow!(e))?;
    let examples_string = examples.join("\n");
    let item_reader = SkimItemReader::default().of_bufread(Cursor::new(examples_string));
    let selected_items = Skim::run_with(&skim_options, Some(item_reader))
        .map(|out| out.selected_items)
        .unwrap_or_else(|| Vec::new());

    let selected = selected_items
        .iter()
        .map(|i| i.output().to_string())
        .collect::<Vec<String>>();

    selected
        .get(0)
        .ok_or("no such an example file.")
        .map_err(|e| anyhow!(e))
        .map(|m| m.clone())
}

fn main() -> anyhow::Result<()> {
    let example_dir = fs::canonicalize(PathBuf::from("./examples"))?;
    println!("Examples directory: {}", &example_dir.to_string_lossy());
    let example_dir = read_dir(example_dir)?;
    let examples = example_dir
        .into_iter()
        .map(|f| f.unwrap().file_name().to_string_lossy().to_string())
        .collect::<Vec<String>>();

    let example = fuzzy_find(examples)?;

    let _ = Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg(example.strip_suffix(".rs").unwrap())
        .exec();

    Ok(())
}
