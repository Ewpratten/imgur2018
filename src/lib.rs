use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestErr(#[from] reqwest::Error),

    #[error(transparent)]
    UrlParseErr(#[from] url::ParseError),

    #[error("Failed to parse imgur's json data")]
    JsonLinkParseErr,
}

/// Upload some data to imgur
pub async fn imgur_upload(client_id: &str, data: Vec<u8>) -> Result<url::Url, Error> {
    // Make post request to imgur
    let response = reqwest::ClientBuilder::default()
        .build()?
        .post("https://api.imgur.com/3/image")
        .header("Authorization", format!("Client-ID {}", client_id))
        .body(data)
        .send()
        .await?;

    // Parse response
    let response_json: serde_json::Value = response.json().await?;
    Ok(url::Url::parse(
        response_json["data"]["link"]
            .as_str()
            .ok_or(Error::JsonLinkParseErr)?,
    )?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_imgur_upload() {
        tokio_test::block_on(async {
            let url = imgur_upload(
                "332741bbdcde865",
                reqwest::get("https://www.rust-lang.org/logos/rust-logo-128x128.png")
                    .await
                    .unwrap()
                    .bytes()
                    .await
                    .unwrap()
                    .iter()
                    .map(|x| *x)
                    .collect(),
            )
            .await
            .unwrap();
            println!("{:?}", url.to_string());
        });
    }
}
