use serde::Serialize;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{prelude::*, web_sys};

#[derive(Serialize)]
struct FormData {
    name: String,
    email: String,
    content: String,
}

pub struct Form {
    link: ComponentLink<Self>,
    name: String,
    email: String,
    content: String,
    fetch_task: Option<FetchTask>,
    loading: bool,
    validation_errors: ValidationErrors,
    success_message: Option<String>,
}

#[derive(Default)]
struct ValidationErrors {
    name: Option<String>,
    email: Option<String>,
    content: Option<String>,
}

pub enum Msg {
    UpdateName(String),
    UpdateEmail(String),
    UpdateContent(String),
    Submit,
    Response(Result<(), anyhow::Error>),
    CloseSuccessMessage,
}

impl Component for Form {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            name: String::new(),
            email: String::new(),
            content: String::new(),
            fetch_task: None,
            loading: false,
            validation_errors: ValidationErrors::default(),
            success_message: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.name = name;
                self.validate_name();
                true
            }
            Msg::UpdateEmail(email) => {
                self.email = email;
                self.validate_email();
                true
            }
            Msg::UpdateContent(content) => {
                self.content = content;
                self.validate_content();
                true
            }
            Msg::Submit => {
                self.validate_all();

                if self.is_valid() && !self.loading {
                    self.loading = true;

                    let form_data = FormData {
                        name: self.name.clone(),
                        email: self.email.clone(),
                        content: self.content.clone(),
                    };

                    let request = Request::post("https://ask.scg.skku.ac.kr/v1/send")
                        .header("Content-Type", "application/json; charset=utf-8")
                        .body(Json(&form_data))
                        .expect("Could not build request.");

                    let callback = self.link.callback(|response: Response<Nothing>| {
                        if response.status().is_success() {
                            Msg::Response(Ok(()))
                        } else {
                            Msg::Response(Err(anyhow::anyhow!("Request failed")))
                        }
                    });

                    let task =
                        FetchService::fetch(request, callback).expect("Failed to start request");
                    self.fetch_task = Some(task);
                }
                true
            }
            Msg::Response(result) => {
                self.loading = false;
                self.fetch_task = None;
                match result {
                    Ok(_) => {
                        self.name = String::new();
                        self.email = String::new();
                        self.content = String::new();
                        self.validation_errors = ValidationErrors::default();
                        self.success_message = Some("문의가 성공적으로 접수되었습니다.".into());

                        if let Some(window) = window() {
                            let link = self.link.clone();
                            let closure = Closure::wrap(Box::new(move || {
                                link.send_message(Msg::CloseSuccessMessage);
                            })
                                as Box<dyn Fn()>);

                            window
                                .set_timeout_with_callback_and_timeout_and_arguments_0(
                                    closure.as_ref().unchecked_ref(),
                                    3100,
                                )
                                .unwrap();

                            closure.forget();
                        }
                    }
                    Err(err) => {
                        log::error!("Request failed: {:?}", err);
                    }
                }
                true
            }
            Msg::CloseSuccessMessage => {
                self.success_message = None;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let is_disabled = self.loading || !self.is_valid();

        html! {
            <div>
                {
                    if self.success_message.is_some() {
                        html! {
                            <div class="success-message show">
                                <p>{ self.success_message.as_ref().unwrap() }</p>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

                <form onsubmit=self.link.callback(|e: FocusEvent| {
                    e.prevent_default();
                    Msg::Submit
                })>
                    <div>
                        <label for="name">{"이름:"}</label>
                        <input
                            type="text"
                            id="name"
                            value={self.name.clone()}
                            placeholder="이름을 작성해주세요."
                            oninput=self.link.callback(|e: InputData| Msg::UpdateName(e.value))
                        />
                        {
                            if let Some(error) = &self.validation_errors.name {
                                html! { <p class="error">{error}</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div>
                        <label for="email">{"이메일:"}</label>
                        <input
                            type="email"
                            id="email"
                            placeholder="답변을 받을 이메일을 작성해주세요."
                            value={self.email.clone()}
                            oninput=self.link.callback(|e: InputData| Msg::UpdateEmail(e.value))
                        />
                        {
                            if let Some(error) = &self.validation_errors.email {
                                html! { <p class="error">{error}</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <div>
                        <label for="content">{"내용:"}</label>
                        <textarea
                            id="content"
                            placeholder="내용을 작성해주세요."
                            value={self.content.clone()}
                            oninput=self.link.callback(|e: InputData| Msg::UpdateContent(e.value))
                        />
                        {
                            if let Some(error) = &self.validation_errors.content {
                                html! { <p class="error">{error}</p> }
                            } else {
                                html! {}
                            }
                        }
                    </div>
                    <button type="submit" disabled=is_disabled>
                        {
                            if self.loading {
                                html! {
                                    <>
                                        <span class="spinner"></span>
                                    </>
                                }
                            } else {
                                { html! { "문의하기" } }
                            }
                        }
                    </button>
                </form>
            </div>
        }
    }
}

impl Form {
    fn validate_name(&mut self) {
        if self.name.trim().is_empty() {
            self.validation_errors.name = Some("이름을 작성해주세요.".into());
        } else {
            self.validation_errors.name = None;
        }
    }

    fn validate_email(&mut self) {
        if self.email.trim().is_empty() {
            self.validation_errors.email = Some("이메일을 작성해주세요.".into());
        } else if !self.email.contains("@") {
            self.validation_errors.email = Some("올바른 이메일 주소를 작성해주세요.".into());
        } else {
            self.validation_errors.email = None;
        }
    }

    fn validate_content(&mut self) {
        if self.content.trim().is_empty() {
            self.validation_errors.content = Some("내용을 작성해주세요.".into());
        } else {
            self.validation_errors.content = None;
        }
    }

    fn validate_all(&mut self) {
        self.validate_name();
        self.validate_email();
        self.validate_content();
    }

    fn is_valid(&self) -> bool {
        self.validation_errors.name.is_none()
            && self.validation_errors.email.is_none()
            && self.validation_errors.content.is_none()
    }
}
