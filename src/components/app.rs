use chrono::{DateTime, Utc};
use crate::components::*;
use crate::entities::*;
use leptos::prelude::*;
use leptos::task::spawn_local;
use crate::{invoke, invoke_without_args};
use gloo_utils::format::JsValueSerdeExt;
use crate::components::app::Mode::{ADD, EDIT};

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
    let show_modal_mode: RwSignal<Mode> = RwSignal::new(ADD);
    let edit_todo_item: RwSignal<Todo> = RwSignal::new(Todo::new_empty());

    let button_new_class = "rounded-full pl-5 pr-5 bg-blue-700 text-white rounded hover:bg-blue-800";

    let refresh :RwSignal<DateTime<Utc>> = RwSignal::new(Utc::now());
    let fetch_todos = LocalResource::new(move || load_data(refresh.get()));

    let add_new_todo = move |x: Todo| {
        edit_todo_item.set(x);
        show_modal_mode.set(ADD);
        show_modal.set(true);
    };

    let edit_todo = move |todo: Todo| {
        edit_todo_item.set(todo);
        show_modal_mode.set(EDIT);
        show_modal.set(true);
    };

    let delete_todo = move |todo: Todo| {
        spawn_local(async move {
            let data = todo.js_value();
            invoke("delete_todo", data).await;
            refresh.set(Utc::now());
        });
    };

    let close_modal_todo = move |x: Option<Todo>| {
        spawn_local(async move {
            if show_modal_mode.read() == ADD {
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