use yew::prelude::*;
//use yewprint::Text;

/// Home page
pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Home {}
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="app">
                <header class="app-header">

                <div class="flex-container">

                    <div class="flex-item">
                        <div class="fadeIn">
                            <p>{"Hi there 👋 "}</p>
                            <p>{"Thomas"}</p>
                        </div>
                    </div>

                    <div class="flex-item">
                        <div class="fadeIn">
                            <p>{"🌱 I’m currently learning Rust, WebAssembly, Kotlin, Godot"}</p>

                            <p>{"📚 I'm well versed in JavaScript, TypeScript, Java, React, Frontend, RESTful APIs."}</p>
                        </div>
                    </div>
                
                </div>
                </header>
            </div>
        }
    }
}
