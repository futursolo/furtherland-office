use css_in_rust::Style;
use yew::prelude::*;

use crate::traits::YieldStyle;

#[derive(Properties, Clone, Debug, PartialEq)]
pub(crate) struct SectionProps {
    #[prop_or_default]
    pub children: Children,
}

impl YieldStyle for SectionProps {
    fn yield_style(&self) -> Style {
        let style_string = r#"
            display: flex;
            flex-direction: column;
            justify-content: space-around;
            align-items: center;
            position: relative;
            "#
        .to_string();

        Style::create("FlSection", style_string).unwrap()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Section {
    props: SectionProps,
    link: ComponentLink<Self>,
    style: Style,
}

impl Component for Section {
    type Message = ();
    type Properties = SectionProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let style = props.yield_style();
        Self { props, link, style }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if props != self.props {
            self.props = props;
            self.style = self.props.yield_style();
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        html! {
            <div>{ self.props.children.clone() }</div>
        }
    }
}
