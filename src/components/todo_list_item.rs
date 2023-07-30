use leptos::*;

#[component]
pub fn TodoList(cx: Scope) -> impl IntoView {
    let initial_todo_list = vec![
        "Clean my room".to_string(),
        "Dry my hair".to_string(),
        "Have a video call with ML team".to_string(),
        "Analyze food dataset".to_string(),
    ];

    let (todo_list, set_todo_list) = create_signal(cx, initial_todo_list);

    let (todo_input, set_todo_input) = create_signal(cx, "".to_string());

    let add_todo_item = move |_| {
        set_todo_list.update(|todo_list| todo_list.push(todo_input.get()));
        set_todo_input("".to_string())
    };

    view! { cx,
        <input
          placeholder="Create new TODO..."
          prop:value = todo_input
          on:input = move |e| set_todo_input(event_target_value(&e))
        />
        <button
          on:click = add_todo_item
        >
          Add
        </button>
        <ul>
          <For
            each = todo_list
            key = |todo| todo.to_owned()
            view = move |cx, todo| {
              view! { cx,
                <li>{todo}</li>
              }
            }
          />
        </ul>
    }
}
