use std::borrow::Cow;

use yew::prelude::*;

use crate::traits::YieldStyle;

#[derive(Properties, Clone, Debug, PartialEq)]
pub(crate) struct SigninSubTitleProps {
    #[prop_or_default]
    pub children: Children,

    #[prop_or("center".to_string())]
    pub text_align: String,
}

#[derive(Debug, Clone)]
pub(crate) struct SigninSubTitle(SigninSubTitleProps);

impl Component for SigninSubTitle {
    type Message = ();
    type Properties = SigninSubTitleProps;
    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.0 {
            self.0 = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.0.yield_style_class()>
                { self.0.children.clone() }
            </div>
        }
    }
}

impl YieldStyle for SigninSubTitleProps {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-signin-sub-title".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        let mut style_string = r#"
        font-size: 0.9rem;
        color: rgb(100, 100, 100);
        "#
        .to_string();

        style_string.push_str(&format!("text-align: {};\n", self.text_align));
        style_string.into()
    }
}
