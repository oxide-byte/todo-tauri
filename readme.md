# Todo - Leptos - Tauri

## Introduction

This is a simple POC of a [Leptos (0.7)](https://leptos.dev/) Todo Web Application embedded in the [Tauri 2](https://v2.tauri.app/) Framework for Desktop and/or Mobile Applications.

## Preparation

* cargo install trunk
* cargo install tauri-cli
* rustup target add wasm32-unknown-unknown

## Creation

cargo new todo-tauri

## Todo Leptos

We start doing a CSR (client-side rendering) Todo Lepos application in a couple of steps. 
This part could easily replaced by an Angular, React or other Web Framework.

Adding the dependency to cargo.toml

The Project itself is currently on Leptos 0.8.2. The documentation of steps is based
on a previous version:

```yaml
[dependencies]
leptos = { version = "0.7.3", features = ["csr"] }
```

We validate the current environment by running the default Hello World

```shell
cargo build
cargo run
```

Let add a simple container index.html file:

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <title>Todo-Tauri</title>
</head>
<body></body>
</html>
```

And modify the current main.rs

```rust
use leptos::prelude::*;

fn main() {
    mount_to_body(|| view! { <h1>Hello World</h1> });
}
```

As we use Leptos and WebAssembly we now render a code that is shown in a browser. 
Trunk offers us a Hot Reload option for developing the Frontend part.

we add a general configuration file trunk.toml
```toml
[build]
target = "index.html"
dist = "dist"
```

start the service

```shell
trunk serve
```

and open in the browser the default url http://127.0.0.1:8080/

Next apply some style to the page in configuring Tailwind (version 4)

Add a file tailwind.css under /style

```css
@import "tailwindcss";
```

Add the link on the header of the index.html

```html
...
<head>
    <link data-trunk rel="tailwind-css" href="/style/tailwind.css" />
</head>
...
```
and add in the root a config file /tailwind.config.js

```javascript
/** @type {import('tailwindcss').Config} */
module.exports = {
    content: {
        files: ["*.html", "./src/**/*.rs"],
        transform: {
            rs: (content) => content.replace(/(?:^|\s)class:/g, ' '),
        },
    },
    theme: {
        extend: {},
    },
    plugins: [],
}
```

Finally we add a Tailwind operator to the code:

```rust
mount_to_body(|| view! { <h1 class="text-4xl">Hello World with Tailwind</h1> });
```

With this we have closed the preparation for creating a client-side web page. The current
produced artifacts are in the folder /dist and could deployed to a server, on Github as Github Page or on AWS in a S3Bucket.

## Todo App

First step, we add some dependencies to the cargo file:
```yaml
[dependencies]
  leptos = { version = "0.7.3", features = ["csr"] }
  uuid = { version = "1.11.0", features = ["v4", "js"] }
  instant = { version = "0.1.13", features = [ "wasm-bindgen", "inaccurate" ] }
```

Now we add a couple of files in our source:

in src/entities

todo.rs
```rust
use instant::Instant;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: Instant,
}

impl Todo {
    pub fn new(title: String, description: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let created = Instant::now();
        Todo { id, title, description, created }
    }

    pub(crate) fn new_empty() -> Todo {
        Self::new("".to_string(), "".to_string())
    }
}
```

and include it in the mod.rs
```rust
mod todo;

pub use todo::Todo;
```

Next the UI components in src/components:

todo_item.rs
```rust
use crate::entities::Todo;
use leptos::html::*;
use leptos::prelude::*;

#[component]
pub fn TodoItem<E, D>(todo: Todo, edit: E, delete: D,
) -> impl IntoView
where
    D: Fn(Todo) + 'static,
    E: Fn(Todo) + 'static,
{
    let button_mod_class = "text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2";
    let button_del_class = "text-white bg-red-700 hover:bg-red-800 focus:ring-4 focus:outline-none focus:ring-red-300 font-medium rounded-full text-sm p-2.5 text-center inline-flex items-center mr-2";
    let todo_item: RwSignal<Todo> = RwSignal::new(todo);

    let on_edit = move |_| {
        edit(todo_item.get());
    };

    let on_delete = move |_| {
        delete(todo_item.get());
    };

    view! {
          <div class="bg-white shadow-md rounded px-8 pt-6 pb-8 mb-4 flex flex-row">

                <div class="basis-11/12">
                    <p class="text-lg text-gray-900">
                        {todo_item.get().title}
                    </p>

                    <textarea class="text-left text-gray-500 w-full" rows=3>
                        {todo_item.get().description}
                    </textarea>
                </div>

                <div class="basis-1/12 flex items-center justify-center">
                   <div class="flex flex-row-reverse space-x-4 space-x-reverse">
                        <button
                            class=button_mod_class
                            on:click=on_edit>
                            <i class="fa-solid fa-edit"></i>
                        </button>
                        <button
                            class=button_del_class
                            on:click=on_delete>
                            <i class="fa-solid fa-minus"></i>
                        </button>
                   </div>
                </div>
          </div>
    }
}
```

todo_modal.rs
```rust
use crate::entities::Todo;
use leptos::html::*;
use leptos::prelude::*;

#[component]
pub fn TodoModal<F>(todo: RwSignal<Todo>, on_close_modal: F) -> impl IntoView
where
    F: Fn(Option<Todo>) + 'static + Copy,
{
    let input_field_class = "shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline";

    let (title, _set_title) = signal(todo.get().title);
    let title_node: NodeRef<Input> = NodeRef::new();

    let (description, _set_description) = signal(todo.get().description);
    let description_node: NodeRef<Textarea> = NodeRef::new();

    let submit = move |_| {
        let title = title_node
            .get()
            .expect("<title> should be mounted")
            .value();

        let description = description_node
            .get()
            .expect("<description> should be mounted")
            .value();

        let mut mod_todo = todo.get().clone();
        mod_todo.title = title;
        mod_todo.description = description;
        on_close_modal(Some(mod_todo));
    };

    let cancel = move |_| {
        on_close_modal(None);
    };

    view! {

    <div class="fixed inset-0 z-50 flex items-center justify-center bg-gray-900 bg-opacity-60">

        <div
          class="block rounded-lg bg-white w-2/5 p-4 shadow-[0_2px_15px_-3px_rgba(0,0,0,0.07),0_10px_20px_-2px_rgba(0,0,0,0.04)] z-70">

            <h5 class="mb-5 text-xl font-medium leading-tight text-neutral-800">
                Create new Todo
            </h5>

                <div class="mb-5">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="title">
                        Title
                    </label>
                    <input
                        node_ref=title_node
                        class=input_field_class
                        id="title"
                        type="text"
                        value=title
                        placeholder="Title"/>
                </div>

                <div class="mb-5">
                    <label class="block text-gray-700 text-sm font-bold mb-2" for="description">
                        Description
                    </label>
                    <textarea
                        node_ref=description_node
                        class=input_field_class
                        rows="3"
                        id="description"
                        placeholder="Description">{
                            description
                        }</textarea>
                </div>

                <div class="flex flex-row-reverse space-x-4 space-x-reverse">
                    <button type="submit"
                        on:click=submit
                        class="bg-blue-700 hover:bg-blue-800 px-5 py-3 text-white rounded-lg">
                        Save
                    </button>
                    <button type="cancel"
                        on:click=cancel
                        class="bg-gray-300 hover:bg-gray-400 px-5 py-3 text-white rounded-lg">
                        Cancel
                    </button>
                </div>
        </div>
    </div>
    }
}
```

app.rs
```rust
use crate::components::*;
use crate::entities::*;
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let show_modal: RwSignal<bool> = RwSignal::new(false);
    let edit_todo_item: RwSignal<Todo> = RwSignal::new(Todo::new_empty());

    let button_new_class = "rounded-full pl-5 pr-5 bg-blue-700 text-white rounded hover:bg-blue-800";

    let todos: RwSignal<Vec<Todo>> = RwSignal::new(Vec::new());

    let add_new_todo = move |x: Todo| {
        edit_todo_item.set(x);
        show_modal.set(true);
    };

    let edit_todo = move |todo: Todo| {
        edit_todo_item.set(todo);
        show_modal.set(true);
    };

    let delete_todo = move |todo: Todo| {
        todos.update(|old| {
            old.retain(|x| x.id != todo.id);
        });
    };

    let close_modal_todo = move |x: Option<Todo>| {
        if let Some(todo) = x {
            todos.update(|old| {
                old.retain(|x| x.id != todo.id);
                old.push(todo);
                old.sort_by(|a, b| a.created.cmp(&b.created));
            });
        }
        show_modal.set(false);
    };

    view! {
        <div class="max-w-md mx-auto mt-10 mt-3 p-5 bg-white rounded-lg shadow-lg">
            <div class="flex justify-between">

                <h1 class="text-4xl font-bold mb-4">Todo List</h1>

                <button on:click={move |_| add_new_todo(Todo::new_empty())} class=button_new_class>
                    <i class="fa-solid fa-plus"></i>
                </button>

            </div>
            <For
                each=move || todos.get()
                key=|state| (state.id.clone(), state.title.clone(), state.description.clone())
                let:child>
                <TodoItem todo=child delete=delete_todo edit=edit_todo/>
            </For>

            <Show when = move || todos.get().is_empty()>
                <div class="flex justify-between">
                    <h2 class="text-2xl font-bold mb-4">Currently no Todos</h2>
                </div>
            </Show>
        </div>

        <Show when = move || show_modal.get()>
            <TodoModal todo=edit_todo_item on_close_modal=close_modal_todo/>
        </Show>

    }
}
```

and include all files in mod.rs
```rust
mod app;
mod todo_item;
mod todo_modal;

pub use app::App;
pub use todo_item::TodoItem;
pub use todo_modal::TodoModal;
```

attaching the mod files in the main.rs
```rust
mod components;
mod entities;

use crate::components::App;
use leptos::prelude::*;

fn main() {
    mount_to_body(App)
}
```

start the service

```shell
trunk serve
```

We have a Todo web application running.

## Tauri

We could apply the command [cargo create-tauri-app] on a new application or [cargo tauri init] on an existing. 

```shell
cargo tauri init
```

with the following configuration:

```text
? What is your app name? › Tauri Todo App
? What should the window title be? › Tauri Todo App
? Where are your web assets (HTML/CSS/JS) located, relative to the "<current dir>/src-tauri/tauri.conf.json" file that will be created? › ../dist
? What is the url of your dev server? › http://localhost:1420
? What is your frontend dev command? · trunk serve
? What is your frontend build command? · trunk build
```

that creates a new module in our project under the folder: src-tauri.

In the main module we add to Cargo.toml
```toml
wasm-bindgen = "0.2"
wasm-bindgen-futures = "0.4"
js-sys = "0.3"
serde = { version = "1", features = ["derive"] }
serde-wasm-bindgen = "0.6"
console_error_panic_hook = "0.1.7"

[workspace]
members = ["src-tauri"]
```

and we add to trunk.toml
```toml
[watch]
ignore = ["./src-tauri"]

[serve]
port = 1420
open = false
```
The change of port is necessary for Tauri, as we defined during the Tauri generation: "What is the url of your dev server: http://localhost:1420".

Start the application with:

```shell
cargo tauri dev
```

We have the first run of the application on the Desktop. 

## Separate Backend / Frontend

The Frontend has limited capabilities, for this we transfer some parts of our application to the Backend, for example to include a communication to a Database. 

We add some dependencies to the main cargo.toml

```toml
[dependencies]
leptos = { version = "0.7.3", features = ["csr"] }
uuid = { version = "1.11.0", features = ["v4", "js"] }
chrono = { version = "0.4.39", features = ["serde", "wasm-bindgen"] }
wasm-bindgen = { version = "0.2.99", features = ["serde"] }
wasm-bindgen-futures = "0.4.49"
web-sys = "0.3.76"
js-sys = "0.3.76"
serde = { version = "1.0.216", features = ["derive"] }
serde-wasm-bindgen = { version = "0.6.5"}
gloo-utils = { version = "0.2.0", features = ["serde"] }
```

and to the cargo.toml in src-tauri:
```toml
[dependencies]
tauri = { version = "2.1.1", features = [] }
tauri-plugin-opener = "2.2.2"
serde = { version = "1.0.216", features = ["derive"] }
serde_json = { version = "1.0.133" }
chrono = { version = "0.4.39", features = ["serde"] }
```

As you can I switch for the timestamps to the chrono library as it had less problems for the serialization.

In the main.rs file from the Frontend we add the #[wasm_bindgen] binders:

```rust
mod components;
mod entities;

use crate::components::App;
use leptos::prelude::*;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn main() {
    mount_to_body(App)
}
```

Two binding methods, to communicate from the Frontend to the Backend.

The Todo Entity get modified:

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TodoJsValue {
    pub todo: Todo
}

impl Todo {
    pub fn new(title: String, description: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let created = Utc::now();
        Todo { id, title, description, created }
    }

    pub fn new_empty() -> Todo {
        Self::new("".to_string(), "".to_string())
    }
    pub fn js_value(&self) -> JsValue {
        let container = TodoJsValue { todo: self.clone() };
        JsValue::from_serde(&container).unwrap()
    }
}
```

The TodoJsValue is a container/wrapper to sent the Todo Data to the Backend by JsValue.

We can now modify the app.rs:

```rust
use chrono::{DateTime, Utc};
use crate::components::*;
use crate::entities::*;
use leptos::prelude::*;
use crate::{invoke, invoke_without_args};
use gloo_utils::format::JsValueSerdeExt;

#[derive(Debug, PartialEq)]
enum Mode {
    ADD,
    EDIT
}

async fn load_data(_trigger: DateTime<Utc>) -> Vec<Todo> {
    let rtn = invoke_without_args("get_todo_list").await;
    let todos = rtn.into_serde::<Vec<Todo>>().unwrap();
    todos
}

#[component]
pub fn App() -> impl IntoView {
    let show_modal: RwSignal<bool> = RwSignal::new(false);
    let show_modal_mode: RwSignal<Mode> = RwSignal::new(Mode::ADD);
    let edit_todo_item: RwSignal<Todo> = RwSignal::new(Todo::new_empty());

    let button_new_class = "rounded-full pl-5 pr-5 bg-blue-700 text-white rounded hover:bg-blue-800";

    let refresh :RwSignal<DateTime<Utc>> = RwSignal::new(Utc::now());
    let fetch_todos = LocalResource::new(move || load_data(refresh.get()));

    let add_new_todo = move |x: Todo| {
        edit_todo_item.set(x);
        show_modal_mode.set(Mode::ADD);
        show_modal.set(true);
    };

    let edit_todo = move |todo: Todo| {
        edit_todo_item.set(todo);
        show_modal_mode.set(Mode::EDIT);
        show_modal.set(true);
    };

    let delete_todo = move |todo: Todo| {
        leptos::task::spawn_local(async move {
            let data = todo.js_value();
            invoke("delete_todo", data).await;
            refresh.set(Utc::now());
        });
    };

    let close_modal_todo = move |x: Option<Todo>| {
        leptos::task::spawn_local(async move {
            if show_modal_mode.read() == Mode::ADD {
                let data = x.unwrap().js_value();
                invoke("add_todo", data).await;
            } else {
                let data = x.unwrap().js_value();
                invoke("edit_todo", data).await;
            }
            refresh.set(Utc::now());
        });
        show_modal.set(false);
    };

    view! {
        <div class="max-w-md mx-auto mt-10 mt-3 p-5 bg-white rounded-lg shadow-lg">
            <div class="flex justify-between">

                <h1 class="text-4xl font-bold mb-4">Todo List</h1>

                <button on:click={move |ev| {
                        ev.prevent_default();
                        add_new_todo(Todo::new_empty())
                        }} class=button_new_class>
                    <i class="fa-solid fa-plus"></i>
                </button>

            </div>

            <Suspense fallback=move || view! { <p>"Loading..."</p> }>

            {move || Suspend::new(async move {
            let todos = fetch_todos.await;
            let todos_is_empty = todos.is_empty();

            view!{
            <For
                each=move || todos.clone()
                key=|state| (state.id.clone(), state.title.clone(), state.description.clone())
                let:child>
                <TodoItem todo=child delete=delete_todo edit=edit_todo/>
            </For>

            <Show when = move || todos_is_empty>
                <div class="flex justify-between">
                    <h2 class="text-2xl font-bold mb-4">Currently no Todos</h2>
                </div>
            </Show>

            }})}
            </Suspense>
        </div>

        <Show when = move || show_modal.get()>
            <TodoModal todo=edit_todo_item on_close_modal=close_modal_todo/>
        </Show>

    }
}
```

The call to the Backend are asynchronous, for this we use spawn_local and call the WASM invoke methods.

Finally in the src-tauri backend part we include the business logic in the lib.rs

```rust
#[tauri::command]
fn get_todo_list(storage: State<Storage>) -> Vec<Todo> {
    println!("Backend:get_todo_list");
    let s = storage.store.lock().unwrap();
    s.to_vec()
}

#[tauri::command]
fn add_todo(todo: Todo, storage: State<Storage>) {
    println!("Backend:add_todo: {:?}", todo);
    let mut s = storage.store.lock().unwrap();
    s.push(todo);
}

#[tauri::command]
fn edit_todo(todo: Todo, storage: State<Storage>) {
    println!("Backend:edit_todo");
    let mut s = storage.store.lock().unwrap();
    s.retain(|x| x.id != todo.id);
    s.push(todo);
}

#[tauri::command]
fn delete_todo(todo: Todo, storage: State<Storage>) {
    println!("Backend:delete_todo");
    let mut s = storage.store.lock().unwrap();
    s.retain(|x| x.id != todo.id);
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: String,
    pub title: String,
    pub description: String,
    pub created: DateTime<Utc>
}

#[derive(Debug)]
struct Storage {
    store: Mutex<Vec<Todo>>,
}
// To evaluate in the future
// #[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Storage { store: Default::default() })
        .invoke_handler(tauri::generate_handler![
            get_todo_list,
            add_todo,
            edit_todo,
            delete_todo,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```
and declare the different commands.

## Final Build

We can now do the final build:

```shell
cargo tauri build
```

and run for your machine the correspondent file: target/release/bundle