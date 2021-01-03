use std::borrow::Cow;

use yew::prelude::*;

use super::SigninSubTitle;
use crate::traits::YieldStyle;

// Start Item
#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BottomLinkItemPosition {
    First,
    // Middle,
    Last,
}

impl BottomLinkItemPosition {
    fn extra_style_str(&self) -> Cow<'static, str> {
        match self {
            Self::First => "text-align: right;\n".into(),
            Self::Last => "text-align: left;\n".into(),
        }
    }
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub(crate) struct BottomLinkItemProps {
    #[prop_or_default]
    children: Children,
    position: BottomLinkItemPosition,
}

#[derive(Debug, Clone)]
pub(crate) struct BottomLinkItem(BottomLinkItemProps);

impl Component for BottomLinkItem {
    type Message = ();
    type Properties = BottomLinkItemProps;
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

impl YieldStyle for BottomLinkItemProps {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-bottom-link-item".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        let mut style_string = r#"
        min-width: 120px;
        padding-left: 5px;
        padding-right: 5px;

        font-size: 0.9rem;
        text-align: center;
        color: rgb(100, 100, 100);
        "#
        .to_string();
        style_string.push_str(&self.position.extra_style_str());
        style_string.into()
    }
}
// End Item

#[derive(Properties, Clone, Debug, PartialEq)]
pub(crate) struct BottomLinksProps {
    children: Children,
}

#[derive(Debug, Clone)]
pub(crate) struct BottomLinks(BottomLinksProps);

impl Component for BottomLinks {
    type Message = ();
    type Properties = BottomLinksProps;
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
        if self.0.children.len() > 2 {
            panic!("You cannot have more than 2 items (for now).");
        }

        let mut children: Vec<Html> = Vec::new();

        for (index, child) in self.0.children.iter().enumerate() {
            if index > 0 {
                children.push(html! { <SigninSubTitle>{ "|" }</SigninSubTitle> });
            }

            let pos = if index == 0 {
                BottomLinkItemPosition::First
            } else {
                BottomLinkItemPosition::Last
            };

            children.push(html! {<BottomLinkItem position=pos>{child}</BottomLinkItem>});
        }

        html! {
            <div class=self.yield_style_class()>
                { children }
            </div>
        }
    }
}

impl YieldStyle for BottomLinks {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-bottom-links".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        r#"
        width: 100%;
        cursor: default;

        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        "#
        .into()
    }
}
