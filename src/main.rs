fn main() {
    use dioxus::desktop::{Config, LogicalPosition, LogicalSize, WindowBuilder};
    use dioxus::prelude::*;

    LaunchBuilder::desktop()
        .with_cfg(desktop! {
            Config::new()
                .with_menu(None)
                .with_window(
                    WindowBuilder::new()
                        .with_title("Crab Paperwork")
                        .with_inner_size(LogicalSize::new(1280.0, 820.0))
                        .with_min_inner_size(LogicalSize::new(900.0, 560.0))
                        .with_position(LogicalPosition::new(40.0, 40.0))
                        .with_always_on_top(false)
                )
        })
        .launch(crab_paperwork::app::App);
}
