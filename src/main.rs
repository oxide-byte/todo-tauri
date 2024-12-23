mod components;
mod entities;

use crate::components::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App)
}