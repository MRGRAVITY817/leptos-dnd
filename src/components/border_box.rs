use leptos::*;

#[component]
pub fn BorderBox<F, IV>(cx: Scope, render_prop: F, children: Children) -> impl IntoView
where
    F: Fn() -> IV,
    IV: IntoView,
{
    view! { cx,
        <div
          style="border: 2px dashed orange; \
                 padding: 14px;"
        >
          <h3 style="margin: 0px 0px 12px 0px">"Render prop"</h3>
          <div
            style="border: 1px solid black; \
                   padding: 6px;"
          >
            {render_prop()}
          </div>

          <h3 style="margin: 12px 0px 12px 0px">"Children"</h3>
          <div
            style="border: 1px solid black; \
                   padding: 6px;"
          >
            {children(cx)}
          </div>
        </div>
    }
}
