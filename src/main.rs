mod palette;

use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view ! {

        <main>
            <div class="flex align-middle justify-center">
                <div>
                    <p class="text-8xl ">Hello  World!</p>
                </div>
                 
            </div>
        </main>
    }
}