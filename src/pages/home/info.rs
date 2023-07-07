
use leptos::*;

#[component]
pub fn InfoContainer(
  cx: Scope,
  children: Children
) -> impl IntoView {


    view! { cx,
        <div class="flex flex-wrap justify-evenly gap-2 grid-cols-3 overflow-y-auto">
          {children(cx)}
        </div>
    }    
}

#[component]
pub fn InfoCard<F, IV>(
  cx: Scope,
  title: &'static str,
  data: F
) -> impl IntoView where
  F: Fn() -> IV,
  IV: IntoView  {

  view! { cx,
    <div class="bg-red-400 rounded-lg w-auto h-80 ">
      <p class="text-center p-1">{title}</p>
      {data()}
    </div>
  }
}