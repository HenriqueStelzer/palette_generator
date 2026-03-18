mod palette;

use leptos::prelude::*;

fn main() {
    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view ! {

        <main>
            <div class="flex flex-col align-middle pt-12">
                <div class="text-center">
                    <p class="text-6xl font-mono">Color Palette Generator</p>
                </div>

                <div class="flex justify-between mx-16 my-16">
                    <div class="bg-amber-400 py-64 px-96 rounded-2xl overflow-hidden -mx-42">
                        <p>Teste 1</p>
                    </div>
                    <div class="bg-amber-400 py-64 px-96 rounded-2xl overflow-hidden -mx-42">
                        <p>Teste 2</p>
                    </div>
                    <div class="bg-amber-400 py-64 px-96 rounded-2xl overflow-hidden -mx-42">
                        <p>Teste 3</p>
                    </div>
                </div>
            </div>
        </main>
    }
}