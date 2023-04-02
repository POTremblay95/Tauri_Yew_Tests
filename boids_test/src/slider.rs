use std::cell::Cell; //To have immutable container with mutable value inside

use web_sys::HtmlInputElement;
use yew::events::InputEvent;
use yew::{html, Callback, Component, Context, Html, Properties, TargetCast};

thread_local! {
    static SLIDER_ID: Cell<usize> = Cell::default();
}

fn next_slider_id() -> usize {
    SLIDER_ID.with(|cell| cell.replace(cell.get() + 1))
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    pub label: &'static str,
    pub value: f64,
    pub onchange: Callback<f64>,
    #[prop_or_default]
    pub precision: Option<usize>,
    #[prop_or_default]
    pub percentage: bool,
    #[prop_or_default]
    pub min: f64,
    pub max: f64,
    #[prop_or_default]
    pub step: Option<f64>,
}

pub struct Slider {
    id: usize,
}

impl Component for Slider {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            id: next_slider_id(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        unimplemented!()
    }

    fn view(&self)
}
