#![recursion_limit = "256"]

use js_sys::Date;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys::HtmlInputElement as InputElement;

struct Model {
    node_ref: NodeRef,
    link: ComponentLink<Self>,
    level: String,
}

impl Model {
    fn level(&self) -> usize {
        let value = self.level.parse::<usize>().unwrap_or(0);
        if value <= 50 {
            value
        } else {
            0
        }
    }

    fn xp_for_level(&self, level: usize) -> usize {
        if level >= 2 {
            (level + 2) * 1000
        } else {
            0
        }
    }

    fn total_xp(&self) -> usize {
        self.cumulative_xp_to_level(50)
    }

    fn cumulative_xp_to_level(&self, level: usize) -> usize {
        (0..=level).map(|l| self.xp_for_level(l)).sum::<usize>()
    }
}

enum Msg {
    Update(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            level: "15".to_string(),
            node_ref: NodeRef::default(),
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            if let Some(input) = self.node_ref.cast::<InputElement>() {
                let _ = input.focus();
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(level) => self.level = level,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let cumulative_xp = self.cumulative_xp_to_level(self.level());
        let percent_complete = (cumulative_xp as f32 / self.total_xp() as f32) * 100.0;

        // Act 3 start and end dates
        let start = Date::parse("2020-10-13");
        let end = Date::parse("2021-01-11");

        let now = Date::now();
        let date_percent_complete = ((now - start) / (end - start)) * 100.0;

        html! {
            <>
                <h1>{ "Valorant Battlepass Tracker" }</h1>
                <div class="container">
                    <label for="level" title={ format!("{} XP", self.xp_for_level(self.level())) }>
                        { format!("Level ({}) - {} (Cumulative) / {} XP", self.level(), cumulative_xp, self.total_xp()) }
                    </label>
                    <input type="number"
                        min=0
                        max=50
                        inputmode="numeric"
                        value=&self.level
                        ref=self.node_ref.clone()
                        id="level"
                        placeholder="Your current level"
                        oninput=self.link.callback(|e: InputData| Msg::Update(e.value)) />

                    <div class="progress-bar">
                        <div style={ format!("background: linear-gradient(90deg, #a66fed {}%, #f0cdf5 {}%);", percent_complete, percent_complete) }>
                            <span>{ format!("Percent complete: {:.1}%", percent_complete) }</span>
                        </div>
                        <div style={ format!("background: linear-gradient(90deg, #754da8 {}%, #d196d9 {}%);", date_percent_complete, date_percent_complete) }>
                            { format!("Percent time complete: {:.1}% (ends 2021-01-11)", date_percent_complete) }
                        </div>
                    </div>
                </div>
                <footer>
                    <p>{ "Made by " } <a href="https://odindutton.com">{ "Odin" }</a></p>
                </footer>
            </>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
