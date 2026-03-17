use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view ! {
        <main>
            <p class="text-8xl ">Hello  World!</p>
        </main>
    }
}