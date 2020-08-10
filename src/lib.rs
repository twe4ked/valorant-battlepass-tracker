#![recursion_limit = "256"]

use js_sys::Date;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::web_sys::HtmlInputElement as InputElement;

struct Model {
    node_ref: NodeRef,
    link: ComponentLink<Self>,
    levels: Vec<usize>,
    level: String,
    total_xp: usize,
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
}

enum Msg {
    Update(String),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut levels = Vec::new();
        levels.push(0); // Level 0
        levels.push(0); // Level 1
        let mut xp = 4000;
        for _ in 2..=50 {
            levels.push(xp);
            xp += 1000;
        }

        let total_xp = levels.iter().sum::<usize>();

        Self {
            link,
            levels,
            level: "15".to_string(),
            total_xp,
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
        let cumulative_xp = self.levels[0..=self.level()].iter().sum::<usize>();
        let percent_complete = (cumulative_xp as f32 / self.total_xp as f32) * 100.0;

        // Act 2 start and end dates
        let start = Date::parse("2020-08-04");
        let end = Date::parse("2020-10-12");

        let now = Date::now();
        let date_percent_complete = ((now - start) / (end - start)) * 100.0;

        html! {
            <div class="container">
                <label for="level" title={ format!("{} XP", self.levels[self.level()]) }>
                    { format!("Level ({}) - {} (Cumulative) / {} XP", self.level(), cumulative_xp, self.total_xp) }
                </label>
                <input type="text"
                    value=&self.level
                    ref=self.node_ref.clone()
                    id="level"
                    oninput=self.link.callback(|e: InputData| Msg::Update(e.value)) />

                <div class="progress-bar">
                    <div style={ format!("background: linear-gradient(90deg, #a66fed {}%, #f0cdf5 {}%);", percent_complete, percent_complete) }>
                        <span>{ format!("Percent complete: {:.1}%", percent_complete) }</span>
                    </div>
                    <div style={ format!("background: linear-gradient(90deg, #754da8 {}%, #d196d9 {}%);", date_percent_complete, date_percent_complete) }>
                        { format!("Percent time complete: {:.1}% (ends 2020-10-12)", date_percent_complete) }
                    </div>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
