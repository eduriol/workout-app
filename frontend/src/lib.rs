use anyhow::Error;
use serde_derive::Deserialize;
use wasm_bindgen::prelude::*;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

#[derive(Deserialize, Debug)]
pub struct Exercise {
    id: i32,
    name: String,
    sets: i32,
    max_reps: i32,
    workout_id: Option<i32>,
    min_reps: i32,
}

#[derive(Deserialize, Debug)]
pub struct Workout {
    pub id: i32,
    pub muscular_group: String,
}

type WorkoutList = Vec<(Workout, Vec<Exercise>)>;

pub enum Msg {
    FetchReady(Result<WorkoutList, Error>),
    Ignore,
}

pub struct Model {
    link: ComponentLink<Model>,
    workouts_list: Option<WorkoutList>,
    ft: Option<FetchTask>,
}

impl Model {
    fn view_data(&self) -> Html {
        if let Some(list) = &self.workouts_list {
            html! {
                <ul style="list-style: none;">{ for list.iter().map(|w| self.view_workout(&w)) }</ul>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }

    fn view_workout(&self, (w, exercises): &(Workout, Vec<Exercise>)) -> Html {
        html! {
            <li>
                <h4>{ w.muscular_group.as_str() }</h4>
                <ul class="list-group">{ for exercises.iter().map(|e| self.view_exercise(&e)) }</ul>
                <br/>
            </li>
        }
    }

    fn view_exercise(&self, e: &Exercise) -> Html {
        html! {
            <li class="list-group-item">
                {
                    e.name.as_str().to_owned() +
                    " ("  + &e.sets.to_string() +
                    " x " + &e.min_reps.to_string() +
                    "-" + &e.max_reps.to_string() +
                    ")"
                }
            </li>
        }
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut model = Model {
            link,
            workouts_list: None,
            ft: None,
        };
        let callback = model.link.callback(
            move |response: Response<Json<Result<WorkoutList, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                if meta.status.is_success() {
                    Msg::FetchReady(data)
                } else {
                    Msg::Ignore // FIXME: Handle this error accordingly.
                }
            },
        );
        let request = Request::get("http://localhost:8000/workouts")
            .body(Nothing)
            .unwrap();
        let task = FetchService::fetch(request, callback).unwrap();
        model.ft = Some(task);

        model
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchReady(response) => {
                self.workouts_list = response.map(|data| data).ok();
            }
            Msg::Ignore => {
                return false;
            }
        }
        true
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <main class="container" role="main">
                <h2 class="d-flex align-items-center p-3 my-3 rounded shadow-sm">{ "Your Workouts:" }</h2>
                { self.view_data() }
            </main>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}
