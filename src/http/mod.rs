use std::collections::HashMap;

use crate::Result;

pub async fn send(params: HashMap<String, String>) -> Result<()> {
    let request_params: Vec<(String, String)> = params.into_iter().collect();

    reqwest::Client::new()
        .post("https://api.pushover.net/1/messages.json")
        .form(&request_params)
        .send()
        .await?
        .error_for_status()?;

    Ok(())
}
