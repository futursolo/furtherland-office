use yew::prelude::*;

use crate::manifest::Manifest;

mod signin;

#[derive(Clone, Properties, Debug, PartialEq)]
pub(crate) struct AppProps {
    pub manifest: Manifest,
}

impl From<Manifest> for AppProps {
    fn from(manifest: Manifest) -> AppProps {
        AppProps { manifest }
    }
}

pub(crate) use signin::SigninApp;
