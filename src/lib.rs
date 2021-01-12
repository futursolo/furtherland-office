#![recursion_limit = "512"]
#![deny(clippy::all)]
use std::result::Result as StdResult;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use log::Level;

mod app_components;
mod apps;
pub mod backend;
pub mod components;
mod error;
mod helpers;
mod manifest;
pub mod traits;
mod wasm_instant;

use apps::{SigninApp, SigninAppProps, SigninSection};
pub use error::{Error, Result};
use manifest::Manifest;

#[wasm_bindgen(start)]
pub fn prepare_office() -> StdResult<(), JsValue> {
    yew::initialize();
    if cfg!(debug_assertions) {
        console_log::init_with_level(Level::Debug).map_err(|e| e.to_string().into())
    } else {
        console_log::init().map_err(|e| e.to_string().into())
    }
}

#[wasm_bindgen(js_name = startOffice)]
pub async fn start_office() -> StdResult<(), JsValue> {
    Ok(())
}

#[wasm_bindgen(js_name = startSignin)]
pub async fn start_signin() -> StdResult<(), JsValue> {
    let manifest: Manifest = Manifest::fetch().await?;
    App::<SigninApp>::new().mount_as_body_with_props(manifest.into());
    Ok(())
}

#[wasm_bindgen(js_name = startSignup)]
pub async fn start_signup() -> StdResult<(), JsValue> {
    let mut props: SigninAppProps = Manifest::fetch().await?.into();
    props.set_first_section(SigninSection::Signup);
    App::<SigninApp>::new().mount_as_body_with_props(props.into());
    Ok(())
}
