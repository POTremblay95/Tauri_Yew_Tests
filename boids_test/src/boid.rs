use std::fmt::Writer;
use std::iter;

use rand::Rng;
use yew::{html, Html};

use crate::math::{self, Mean, Vector2D, WeightedMean};
use crate::settings::Settings;
use crate::simulation::SIZE;
