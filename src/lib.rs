#![deny(clippy::pedantic)]

pub mod gui;
pub mod shell;
pub mod subnautica;

pub use gui::app;

const APP_NAME: &str = "Subnautica Base Planner";
