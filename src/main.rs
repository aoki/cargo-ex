//! # cargo ex
//! This cargo sub-command execute `cargo run --example` easily via fuzzy finder.

use anyhow::{anyhow, bail, Context};
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

fn fuzzy_find(examples: Vec<String>) -> anyhow::Result<String> {
    // 2 is count line and search input line's height
    let size = (examples.len() + 2).to_string();
    let skim_options = SkimOptionsBuilder::default()
        .min_height(Some(&size))
        .height(Some(&size))
        .build()
        .map_err(|e| anyhow!(e))?;
    let examples_string = examples.join("\n");
    let item_reader = SkimItemReader::default().of_bufread(Cursor::new(examples_string));
    let selected_items = Skim::run_with(&skim_options, Some(item_reader))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

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

fn get_files(path: String) -> anyhow::Result<Vec<String>> {
    let target_dir = fs::canonicalize(PathBuf::from(&path))
        .with_context(|| format!("Couldn't canonicalize: {}", &path))?;
    let target_dir =
        read_dir(target_dir).with_context(|| format!("Couldn't read directory: {path}"))?;

    let (entries, errors): (Vec<_>, Vec<_>) = target_dir.into_iter().partition(Result::is_ok);
    let entries: Vec<_> = entries.into_iter().map(Result::unwrap).collect();
    let errors: Vec<_> = errors.into_iter().map(Result::unwrap_err).collect();
    if !errors.is_empty() {
        bail!("Couldn't read directory: {:?}", errors);
    }

    let mut examples: Vec<_> = entries
        .iter()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect();
    examples.sort();

    Ok(examples)
}

fn main() -> anyhow::Result<()> {
    let examples = get_files("./examples".to_string())?;

    let example = fuzzy_find(examples).with_context(|| "Couldn't use fuzzy finder")?;

    Command::new("cargo")
        .arg("run")
        .arg("--example")
        .arg(example.strip_suffix(".rs").unwrap())
        .exec();

    Ok(())
}

#[cfg(test)]
mod test {
    use crate::get_files;

    #[test]
    fn get_target_files() {
        let mut expected = vec![
            "hello-cargo.rs",
            "hello-example.rs",
            "hello-rust.rs",
            "hello-wave.rs",
        ];
        expected.sort();
        let expected = expected
            .into_iter()
            .map(|e| e.to_string())
            .collect::<Vec<String>>();
        let actual = get_files("./examples".to_string()).unwrap();
        assert_eq!(actual, expected);
    }
}
