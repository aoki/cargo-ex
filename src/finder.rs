fn finder() -> anyhow::Result<()> {
    let skim_options = SkimOptionsBuilder::default()
        .height(Some("50%"))
        .build()
        .map_err(|e| anyhow!(e))?;
}
