fn retry() {
    router.route("/win", get(win));

    async fn win() -> Result<Bytes, HttpError> {
        let backoff = ExponentialBackoff::default();
        let response = backoff::future::retry(backoff, || async {
            let url = "https://httpstat.us/402";
            info!("GET {}", url);
            let response = reqwest::get(url).await?.error_for_status()?;
            Ok(response)
        })
        .await?;
        Ok(response.bytes().await?)
    }
}

fn retry_notify() {
    async fn win() -> Result<Bytes, HttpError> {
        let backoff = ExponentialBackoff::default();
        let response = retry_notify(
            backoff,
            || async {
                let url = "https://httpstat.us/402";
                info!("GET {}", url);
                let response = reqwest::get(url).await?.error_for_status()?;
                Ok(response)
            },
            |error, duration| error!("Retrying at {duration:?}: {error}"),
        )
        .await?;
        Ok(response.bytes().await?)
    }
}