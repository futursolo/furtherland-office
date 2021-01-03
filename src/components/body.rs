use std::borrow::Cow;
use std::time::Duration;

use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

use crate::helpers::{call_after, document, is_mobile_safari, window, Instant};
use crate::traits::YieldStyle;

#[derive(Properties, Debug, Clone, PartialEq)]
pub struct BodyProps {
    #[prop_or(false)]
    pub grey_background: bool,
    pub children: Children,
    pub title: String,
}

impl YieldStyle for BodyProps {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-body".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        let mut style_string = r#"
        margin: 0;
        padding: 0;
        width: 100vw;
        /* min-height: 100vh; */
        font-size: 15px;

        display: flex;
        flex-direction: row;
        justify-content: space-around;
        align-items: center;

        @media only screen and (max-width: 340px) {
            font-size: 14px;
        }
        "#
        .to_string();

        if self.grey_background {
            style_string.push_str("background-color: rgb(244, 245, 249);\n");
        }

        if !is_mobile_safari() {
            style_string.push_str("min-height: 100vh;\n");
        }

        style_string.into()
    }
}

pub enum BodyMsg {
    UpdateViewHeight,
}

type BodyParentChanged = bool;

#[derive(Debug, Clone)]
struct BodyParent {
    element: HtmlElement,
    style_class: Option<String>,
    grey_background: bool,
}

impl BodyParent {
    fn new(element: HtmlElement, body_props: &BodyProps) -> Self {
        Self {
            element,
            style_class: None,
            grey_background: body_props.grey_background,
        }
    }

    fn change(&mut self, body_props: &BodyProps) {
        self.grey_background = body_props.grey_background;
    }

    fn render(&mut self) -> BodyParentChanged {
        let new_style_class = self.yield_style_class();
        if let Some(ref m) = self.style_class {
            if new_style_class.as_str() == m {
                return false;
            }

            self.element.class_list().remove_1(m).unwrap();
        }

        self.element.class_list().add_1(&new_style_class).unwrap();

        self.style_class = Some(new_style_class);

        true
    }
}

impl YieldStyle for BodyParent {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-html".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        let mut style_string = r#"
        margin: 0;
        padding: 0;
        width: 100vw;
        /* min-height: 100vh; */

        &[lang="zh-hant"] {
            font-family: "PingFang TC", "Helvetica Neue", Helvetica, sans-serif;
        }
        "#
        .to_string();

        if self.grey_background {
            style_string.push_str("background-color: rgb(244, 245, 249);\n");
        }

        if !is_mobile_safari() {
            style_string.push_str("min-height: 100vh;\n");
        }

        style_string.into()
    }
}

#[derive(Debug, Clone)]
pub struct Body {
    props: BodyProps,
    parent: Option<BodyParent>,
    link: ComponentLink<Self>,
    body_ref: NodeRef,
    last_vh_update: Option<Instant>,
}

impl Component for Body {
    type Message = BodyMsg;
    type Properties = BodyProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            parent: None,
            link,
            body_ref: NodeRef::default(),
            last_vh_update: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            BodyMsg::UpdateViewHeight => {
                // At least wait 100ms.
                if let Some(ref m) = self.last_vh_update {
                    if m.elapsed() < Duration::from_millis(100) {
                        return false;
                    }
                }

                self.last_vh_update = Some(Instant::now());
                self.update_view_height();

                true
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if is_mobile_safari() {
                self.register_events();
                self.link.send_message(BodyMsg::UpdateViewHeight);
            }
        }

        if let Some(ref mut m) = self.parent {
            m.change(&self.props);
            m.render();
        } else {
            let mut parent = BodyParent::new(self.html_element(), &self.props);
            parent.render();

            self.parent = Some(parent);
        }

        document().set_title(&self.props.title);
    }

    fn view(&self) -> Html {
        html! {
            <body class=self.props.yield_style_class() ref=self.body_ref.clone()>
                    { self.props.children.clone() }
            </body>
        }
    }
}

impl Body {
    fn body_element(&self) -> HtmlElement {
        self.body_ref.cast().unwrap()
    }

    fn html_element(&self) -> HtmlElement {
        self.body_element()
            .parent_node()
            .unwrap()
            .dyn_into()
            .unwrap()
    }

    fn update_view_height(&self) {
        let body = self.body_element();
        let html_ = self.html_element();

        let view_height = window().inner_height().unwrap().as_f64().unwrap() as i64;
        let view_height_str = format!("{}px", view_height);

        body.style()
            .set_property("min-height", &view_height_str)
            .unwrap();
        html_
            .style()
            .set_property("min-height", &view_height_str)
            .unwrap();
    }

    fn register_events(&self) {
        let link = self.link.clone();
        let cb = Closure::wrap(Box::new(move || {
            let link = link.clone();
            call_after(
                move || link.send_message(BodyMsg::UpdateViewHeight),
                Duration::from_millis(100),
            )
        }) as Box<dyn FnMut()>);

        window()
            .add_event_listener_with_callback("orientationchange", cb.as_ref().unchecked_ref())
            .unwrap();
        window()
            .add_event_listener_with_callback("pageshow", cb.as_ref().unchecked_ref())
            .unwrap();
        window()
            .add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref())
            .unwrap();

        cb.forget();
    }
}
