use std::borrow::Cow;

use reqwest::Url;
use serde::Deserialize;

use crate::helpers::window;
use crate::Result;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub(crate) struct Manifest {
    pub site_title: String,
    pub endpoint: String,
    pub asset_base_url: String,
}

impl Manifest {
    pub(crate) async fn fetch() -> Result<Self> {
        let href = window().location().href().expect("href is not available.");

        let base_url = reqwest::Url::parse(&href).expect("href is not valid.");
        let url = base_url.join("manifest.toml").unwrap();

        let manifest_str = reqwest::get(url).await?.text().await?;

        Ok(toml::from_str(manifest_str.as_str())?)
    }

    pub fn get_asset_url<S: AsRef<str>>(&self, partial: S) -> Result<String> {
        let url_base: Cow<'_, str> = if !self.asset_base_url.ends_with('/') {
            format!("{}/", self.asset_base_url).into()
        } else {
            self.asset_base_url.as_str().into()
        };

        let url = Url::parse(url_base.as_ref())?;

        Ok(url.join(partial.as_ref())?.into_string())
    }
}
