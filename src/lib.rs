#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod article;
mod feed;
mod opml;
mod utils;
mod view;
mod widget;
pub use app::RSSucks;
pub mod renderer;
