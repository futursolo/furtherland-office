use std::borrow::Cow;
use std::time::Duration;

use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;
use yewtil::future::LinkFuture;

use crate::app_components;
use crate::backend::SigninResident;
use crate::components;
use crate::helpers::{call_after, document, sleep, WeakComponentLink};
use crate::manifest::Manifest;
use crate::traits::YieldStyle;

use app_components::{BottomLinks, FlexGrow, SigninLogo, SigninProfile, SigninSubTitle};
use components::{Body, Button, Card, Input, InputMsg, InputType, InputValue, Spinner};

#[derive(Clone, Properties, Debug, PartialEq)]
pub(crate) struct SigninAppProps {
    pub manifest: Manifest,
    pub first_section: SigninSection,
}

impl SigninAppProps {
    pub fn set_first_section(&mut self, section: SigninSection) {
        self.first_section = section;
    }
}

impl From<Manifest> for SigninAppProps {
    fn from(manifest: Manifest) -> SigninAppProps {
        SigninAppProps {
            manifest,
            first_section: SigninSection::Name,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum SigninSection {
    Name,
    Password,
    Otp,
    Signup,
    SignupFinish,
}

impl SigninSection {
    fn title(&self) -> &'static str {
        match self {
            Self::Name => "歡迎來到未來領域管理局",
            Self::Password => "歡迎回來",
            Self::Otp => "額外的安全驗證",
            Self::Signup => "歡迎來到未來領域管理局",
            Self::SignupFinish => "註冊成功",
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct SigninApp {
    link: ComponentLink<Self>,
    root_ref: NodeRef,
    props: SigninAppProps,
    section: SigninSection,
    busy: bool,

    resident: Option<SigninResident>,

    name_input_link: WeakComponentLink<Input>,
    name_input_val: Option<InputValue>,

    password_input_link: WeakComponentLink<Input>,
    password_input_val: Option<InputValue>,

    email_input_link: WeakComponentLink<Input>,
    email_input_val: Option<InputValue>,

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

    EmailInput(InputValue),

    SignupNext,

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
    type Properties = SigninAppProps;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let first_section = props.first_section.clone();

        Self {
            link,
            props,
            root_ref: NodeRef::default(),
            section: first_section,
            busy: false,

            resident: None,

            name_input_link: WeakComponentLink::default(),
            name_input_val: None,

            password_input_link: WeakComponentLink::default(),
            password_input_val: None,

            email_input_link: WeakComponentLink::default(),
            email_input_val: None,

            otp_input_link: WeakComponentLink::default(),
            otp_input_val: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            SigninMsg::Restart => self.restart(),
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
            SigninMsg::EmailInput(m) => {
                self.email_input_val = Some(m);
                false
            }
            SigninMsg::NameNext => self.on_name_next(),
            SigninMsg::PasswordNext => self.on_password_next(),
            SigninMsg::OtpNext => self.on_otp_next(),
            SigninMsg::SignupNext => self.on_signup_next(),

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

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Currently you cannot modify props once set
        false
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
                    <Card with_margin={ false } max_width={ "400px" } width={ "calc(100vw - 40px)" } height={ self.card_height() }>
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
                    <footer>{ "© 2020 未來領域管理局" }</footer>
                </div>
            </Body>
        }
    }
}

impl SigninApp {
    fn card_height(&self) -> &'static str {
        if self.props.first_section == SigninSection::Name {
            "500px"
        } else {
            "550px"
        }
    }

    fn to_title(&self) -> String {
        format!("登入 - {}", self.props.manifest.site_title)
    }

    fn restart(&mut self) -> ShouldRender {
        let link = self.link.clone();
        self.resident = None;
        let first_section = self.props.first_section.clone();
        call_after(
            move || link.send_message(SigninMsg::NextSection(first_section.clone())),
            Duration::from_millis(1),
        );
        false
    }

    fn on_name_next(&mut self) -> ShouldRender {
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

    fn on_password_next(&mut self) -> ShouldRender {
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

    fn on_otp_next(&mut self) -> ShouldRender {
        self.otp_input_link
            .borrow()
            .as_ref()
            .unwrap()
            .send_message(InputMsg::Validate);
        false
    }

    fn on_signup_next(&mut self) -> ShouldRender {
        self.name_input_link
            .borrow()
            .as_ref()
            .unwrap()
            .send_message(InputMsg::Validate);
        self.email_input_link
            .borrow()
            .as_ref()
            .unwrap()
            .send_message(InputMsg::Validate);
        self.password_input_link
            .borrow()
            .as_ref()
            .unwrap()
            .send_message(InputMsg::Validate);

        let _name_val: String = if let Some(ref m) = self.name_input_val {
            if !m.is_valid() {
                return false;
            }

            m.to_string()
        } else {
            return false;
        };

        let _email_val: String = if let Some(ref m) = self.email_input_val {
            if !m.is_valid() {
                return false;
            }

            m.to_string()
        } else {
            return false;
        };

        let _password_val: String = if let Some(ref m) = self.password_input_val {
            if !m.is_valid() {
                return false;
            }

            m.to_string()
        } else {
            return false;
        };

        self.busy = true;

        self.link.send_future(async {
            sleep(Duration::from_secs(2)).await;
            SigninMsg::NextSection(SigninSection::SignupFinish)
        });
        true
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

    fn signup_callback(&self) -> Callback<KeyboardEvent> {
        self.link.callback(|e: KeyboardEvent| {
            if e.key_code() == 13 {
                SigninMsg::SignupNext
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
                    <Input onkeyup=self.name_callback() weak_link=self.name_input_link.clone() name="name"
                        placeholder="用戶名" width="100%" oninput=self.link.callback(|s| SigninMsg::NameInput(s))
                        required=true minlength=3 maxlength=32 pattern="[a-zA-Z0-9]+"
                        pattern_hint="用戶名只能是數字和大小寫半角英文字符" />
                    <Button width="100%" onclick=self.link.callback(|_| SigninMsg::NameNext)>{ "下一步" }</Button>
                    <BottomLinks>
                        <a href="./signup" style="text-decoration: none; outline: 0 !important;"><SigninSubTitle text_align="right">{ "註冊新帳戶" }</SigninSubTitle></a>
                        <span>{ "忘記密碼" }</span>
                    </BottomLinks>
                </>
            },
            SigninSection::Password => html! {
                <>
                    <FlexGrow />
                    <SigninSubTitle>{ "如要繼續，請輸入密碼並點擊「登入」。" }</SigninSubTitle>
                    <Input type_=InputType::Password onkeyup=self.password_callback() weak_link=self.password_input_link.clone()
                        name="password" placeholder="密碼" width="100%" oninput=self.link.callback(|s| SigninMsg::PasswordInput(s))
                        required=true minlength=8 />
                    <Button width="100%" onclick=self.link.callback(|_| SigninMsg::PasswordNext)>{ "登入" }</Button>
                    <BottomLinks>
                        <span onclick=self.link.callback(|_e| SigninMsg::Restart)>{ "使用其它帳戶" }</span>
                        <span>{ "忘記密碼" }</span>
                    </BottomLinks>
                </>
            },
            SigninSection::Otp => html! {
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
            SigninSection::Signup => html! {
                <>
                    <FlexGrow />
                    <SigninSubTitle>{ "要註冊新帳戶，請填寫以下項目。" }</SigninSubTitle>
                    <Input onkeyup=self.signup_callback() weak_link=self.name_input_link.clone() name="name"
                        placeholder="用戶名" width="100%" oninput=self.link.callback(|s| SigninMsg::NameInput(s))
                        required=true minlength=3 maxlength=32 pattern="[a-zA-Z0-9]+"
                        pattern_hint="用戶名只能是數字和大小寫半角英文字符" />
                    <Input type_=InputType::Password onkeyup=self.signup_callback() weak_link=self.password_input_link.clone()
                        name="password" placeholder="密碼" width="100%" oninput=self.link.callback(|s| SigninMsg::PasswordInput(s))
                        required=true minlength=8 />
                    <Input type_=InputType::Email onkeyup=self.signup_callback() weak_link=self.email_input_link.clone() name="email"
                        placeholder="電子郵件地址" width="100%" oninput=self.link.callback(|s| SigninMsg::EmailInput(s))
                        required=true />
                    <Button width="100%" onclick=self.link.callback(|_| SigninMsg::SignupNext)>{ "註冊" }</Button>
                    <BottomLinks>
                        <a href="./signin" style="text-decoration: none; outline: 0 !important;"><SigninSubTitle>{ "已有帳戶？" }</SigninSubTitle></a>
                    </BottomLinks>
                </>
            },
            SigninSection::SignupFinish => html! {
                <>
                    <div style="flex-grow: 1; display: flex; flex-direction: column; align-items: center; justify-content: space-around; width: 100%; margin-top: 80px; margin-bottom: 80px;">
                        <SigninSubTitle>{ "請點擊「繼續」來登入。" }</SigninSubTitle>
                        <a href="./signin" style="text-decoration: none; outline: 0 !important; width: 100%; display: block;"><Button width="100%">{ "繼續" }</Button></a>
                    </div>
                </>
            },
        }
    }
}
