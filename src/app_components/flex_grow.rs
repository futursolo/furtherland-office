use std::borrow::Cow;

use yew::prelude::*;

use crate::traits::YieldStyle;

#[derive(Debug, Clone)]
pub(crate) struct FlexGrow;

impl Component for FlexGrow {
    type Message = ();
    type Properties = ();
    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=self.yield_style_class()>
            </div>
        }
    }
}

impl YieldStyle for FlexGrow {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-flex-grow".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        r#"
        margin-top: 0 !important;
        margin-bottom: 0 !important;
        height: 1px;
        flex-grow: 1;
        width: 100%;
        "#
        .into()
    }
}
