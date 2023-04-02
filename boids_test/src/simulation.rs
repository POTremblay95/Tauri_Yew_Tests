use gloo::timers::callback::Interval;
use yew::{html, Component, Context, Html, Properties};

use crate::boid::Boid;
use crate::math::Vector2D;
use crate::settings::Settings;

pub const SIZE: Vector2D = Vector2D::new(1600.0, 1000.0);
