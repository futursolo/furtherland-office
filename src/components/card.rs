use std::borrow::Cow;

use yew::prelude::*;

use crate::traits::YieldStyle;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct CardProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or(true)]
    pub with_shadow: bool,

    #[prop_or(true)]
    pub with_margin: bool,

    #[prop_or(true)]
    pub with_padding: bool,

    #[prop_or(None)]
    pub max_width: Option<String>,

    #[prop_or(None)]
    pub width: Option<String>,

    #[prop_or(None)]
    pub max_height: Option<String>,

    #[prop_or(None)]
    pub height: Option<String>,

    #[prop_or(None)]
    pub flex_direction: Option<String>,

    #[prop_or(None)]
    pub justify_content: Option<String>,

    #[prop_or(None)]
    pub align_items: Option<String>,
}

impl YieldStyle for CardProps {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-card".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        let mut style_string = r#"
        display: flex;
        flex-direction: column;
        justify-content: space-around;
        align-items: center;

        border-radius: 10px;
        background-color: white;
        box-sizing: border-box;
        padding: 20px;

        @supports (backdrop-filter: blur(100px) saturate(180%)) or (-webkit-backdrop-filter: blur(100px) saturate(180%)) {
            background-color: rgba(255, 255, 255, 0.7);
            backdrop-filter: blur(100px) saturate(180%);
            -webkit-backdrop-filter: blur(100px) saturate(180%)
        }
        "#.to_string();

        if self.with_shadow {
            style_string.push_str("box-shadow: 0 0 10px 0 rgba(150, 150, 150, 0.8);\n");
        }

        if self.with_margin {
            style_string.push_str("margin: 20px;\n");
        } else {
            style_string.push_str("margin: 0;\n");
        }

        if self.with_padding {
            style_string.push_str("padding: 20px;\n");
        } else {
            style_string.push_str("padding: 0;\n");
        }

        if let Some(ref m) = self.max_width {
            style_string.push_str(format!("max-width: {};\n", m).as_str());
        }

        if let Some(ref m) = self.width {
            style_string.push_str(format!("width: {};\n", m).as_str());
        }

        if let Some(ref m) = self.max_height {
            style_string.push_str(format!("max-height: {};\n", m).as_str());
        }

        if let Some(ref m) = self.height {
            style_string.push_str(format!("height: {};\n", m).as_str());
        }

        if let Some(ref m) = self.flex_direction {
            style_string.push_str(format!("flex-direction: {};\n", m).as_str());
        }

        if let Some(ref m) = self.justify_content {
            style_string.push_str(format!("justify-content: {};\n", m).as_str());
        }

        if let Some(ref m) = self.align_items {
            style_string.push_str(format!("align-items: {};\n", m).as_str());
        }
        style_string.into()
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    props: CardProps,
    link: ComponentLink<Self>,
}

impl Component for Card {
    type Message = ();
    type Properties = CardProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.props.yield_style_class()>
                { self.props.children.clone() }
            </div>
        }
    }
}
