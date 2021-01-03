use std::borrow::Cow;

use web_sys::MouseEvent;
use yew::prelude::*;

use crate::traits::YieldStyle;

#[derive(Debug, Clone, PartialEq)]
pub enum ButtonColor {
    Blue,
    Black,
    Red,
    Green,
    Yellow,
    Custom {
        normal: (u8, u8, u8),
        hover: (u8, u8, u8),
    },
}

impl ButtonColor {
    fn normal_color(&self) -> (u8, u8, u8) {
        match self {
            Self::Blue => (92, 184, 230),
            Self::Black => (5, 5, 5),
            Self::Red => (238, 82, 26),
            Self::Green => (50, 191, 50),
            Self::Yellow => (255, 193, 7),
            Self::Custom { normal, hover: _ } => *normal,
        }
    }

    fn hover_color(&self) -> (u8, u8, u8) {
        match self {
            Self::Blue => (125, 198, 235),
            Self::Black => (50, 50, 50),
            Self::Red => (240, 99, 48),
            Self::Green => (91, 204, 91),
            Self::Yellow => (255, 205, 57),
            Self::Custom { normal: _, hover } => *hover,
        }
    }
}

impl Default for ButtonColor {
    fn default() -> ButtonColor {
        ButtonColor::Blue
    }
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub color: ButtonColor,

    #[prop_or("40px".to_string())]
    pub height: String,

    #[prop_or(None)]
    pub width: Option<String>,

    #[prop_or_else(Callback::noop)]
    pub onclick: Callback<MouseEvent>,
}

impl YieldStyle for ButtonProps {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-button".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        let normal_color = self.color.normal_color();
        let hover_color = self.color.hover_color();

        let mut style_string = format!(
            r#"
            display: inline-flex;
            justify-content: center;
            align-items: center;
            flex-direction: row;

            border-radius: 500px;
            padding-left: 30px;
            padding-right: 30px;
            height: {height};
            box-sizing: border-box;

            text-align: center;

            background-color: rgb({}, {}, {});
            color: white;
            transition: background-color 0.20s;

            cursor: default;
            -moz-user-select: none;
            -webkit-user-select: none;
            -ms-user-select: none;
            user-select: none;

            &:hover {{
                background-color: rgb({}, {}, {});
            }}
            "#,
            normal_color.0,
            normal_color.1,
            normal_color.2,
            hover_color.0,
            hover_color.1,
            hover_color.2,
            height = &self.height,
        );

        if let Some(ref m) = self.width.as_ref() {
            style_string.push_str(format!("width: {};\n", m).as_str());
        }

        style_string.into()
    }
}

#[derive(Debug, Clone)]
pub struct Button {
    props: ButtonProps,
    link: ComponentLink<Self>,
    root_ref: NodeRef,
}

impl Component for Button {
    type Message = ();
    type Properties = ButtonProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            root_ref: NodeRef::default(),
        }
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
            <div ref=self.root_ref.clone() class=self.props.yield_style_class() onclick=self.props.onclick.clone()>
                { self.props.children.clone() }
            </div>
        }
    }
}
