use leptos::*;

#[component]
pub fn BigOrSmall(cx: Scope, value: ReadSignal<i32>) -> impl IntoView {
    view! { cx,
        <Show
          when={move || { value() > 5 }}
          fallback={move |cx| view! { cx, <Small num={value} /> }}
        >
          <Big num={value} />
        </Show>
    }
}

#[component]
fn Big(cx: Scope, num: ReadSignal<i32>) -> impl IntoView {
    let setter = use_context::<WriteSignal<i32>>(cx).expect("to have count setter");

    view! { cx,
        <h1>This is big {num}</h1>
        <button on:click=move |_| setter.update(|value| *value += 2)>"Add count"</button>
    }
}

#[component]
fn Small(cx: Scope, num: ReadSignal<i32>) -> impl IntoView {
    view! { cx,
        <p>This is small {num}</p>
    }
}

#[component]
pub fn SizeHeader(cx: Scope, num: ReadSignal<i32>) -> impl IntoView {
    view! { cx,
        {move || match num() % 4 {
            0 => view! { cx, <h1>"Header "{num}</h1>}.into_any(),
            1 => view! { cx, <h2>"Header "{num}</h2>}.into_any(),
            2 => view! { cx, <h3>"Header "{num}</h3>}.into_any(),
            _ => view! { cx, <h4>"Header "{num}</h4>}.into_any(),
        }}
    }
}
