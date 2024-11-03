use yew::web_sys::window;

pub fn get_env_var(key: &str) -> Option<String> {
    window()
        .and_then(|win| win.get(key))
        .and_then(|val| val.as_string())
}
