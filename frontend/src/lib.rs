// src/lib.rs
mod app;
mod components;
mod env;
mod pages;
mod route;

use env::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}
