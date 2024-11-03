use serde::Serialize;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use crate::get_api_url;

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
}

pub enum Msg {
    UpdateName(String),
    UpdateEmail(String),
    UpdateContent(String),
    Submit,
    Response(Result<(), anyhow::Error>),
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
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateName(name) => {
                self.name = name;
                true
            }
            Msg::UpdateEmail(email) => {
                self.email = email;
                true
            }
            Msg::UpdateContent(content) => {
                self.content = content;
                true
            }
            Msg::Submit => {
                self.loading = true;

                let form_data = FormData {
                    name: self.name.clone(),
                    email: self.email.clone(),
                    content: self.content.clone(),
                };

                let request = Request::post(get_api_url())
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

                let task = FetchService::fetch(request, callback).expect("Failed to start request");
                self.fetch_task = Some(task);

                false
            }
            Msg::Response(result) => {
                self.loading = false;
                match result {
                    Ok(_) => log::info!("Request successful"),
                    Err(err) => log::error!("Request failed: {:?}", err),
                }
                self.fetch_task = None;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
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
                </div>
                <div>
                    <label for="email">{"이메일:"}</label>
                    <input
                        type="email"
                        id="email"
                        placeholder="답변을 받을 이메일을 작성해주세요."
                        value=self.email.clone()
                        oninput=self.link.callback(|e: InputData| Msg::UpdateEmail(e.value))
                    />
                </div>
                <div>
                    <label for="content">{"내용:"}</label>
                    <textarea
                        id="content"
                        placeholder="내용을 작성해주세요."
                        value={self.content.clone()}
                        oninput=self.link.callback(|e: InputData| Msg::UpdateContent(e.value))
                    />
                </div>
                <button type="submit" disabled=self.loading>{"문의하기"}</button>
            </form>
        }
    }
}
