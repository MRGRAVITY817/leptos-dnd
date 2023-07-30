mod components;

use components::{
    big_or_small::{BigOrSmall, SizeHeader},
    border_box::BorderBox,
    progress_bar::ProgressBar,
    simple_input::SimpleInput,
    todo_list_item::TodoList,
};
use leptos::*;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    provide_context(cx, set_count);

    let handle_click = move |_| {
        set_count.update(|n| *n += 1);
    };

    let double_count = move || count() * 2;

    view! { cx,
        <div style="padding: 12px; display: flex; flex-direction: column; gap: 12px;" >
            <div>
              <button on:click = handle_click
                  class:red = move || count() % 3 == 0
                  class:blue = move || count() % 3 == 1
              >
                "Click me: " {count}
              </button>
              <p>{double_count}</p>
            </div>
            <div>
              <ProgressBar progress=count />
              <ProgressBar progress=Signal::derive(cx, double_count) />
            </div>
            <BigOrSmall value={count} />
            <SizeHeader num={count} />
            <BorderBox render_prop={|| view! {cx, <TodoList />}}>
              <SimpleInput />
            </BorderBox>
        </div>
    }
}

fn main() {
    mount_to_body(|cx| view! {cx, <App />})
}
