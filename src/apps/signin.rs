use std::borrow::Cow;
use std::time::Duration;

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewtil::future::LinkFuture;

use super::AppProps;
use crate::app_components;
use crate::backend::SigninResident;
use crate::components;
use crate::helpers::{call_after, document, sleep, WeakComponentLink};
use crate::traits::YieldStyle;

use app_components::{BottomLinks, FlexGrow, SigninLogo, SigninProfile, SigninSubTitle};
use components::{Body, Button, Card, Input, InputMsg, InputType, InputValue, Spinner};

#[derive(Debug, Clone)]
pub(crate) enum SigninSection {
    Name,
    Password,
    Otp,
    // Signup,
    // SignupFinish,
}

impl SigninSection {
    fn title(&self) -> &'static str {
        match self {
            Self::Name => "歡迎來到未來領域管理局",
            Self::Password => "歡迎回來",
            Self::Otp => "額外的安全驗證",
            // Self::Signup => "歡迎來到未來領域管理局",
            // Self::SignupFinish => "註冊成功",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SigninApp {
    link: ComponentLink<Self>,
    root_ref: NodeRef,
    props: AppProps,
    section: SigninSection,
    busy: bool,

    resident: Option<SigninResident>,

    name_input_link: WeakComponentLink<Input>,
    name_input_val: Option<InputValue>,

    password_input_link: WeakComponentLink<Input>,
    password_input_val: Option<InputValue>,

    otp_input_link: WeakComponentLink<Input>,
    otp_input_val: Option<InputValue>,
}

#[derive(Debug, Clone)]
pub(crate) enum SigninMsg {
    Restart,
    NextSection(SigninSection),

    NameInput(InputValue),
    NameNext,

    PasswordInput(InputValue),
    PasswordNext,

    OtpInput(InputValue),
    OtpNext,

    Ignore,
}

impl YieldStyle for SigninApp {
    fn element_name(&self) -> Cow<'static, str> {
        "fl-signin-app".into()
    }

    fn yield_style_str(&self) -> Cow<'static, str> {
        r#"
        max-width: 1200px;
        display: block;
        padding-top: 20px;
        & > div > * {
            margin-top: 12px;
            margin-bottom: 12px;
        }
        footer {
            font-size: 0.8rem;
            height: 30px;
            line-height: 30px;
            text-align: center;
            color: rgb(150, 150, 150);
        }
        "#
        .into()
    }
}

impl Component for SigninApp {
    type Message = SigninMsg;
    type Properties = AppProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            props,
            root_ref: NodeRef::default(),
            section: SigninSection::Name,
            busy: false,

            resident: None,

            name_input_link: WeakComponentLink::default(),
            name_input_val: None,

            password_input_link: WeakComponentLink::default(),
            password_input_val: None,

            otp_input_link: WeakComponentLink::default(),
            otp_input_val: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SigninMsg::Restart => {
                let link = self.link.clone();
                self.resident = None;
                call_after(
                    move || link.send_message(SigninMsg::NextSection(SigninSection::Name)),
                    Duration::from_millis(1),
                );
                false
            }
            SigninMsg::NameInput(m) => {
                self.name_input_val = Some(m);
                false
            }
            SigninMsg::PasswordInput(m) => {
                self.password_input_val = Some(m);
                false
            }
            SigninMsg::OtpInput(m) => {
                self.otp_input_val = Some(m);
                false
            }
            SigninMsg::NameNext => {
                self.name_input_link
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .send_message(InputMsg::Validate);

                if let Some(ref m) = self.name_input_val {
                    if !m.is_valid() {
                        return false;
                    }
                    self.busy = true;

                    self.link.send_future(async {
                        sleep(Duration::from_secs(2)).await;
                        SigninMsg::NextSection(SigninSection::Password)
                    });
                    true
                } else {
                    false
                }
            }

            SigninMsg::PasswordNext => {
                self.password_input_link
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .send_message(InputMsg::Validate);

                if let Some(ref m) = self.password_input_val {
                    if !m.is_valid() {
                        return false;
                    }
                    self.busy = true;

                    self.link.send_future(async {
                        sleep(Duration::from_secs(2)).await;
                        SigninMsg::NextSection(SigninSection::Otp)
                    });
                    true
                } else {
                    false
                }
            }

            SigninMsg::OtpNext => {
                self.otp_input_link
                    .borrow()
                    .as_ref()
                    .unwrap()
                    .send_message(InputMsg::Validate);
                false
            }

            SigninMsg::NextSection(section) => {
                self.section = section;
                self.busy = false;
                self.resident = Some(SigninResident {
                        name: "futursolo".into(),
                        display_name: Some("星川かえで".into()),
                        avatar_url: Some("https://www.gravatar.com/avatar/0dd494a963ae648caebe34288b664ca6?s=200&d=mp".into()),
                    });
                true
            }

            SigninMsg::Ignore => false,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn rendered(&mut self, _first_render: bool) {
        self.focus_first_input();
    }

    fn view(&self) -> Html {
        let logo_url = self
            .props
            .manifest
            .get_asset_url("images/favicon-wide.png")
            .unwrap();

        html! {
            <Body grey_background={ true } title={ self.to_title() }>
                <div class=self.yield_style_class() ref=self.root_ref.clone()>
                    <Card with_margin={ false } max_width={ "400px" } width={ "calc(100vw - 40px)" } height={ "500px" }>
                        <div style="font-size: 1.5rem;">{ self.section.title() }</div>
                        {
                            if let Some(ref m) = self.resident {
                                html!{ <SigninProfile resident={ m.clone() } /> }
                            } else {
                                html!{
                                    <SigninLogo src={ logo_url } />
                                }
                            }
                        }
                        { self.render_section() }
                    </Card>
                    <footer>{ "&copy;&nbsp;2020 未來領域管理局" }</footer>
                </div>
            </Body>
        }
    }
}

impl SigninApp {
    fn to_title(&self) -> String {
        format!("登入 - {}", self.props.manifest.site_title)
    }

    fn name_callback(&self) -> Callback<KeyboardEvent> {
        self.link.callback(|e: KeyboardEvent| {
            if e.key_code() == 13 {
                SigninMsg::NameNext
            } else {
                SigninMsg::Ignore
            }
        })
    }

    fn password_callback(&self) -> Callback<KeyboardEvent> {
        self.link.callback(|e: KeyboardEvent| {
            if e.key_code() == 13 {
                SigninMsg::PasswordNext
            } else {
                SigninMsg::Ignore
            }
        })
    }

    fn otp_callback(&self) -> Callback<KeyboardEvent> {
        self.link.callback(|e: KeyboardEvent| {
            if e.key_code() == 13 {
                SigninMsg::OtpNext
            } else {
                SigninMsg::Ignore
            }
        })
    }

    fn focus_first_input(&self) {
        if let Some(m) = document().query_selector("input").unwrap() {
            m.dyn_into::<HtmlElement>().unwrap().focus().unwrap();
        }
    }

    fn render_section(&self) -> Html {
        if self.busy {
            return html! {
                <>
                    <div style="text-align: center; flex-grow: 1; display: flex; justify-content: center; align-items: center; flex-direction: column;">
                        <SigninSubTitle>{ "請稍候..." }</SigninSubTitle>
                        <Spinner />
                    </div>
                </>
            };
        }

        match &self.section {
            SigninSection::Name => html! {
                <>
                    <FlexGrow />
                    <SigninSubTitle>{ "如要繼續，請輸入用戶名並點擊「下一步」。" }</SigninSubTitle>
                    <Input onkeyup=self.name_callback() weak_link=self.name_input_link.clone() name="name" placeholder="用戶名" width="100%" oninput=self.link.callback(|s| SigninMsg::NameInput(s)) required=true minlength=3 maxlength=32 pattern="[a-zA-Z0-9]+" pattern_hint="用戶名只能是數字和大小寫半角英文字符" />
                    <Button width="100%" onclick=self.link.callback(|_| SigninMsg::NameNext)>{ "下一步" }</Button>
                    <BottomLinks>
                        <a href="./signup" style="text-decoration: none; outline: 0 !important;"><SigninSubTitle text_align="right">{ "註冊新帳戶" }</SigninSubTitle></a>
                        <span>{ "忘記密碼" }</span>
                    </BottomLinks>
                </>
            },
            SigninSection::Password { .. } => html! {
                <>
                    <FlexGrow />
                    <SigninSubTitle>{ "如要繼續，請輸入密碼並點擊「登入」。" }</SigninSubTitle>
                    <Input type_=InputType::Password onkeyup=self.password_callback() weak_link=self.password_input_link.clone() name="password" placeholder="密碼" width="100%" oninput=self.link.callback(|s| SigninMsg::PasswordInput(s)) required=true minlength=8 />
                    <Button width="100%" onclick=self.link.callback(|_| SigninMsg::PasswordNext)>{ "登入" }</Button>
                    <BottomLinks>
                        <span onclick=self.link.callback(|_e| SigninMsg::Restart)>{ "使用其它帳戶" }</span>
                        <span>{ "忘記密碼" }</span>
                    </BottomLinks>
                </>
            },
            SigninSection::Otp { .. } => html! {
                <>
                    <FlexGrow />
                    <SigninSubTitle>{ "由於你已啟用兩步驗證，你需要輸入兩步驗證代碼來完成登入。" }</SigninSubTitle>
                    <Input onkeyup=self.otp_callback() weak_link=self.otp_input_link.clone() name="otpcode" placeholder="兩步驗證代碼" width="100%" oninput=self.link.callback(|s| SigninMsg::OtpInput(s)) required=true minlength=6 maxlength=6 pattern="[0-9]+" pattern_hint="兩步驗證代碼只能是六位數字" />
                    <Button width="100%" onclick=self.link.callback(|_| SigninMsg::OtpNext)>{ "驗證" }</Button>
                    <BottomLinks>
                        <span onclick=self.link.callback(|_e| SigninMsg::Restart)>{ "使用其它帳戶" }</span>
                        <span>{ "忘記密碼" }</span>
                    </BottomLinks>
                </>
            },
        }
    }
}
