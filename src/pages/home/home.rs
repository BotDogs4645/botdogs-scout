use leptos::*;

#[component]
pub fn HomePage(cx: Scope) -> impl IntoView {

    view! { cx,
        <div>
            <Navbar/>
        </div>
    }
}

#[component]
fn Navbar(cx: Scope) -> impl IntoView {

    // let settings_icon: String = "M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z".to_string();

    view! { cx,
        <div class="flex h-screen w-16 flex-col justify-between border-e bg-white"> 
          <div> 
            <div class="inline-flex h-16 w-16 items-center justify-center">
              <span class="grid h-10 w-10 place-content-center rounded-lg bg-gray-100 text-xs text-gray-600">
                "USER HERE" // TODO: fetch the dynamic data from some server function (auth with google)
              </span>
            </div>
        
            <div class="border-t border-gray-100">
              <div class="px-2">
                <div class="px-4">
                // TODO: add link
                  <a href="" class="t group relative flex justify-center rounded px-2 py-1.5 text-blue-500 bg-blue-50">
                    <svg width="800px" height="800px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
                      <path stroke="#000000" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" d="M3 8L15 8M15 8C15 9.65686 16.3431 11 18 11C19.6569 11 21 9.65685 21 8C21 6.34315 19.6569 5 18 5C16.3431 5 15 6.34315 15 8ZM9 16L21 16M9 16C9 17.6569 7.65685 19 6 19C4.34315 19 3 17.6569 3 16C3 14.3431 4.34315 13 6 13C7.65685 13 9 14.3431 9 16Z" />
                    </svg>
                    <span class="absolute start-full top-1/2 ms-4 -translate-y-1/2 rounded bg-gray-900 px-2 py-1.5 text-xs font-medium text-white opacity-0 group-hover:opacity-100">
                      "Admin Settings"
                    </span>
                  </a>
                </div>

                <ul class="space-y-1 border-t border-gray-100 pt-4">
                  <li>
                    // TODO: add link
                    <a href="" class="group relative flex justify-center rounded px-2 py-1.5 text-gray-500 bg-gray-400 hover:bg-blue-600 hover:text-gray-700">
                      <svg>
                        <path stroke-linecap="round" stroke-linejoin="round" d="M17 20h5v-2a3 3 0 00-5.356-1.857M17 20H7m10 0v-2c0-.656-.126-1.283-.356-1.857M7 20H2v-2a3 3 0 015.356-1.857M7 20v-2c0-.656.126-1.283.356-1.857m0 0a5.002 5.002 0 019.288 0M15 7a3 3 0 11-6 0 3 3 0 016 0zm6 3a2 2 0 11-4 0 2 2 0 014 0zM7 10a2 2 0 11-4 0 2 2 0 014 0z" />
                      </svg>
                      <span class="absolute start-full top-1/2 ms-4 -translate-y-1/2 rounded bg-gray-900 px-2 py-1.5 text-xs font-medium text-white opacity-0 group-hover:opacity-100">
                        "Something"
                      </span>
                    </a>
                  </li>
                </ul>

              </div>
            </div>
          </div>
        </div> 
    }
}