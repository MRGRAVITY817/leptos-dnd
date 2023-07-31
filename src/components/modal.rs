use leptos::*;

#[component]
pub fn Modal(cx: Scope) -> impl IntoView {
    view! { cx,
      <button popovertarget="mymodal" style="margin: 12px; width: 72px; height: 72px">"Toggle modal"</button>

      <div
        style="background-color: #eeeeee; \
               border-radius: 6px; \
               box-shadow: 3px; \
               padding: 12px;"
        class="mymodal"
        id="mymodal"
        popover
      >
        <h1>"This is a modal"</h1>
        <p>
          "The Popover API provides developers with a standard, consistent, \
           flexible mechanism for displaying popover content on top of other page content."
          <br />
          "Popover content can be controlled either declaratively using HTML attributes, or via JavaScript."
        </p>
      </div>
    }
}
