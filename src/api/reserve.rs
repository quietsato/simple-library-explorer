use anyhow::Result;

pub fn reserve() -> Result<String> {
    let client = reqwest::blocking::Client::new().get("").send()?;
    Ok(client.text()?)
}
