mod app;
mod export;
mod markdown;
mod platform;
mod storage;
mod ui;

fn main() {
    dioxus::launch(app::App);
}
