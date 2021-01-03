use std::borrow::Cow;

use yew::prelude::*;

use crate::backend::SigninResident;
use crate::components::Avatar;
use crate::traits::YieldStyle;

#[derive(Properties, Clone, Debug, PartialEq)]
pub(crate) struct SigninProfileProps {
    #[prop_or_default]
    pub resident: SigninResident,
}

impl YieldStyle for SigninProfileProps {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-signin-profile".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        r#"
            display: block;
            text-align: center;

            .name {
                font-size: 0.9rem;
                color: rgb(100, 100, 100);
            }

            .display-name {
                font-size: 1rem;
                color: black;
            }
            "#
        .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SigninProfile {
    props: SigninProfileProps,
    link: ComponentLink<Self>,
}

impl Component for SigninProfile {
    type Message = ();
    type Properties = SigninProfileProps;
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
                <Avatar src={ self.props.resident.avatar_url() } />
                <div class="name">{ self.props.resident.name.clone() }</div>
                {
                    if let Some(ref m) = self.props.resident.display_name {
                        html! { <div class="display-name">{ m.clone() }</div> }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }
}
