use std::cell::RefCell;
use std::convert::TryInto;
use std::ops::Deref;
use std::rc::Rc;
use std::time::Duration;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{Document, Window};
use yew::html::{Component, ComponentLink};

use once_cell::sync::Lazy;
use regex::Regex;

pub(crate) use crate::wasm_instant::Instant;

pub(crate) fn window() -> Window {
    web_sys::window().unwrap()
}

pub(crate) fn document() -> Document {
    window().document().unwrap()
}

pub(crate) fn is_mobile_safari() -> bool {
    static IOS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)iP(ad|od|hone)").unwrap());
    static WEBKIT_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)WebKit").unwrap());
    static CRIOS_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)CriOS").unwrap());

    let user_agent = window().navigator().user_agent().unwrap();

    let is_ios = IOS_RE.is_match(&user_agent);
    if is_ios {
        log::debug!("I am running on iOS.");
    } else {
        log::debug!("I am not running on iOS.");
    }

    let is_webkit = WEBKIT_RE.is_match(&user_agent);
    if is_webkit {
        log::debug!("I am running on WebKit.");
    } else {
        log::debug!("I am not running on WebKit.");
    }

    let is_crios = CRIOS_RE.is_match(&user_agent);
    if is_ios {
        log::debug!("I am running on iOS Chrome.");
    } else {
        log::debug!("I am not running on iOS Chrome.");
    }

    is_ios && is_webkit && !is_crios
}

pub(crate) async fn sleep(dur: Duration) {
    let (s, r) = async_channel::unbounded::<()>();

    call_after(
        move || {
            s.close();
        },
        dur,
    );
    r.recv().await.unwrap_err();
}

pub(crate) fn call_after<F>(f: F, dur: Duration)
where
    F: 'static + FnOnce() + Fn(),
{
    let callback = Closure::once_into_js(f);
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            callback.unchecked_ref(),
            dur.as_millis().try_into().unwrap(),
        )
        .unwrap();
}

#[derive(Debug)]
pub struct WeakComponentLink<COMP: Component>(Rc<RefCell<Option<ComponentLink<COMP>>>>);

impl<COMP: Component> Clone for WeakComponentLink<COMP> {
    fn clone(&self) -> Self {
        Self(Rc::clone(&self.0))
    }
}

impl<COMP: Component> Default for WeakComponentLink<COMP> {
    fn default() -> Self {
        Self(Rc::default())
    }
}

impl<COMP: Component> Deref for WeakComponentLink<COMP> {
    type Target = Rc<RefCell<Option<ComponentLink<COMP>>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<COMP: Component> PartialEq for WeakComponentLink<COMP> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0, &other.0)
    }
}
