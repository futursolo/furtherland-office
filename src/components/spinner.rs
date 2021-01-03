use std::borrow::Cow;

use yew::prelude::*;

use crate::traits::YieldStyle;

#[derive(Debug, Clone)]
pub struct Spinner;

impl Component for Spinner {
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
                <div class="bounce1 bounce"></div>
                <div class="bounce2 bounce"></div>
                <div class="bounce3 bounce"></div>
            </div>
        }
    }
}

impl YieldStyle for Spinner {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-spinner".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        r#"
            display: inline-flex;
            align-items: center;
            flex-direction: row;
            justify-content: space-evenly;

            padding-left: 20px;
            padding-right: 20px;
            height: 40px;
            width: 120px;
            box-sizing: border-box;

            text-align: center;

            .bounce {
                width: 0.7rem;
                height: 0.7rem;
                background-color: rgb(100, 100, 100);
                transition: background-color 0.20s;

                border-radius: 100%;
                display: inline-block;
                -webkit-animation: sk-bouncedelay 1.4s infinite ease-in-out both;
                animation: sk-bouncedelay 1.4s infinite ease-in-out both;
            }

            .bounce1 {
                -webkit-animation-delay: -0.32s;
                animation-delay: -0.32s;
            }

            .bounce2 {
                -webkit-animation-delay: -0.16s;
                animation-delay: -0.16s;
            }

            @-webkit-keyframes sk-bouncedelay {
                0%, 80%, 100% { -webkit-transform: scale(0) }
                40% { -webkit-transform: scale(1.0) }
            }

            @keyframes sk-bouncedelay {
                0%, 80%, 100% {
                    -webkit-transform: scale(0);
                    transform: scale(0);
                } 40% {
                    -webkit-transform: scale(1.0);
                    transform: scale(1.0);
                }
            }
            "#
        .into()
    }
}
