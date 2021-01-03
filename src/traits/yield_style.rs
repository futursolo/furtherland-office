use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Mutex;

use css_in_rust::Style;

use once_cell::sync::Lazy;

pub trait YieldStyle {
    fn element_name(&self) -> Cow<'static, str> {
        "unknown-element".into()
    }

    // Update to yield style string
    fn yield_style_str(&self) -> Cow<'static, str>;

    fn yield_style_class(&self) -> String {
        static STYLES: Lazy<Mutex<HashMap<(Cow<'static, str>, Cow<'static, str>), String>>> =
            Lazy::new(|| Mutex::new(HashMap::with_capacity(1)));

        let k = (self.element_name(), self.yield_style_str());
        let mut styles = STYLES.lock().unwrap();

        if let Some(v) = styles.get(&k) {
            v.clone()
        } else {
            let v = Style::create(&*k.0, &*k.1).unwrap().get_class_name();
            styles.insert(k, v.clone());
            v
        }
    }
}
