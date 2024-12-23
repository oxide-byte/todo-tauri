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