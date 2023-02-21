use serde_json::{self, Value};
use std::error::Error;
pub fn parse(data: String) -> Result<Vec<Value>, Box<dyn Error>> {
    let v: Value = serde_json::from_str(&data)?;
    let types = v.get("types").ok_or("No types provided")?.to_owned();
    let endpints = v
        .get("endpoints")
        .ok_or("No endpoints provided")?
        .to_owned();
    Ok(vec![endpints, types])
}
