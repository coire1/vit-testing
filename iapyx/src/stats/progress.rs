
pub fn read_initials<S: Into<String>>(snapshot: S) -> Result<Vec<Initial>, IapyxStatsCommandError> {
    let snapshot = snapshot.into();
    let value: serde_json::Value = serde_json::from_str(&snapshot)?;
    let initial = serde_json::to_string(&value["initial"])?;
    serde_json::from_str(&initial).map_err(Into::into)
}