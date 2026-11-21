extern crate google_sheets4 as sheets4;
use sheets4::{hyper_util, hyper_rustls, yup_oauth2, Sheets, Result};
use hyper_rustls::HttpsConnector;
use hyper_util::{
    client::legacy::{connect::HttpConnector, Client},
    rt::TokioExecutor
};
use crate::config::{Config};
pub type Hub = Sheets<HttpsConnector<HttpConnector>>;

pub async fn build_hub(conf: &Config) -> Result<Hub> {
    let secret = yup_oauth2::read_service_account_key(&conf.priv_key).await?;
    let auth = yup_oauth2::ServiceAccountAuthenticator::builder(secret)
        .build()
        .await?;
    let connector = hyper_rustls::HttpsConnectorBuilder::new()
    .with_native_roots()?
    .https_only()
    .enable_http1()
    .build();
    let client = Client::builder(TokioExecutor::new()).build(connector);

    Ok(Sheets::new(client, auth))
}

