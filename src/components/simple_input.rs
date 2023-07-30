use leptos::{ev::SubmitEvent, html::Input, *};

#[component]
pub fn SimpleInput(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, "Frank".to_string());

    let input_element: NodeRef<Input> = create_node_ref(cx);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();

        set_name(value);
    };

    view! { cx,
      <h2>Uncontrolled Component</h2>

      <form on:submit=on_submit>
        <input type="text" value=name node_ref=input_element />
        <input type="submit" value="Submit" />
      </form>
    }
}
