use leptos::*;
use leptos_dnd::components::todo_list::TodoList;
use wasm_bindgen_test::*;
use web_sys::HtmlElement;

wasm_bindgen_test_configure!(run_in_browser);

/// Simple wrapper for cleaning each tests.
#[wasm_bindgen_test]
fn clear() {
    let document = leptos::document();
    let test_wrapper: HtmlElement = document.create_element("section").into();
    document.body().unwrap().append_child(&test_wrapper);

    // Start by rendering our todo list
    mount_to(
        test_wrapper.clone().try_into().unwrap(),
        |cx| view! { cx, <TodoList />},
    );
}
