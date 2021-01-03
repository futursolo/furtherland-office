use std::borrow::Cow;
use std::ops::Deref;
use std::string::ToString;

use web_sys::{HtmlInputElement, KeyboardEvent};
use yew::prelude::*;

use crate::helpers::WeakComponentLink;
use crate::traits::YieldStyle;

#[derive(Clone, Debug, PartialEq)]
pub struct InputValue {
    value: String,
    is_valid: bool,
}

impl InputValue {
    pub fn new(value: String, is_valid: bool) -> Self {
        Self { value, is_valid }
    }

    pub fn into_string(self) -> String {
        self.value
    }

    pub fn is_valid(&self) -> bool {
        self.is_valid
    }
}

impl ToString for InputValue {
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

impl Deref for InputValue {
    type Target = str;

    fn deref(&self) -> &str {
        self.value.as_str()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputType {
    Text,
    Password,
}

impl InputType {
    fn type_str(&self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Password => "password",
        }
    }
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct InputProps {
    #[prop_or(None)]
    pub value: Option<String>,
    #[prop_or_default]
    pub input_ref: NodeRef,
    #[prop_or(InputType::Text)]
    pub type_: InputType,

    #[prop_or_else(WeakComponentLink::default)]
    pub weak_link: WeakComponentLink<Input>,

    #[prop_or_else(Callback::noop)]
    pub oninput: Callback<InputValue>,

    #[prop_or_else(Callback::noop)]
    pub onkeyup: Callback<KeyboardEvent>,

    #[prop_or("40px".to_string())]
    pub height: String,

    #[prop_or("400px".to_string())]
    pub width: String,

    #[prop_or(None)]
    pub pattern_hint: Option<String>,

    #[prop_or(false)]
    pub disabled: bool,

    #[prop_or(false)]
    pub required: bool,

    #[prop_or(false)]
    pub readonly: bool,

    #[prop_or(false)]
    pub autofocus: bool,

    #[prop_or(None)]
    pub maxlength: Option<u64>,

    #[prop_or(None)]
    pub minlength: Option<u64>,

    #[prop_or(None)]
    pub pattern: Option<String>,

    #[prop_or(None)]
    pub placeholder: Option<String>,

    #[prop_or(None)]
    pub spellcheck: Option<bool>,

    #[prop_or(None)]
    pub autocomplete: Option<bool>,

    #[prop_or(None)]
    pub name: Option<String>,

    #[prop_or(None)]
    pub tabindex: Option<u64>,

    #[prop_or(None)]
    pub title: Option<String>,
}

#[derive(Debug, Clone)]
pub enum InputMsg {
    Focus,
    Input(String),
    Validate,
    BecomeInvalid(String),
}

#[derive(Debug, Clone)]
pub struct Input {
    props: InputProps,
    link: ComponentLink<Self>,
    invalid: bool,
    focus_after_report_validity: bool,
}

impl Component for Input {
    type Message = InputMsg;
    type Properties = InputProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        props.weak_link.borrow_mut().replace(link.clone());
        Self {
            props,
            link,
            invalid: false,
            focus_after_report_validity: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        let input = self.props.input_ref.cast::<HtmlInputElement>().unwrap();
        match msg {
            InputMsg::Focus => {
                log::debug!("Input Focused!");
                if !self.focus_after_report_validity {
                    self.invalid = false;
                }
                true
            }
            InputMsg::Input(val) => {
                self.props
                    .oninput
                    .emit(InputValue::new(val.clone(), input.validity().valid()));

                let mut changed = false;
                if self.invalid {
                    self.invalid = false;
                    changed = true;
                }

                let validity = input.validity();

                if validity.valid() {
                    return changed;
                }

                if validity.pattern_mismatch() {
                    input.set_custom_validity(self.props.pattern_hint.as_deref().unwrap_or(""));
                    return changed;
                }
                input.set_custom_validity("");

                changed
            }
            InputMsg::Validate => {
                log::debug!("Validating Input");
                self.invalid = !input.report_validity();
                self.focus_after_report_validity = true;
                log::debug!("Result: is_valid: {}", !self.invalid);
                true
            }
            InputMsg::BecomeInvalid(m) => {
                input.set_custom_validity(&m);
                self.invalid = true;
                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        let input = self.props.input_ref.cast::<HtmlInputElement>().unwrap();

        if let Some(ref m) = self.props.value.as_ref() {
            if m.as_str() != input.value().as_str() {
                input.set_value(m);
            }
        }

        if let Some(m) = self.props.maxlength.as_ref() {
            input.set_attribute("maxlength", &m.to_string()).unwrap();
        } else {
            input.remove_attribute("maxlength").unwrap();
        }

        if let Some(m) = self.props.minlength.as_ref() {
            input.set_attribute("minlength", &m.to_string()).unwrap();
        } else {
            input.remove_attribute("minlength").unwrap();
        }

        if let Some(ref m) = self.props.pattern.as_ref() {
            input.set_attribute("pattern", m).unwrap();
        } else {
            input.remove_attribute("pattern").unwrap();
        }

        if let Some(ref m) = self.props.placeholder.as_ref() {
            input.set_attribute("placeholder", m).unwrap();
        } else {
            input.remove_attribute("placeholder").unwrap();
        }

        if let Some(m) = self.props.spellcheck.as_ref() {
            input.set_attribute("spellcheck", &m.to_string()).unwrap();
        } else {
            input.remove_attribute("spellcheck").unwrap();
        }

        if let Some(m) = self.props.autocomplete.as_ref() {
            input.set_attribute("autocomplete", &m.to_string()).unwrap();
        } else {
            input.remove_attribute("autocomplete").unwrap();
        }

        if let Some(ref m) = self.props.name.as_ref() {
            input.set_attribute("name", m).unwrap();
        } else {
            input.remove_attribute("name").unwrap();
        }

        if let Some(ref m) = self.props.tabindex.as_ref() {
            input.set_attribute("tabindex", &m.to_string()).unwrap();
        } else {
            input.remove_attribute("tabindex").unwrap();
        }

        if let Some(ref m) = self.props.title.as_ref() {
            input.set_attribute("title", m).unwrap();
        } else {
            input.remove_attribute("title").unwrap();
        }
    }

    fn view(&self) -> Html {
        html! {
            <input class=self.yield_style_class() ref=self.props.input_ref.clone() onkeyup=self.props.onkeyup.clone() onfocus=self.link.callback(|_| InputMsg::Focus) oninput=self.link.callback(|e: InputData| InputMsg::Input(e.value)) type={ self.props.type_.type_str() } disabled={ self.props.disabled } autofocus={ self.props.autofocus } required={ self.props.required } readonly={ self.props.readonly } />
        }
    }
}

impl YieldStyle for Input {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-input".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        let mut style_string = format!(
            r#"
            height: {height};
            width: {width};
            border-radius: 500px;
            border: 1px solid rgb(150, 150, 150);
            box-sizing: border-box;
            padding-left: 20px;
            padding-right: 20px;
            outline: 0;
            transition: box-shadow 0.20s, border-color 0.20s, background-color 0.20s;
            font-size: 1rem;

            -webkit-appearance: none;
        "#,
            height = &self.props.height,
            width = &self.props.width,
        );

        if self.invalid {
            style_string.push_str(
                r#"
                border-color: rgb(238, 82, 26);
                box-shadow: 0 0 0 2px rgba(238, 82, 26, 1);
                background-color: rgb(254, 237, 234);
                "#,
            );
        } else {
            style_string.push_str(
                r#"
                box-shadow: 0 0 0 2px rgba(125, 198, 235, 0);
                background-color: rgb(255, 255, 255);

                &:hover {
                    border-color: rgb(100, 100, 100);
                }

                &:focus {
                    border-color: rgb(125, 198, 235);
                    box-shadow: 0 0 0 2px rgba(125, 198, 235, 1);
                }
                "#,
            );
        }

        style_string.into()
    }
}
