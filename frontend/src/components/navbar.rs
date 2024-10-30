use yew::prelude::*;

pub struct Navbar {
    props: Props,
}

#[derive(Properties, Clone)]
pub struct Props {}

impl Component for Navbar {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
          <div class="navbar">
            <img alt="logo" class="navbar_logo" src="/images/logo.png"/>
            <div class="navbar_title">{"SCG Ask"}</div>
          </div>
        }
    }
}
