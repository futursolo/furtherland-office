use std::borrow::Cow;

use yew::prelude::*;

use crate::traits::YieldStyle;

#[derive(Properties, Clone, Debug, PartialEq)]
pub(crate) struct SigninLogoProps {
    pub src: String,
    #[prop_or(80)]
    pub height: u64,
}

impl YieldStyle for SigninLogoProps {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-signin-logo".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        format!(
            r#"
            background-image: url({url});
            height: {height}px;
            width: 100%;
            background-repeat: no-repeat;
            background-size: contain;
            background-position: center;
            "#,
            url = &self.src,
            height = self.height
        )
        .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SigninLogo {
    props: SigninLogoProps,
    link: ComponentLink<Self>,
}

impl Component for SigninLogo {
    type Message = ();
    type Properties = SigninLogoProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            // self.style = self.props.yield_style();
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
