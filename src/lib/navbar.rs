use leptos::*;

#[component]
pub fn Navbar(cx: Scope) -> impl IntoView {

    
  view! { cx,
    <nav class="rounded-md w-72 min-h-screen flex-col justify-between shadow-lg">
    	<div class=" bg-white h-full">
    		<div class="flex justify-center py-10 shadow-sm pr-4">
          		<img class="aspect-[104/80.1] w-[104px] hidden sm:block" src="/4645_circular_mascot.png"/>
    			<div class="pl-2">
    				<p class="text-2xl font-bold text-blue-900">"The Chicago Style Botdogs"</p>
    				<span class="text-xs block text-red-800">"SCOUTING DASHBOARD"</span>
    			</div>
    		</div>
    		<div class="pl-10">
    			<ul class="space-y-8 pt-10 pb-10">
    				<li class="flex space-x-4 items-center hover:text-indigo-600 cursor-pointer">
    					<svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24"
    						stroke="currentColor">
    						<path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
    							d="M9 17V7m0 10a2 2 0 01-2 2H5a2 2 0 01-2-2V7a2 2 0 012-2h2a2 2 0 012 2m0 10a2 2 0 002 2h2a2 2 0 002-2M9 7a2 2 0 012-2h2a2 2 0 012 2m0 10V7m0 10a2 2 0 002 2h2a2 2 0 002-2V7a2 2 0 00-2-2h-2a2 2 0 00-2 2" />
    					</svg>
    					<a href="">"Dashboard"</a>
    				</li>
    				<li class="flex space-x-4 items-center hover:text-indigo-600 cursor-pointer">
                        <svg class="h-6 w-6" viewBox="0 0 24 24" stroke-width="1.5" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                          <path d="M12 12a4 4 0 100-8 4 4 0 000 8z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                          <path d="M5 20v-1a7 7 0 0110-6.326M16.635 16.415l1.039-2.203a.357.357 0 01.652 0l1.04 2.203 2.323.356c.298.045.416.429.2.649l-1.68 1.713.396 2.421c.051.311-.26.548-.527.401L18 20.812l-2.078 1.143c-.267.147-.578-.09-.527-.4l.396-2.422-1.68-1.713c-.216-.22-.098-.604.2-.65l2.324-.355z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
    					<a href="">"My Team"</a>
    				</li>
    				<li class="flex space-x-4 items-center hover:text-indigo-600 cursor-pointer">
                        <svg class="h-6 w-6" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                          <path d="M20 20H4V4" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                          <path d="M4 16.5L12 9l3 3 4.5-4.5" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
    					<a href="">"Team Statistics"</a>
    				</li>
    				<li class="flex space-x-4 items-center hover:text-indigo-600 cursor-pointer">
                        <svg class="h-6 w-6" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                          <path d="M5 15l.95-10.454A.6.6 0 016.548 4h13.795a.6.6 0 01.598.654l-.891 9.8a.6.6 0 01-.598.546H5zm0 0l-.6 6M9 7.5l7 4" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
    					<a href="">"Competition Statistics"</a>
    				</li>
    				<li class="flex space-x-4 items-center hover:text-indigo-600 cursor-pointer">
                        <svg class="h-6 w-6" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                          <path d="M15 4V2m0 2v2m0-2h-4.5M3 10v9a2 2 0 002 2h14a2 2 0 002-2v-9H3zM3 10V6a2 2 0 012-2h2M7 2v4M21 10V6a2 2 0 00-2-2h-.5" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
    					<a href="">"Match Schedules"</a>
    				</li>
    				<li class="flex space-x-4 items-center hover:text-indigo-600 cursor-pointer">
                        <svg class="h-6 w-6" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                          <path d="M12 15a3 3 0 100-6 3 3 0 000 6z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                          <path d="M19.622 10.395l-1.097-2.65L20 6l-2-2-1.735 1.483-2.707-1.113L12.935 2h-1.954l-.632 2.401-2.645 1.115L6 4 4 6l1.453 1.789-1.08 2.657L2 11v2l2.401.655L5.516 16.3 4 18l2 2 1.791-1.46 2.606 1.072L11 22h2l.604-2.387 2.651-1.098C16.697 18.831 18 20 18 20l2-2-1.484-1.75 1.098-2.652 2.386-.62V11l-2.378-.605z" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
    					<a href="">"App Administration"</a>
    				</li>
    				<li class="flex space-x-4 items-center hover:text-indigo-600 cursor-pointer">
                        <svg class="h-6 w-6" stroke-width="1.5" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg" color="#000000">
                          <path d="M6.755 17.283l-1.429-10A2 2 0 017.306 5h3.388a2 2 0 011.98 2.283l-1.429 10A2 2 0 019.265 19h-.53a2 2 0 01-1.98-1.717z" stroke="#000000" stroke-width="1.5"/>
                          <path d="M2 12h4m16 0H12" stroke="#000000" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
                        </svg>
    					<a href="">"Settings"</a>
    				</li>
    			</ul>
    		</div>
    	</div>
    </nav>
        
  }
}