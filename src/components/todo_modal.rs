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