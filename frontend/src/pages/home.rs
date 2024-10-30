use std::time::Duration;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::window;
use yew::services::timeout::{TimeoutService, TimeoutTask};
use yew::{prelude::*, web_sys};

use crate::components::Form;

pub struct Home {
    link: ComponentLink<Self>,
    text: String,
    index: usize,
    task: Option<TimeoutTask>,
    show_form: bool,
}

pub enum Msg {
    TypeWriter,
    ShowForm,
}

impl Home {
    fn start_typing(&mut self) {
        let callback = self.link.callback(|_| Msg::TypeWriter);
        let handle = TimeoutService::spawn(Duration::from_millis(100), callback);
        self.task = Some(handle);
    }
}

impl Component for Home {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|_| Msg::ShowForm);
        let window = window().unwrap();
        let closure = Closure::wrap(Box::new(move || {
            callback.emit(());
        }) as Box<dyn Fn()>);
        window
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                2000,
            )
            .expect("should register `setTimeout` OK");
        closure.forget();

        let mut model = Self {
            link,
            text: String::new(),
            index: 0,
            task: None,
            show_form: false,
        };
        model.start_typing();
        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TypeWriter => {
                let full_text = "궁금한 점이 있으신가요? 언제든지 문의해 주세요!";
                if self.index < full_text.len() {
                    if let Some(c) = full_text.chars().nth(self.index) {
                        self.text.push(c);
                        self.index += 1;
                        self.start_typing();
                    }
                }
            }
            Msg::ShowForm => {
                self.show_form = true;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
          <>
            <div class="typing-container">
              <span id="text">{ &self.text }</span><span id="cursor"></span>
            </div>
            <div class={if self.show_form { "form-container animate" } else { "form-container" }}>
              <Form/>
            </div>
          </>
        }
    }
}
