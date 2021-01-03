use std::borrow::Cow;

use yew::prelude::*;

use crate::traits::YieldStyle;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct AvatarProps {
    pub src: String,
}

impl YieldStyle for AvatarProps {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-avatar".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        format!(
            r#"
            height: 100px;
            width: 100px;
            border-radius: 100%;
            background-repeat: no-repeat;
            background-size: contain;
            background-position: center;
            display: block;
            background-image: url({});
            "#,
            &self.src
        )
        .into()
    }
}

#[derive(Debug, Clone)]
pub struct Avatar {
    props: AvatarProps,
    link: ComponentLink<Self>,
}

impl Component for Avatar {
    type Message = ();
    type Properties = AvatarProps;
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
            </div>
        }
    }
}
