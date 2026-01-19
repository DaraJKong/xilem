//! This minimal example demonstrates how changing many properties make build times awfully slow.
//!
//! 1. Start by building this example normally. The build speed here is okay, since it's the first build.
//!
//! 2. Try modifying the padding of the button and then rebuild with the command `cargo build --example slow_build_times --timings`.
//!    The incremental rebuild takes forever considering the simplicity of this example!
//!
//! 3. Try commenting out all properties (lines 27-33) to see the difference.
//!    Removing properties gives a 10x speedup, insane! The worst is to come though.
//!
//! 4. Try uncommenting all properties in addition to uncommenting the line 34 (`.color(css::WHITE)`).
//!    The `Button` view does not implement `HasProperty<ContentColor>`, so building here will fail. However, for the sake of the exercise,
//!    try to pay attention to your editor's LSP suddenly slowing down after uncommenting the line.
//!    
//!    Now try running the same build command as before with the `--timings` argument. What will happen is that rustc will be too busy to
//!    catch the error early. Instead, it takes forever (40x slower) for the compilation to fail and give feedback. This is really bad for
//!    the developer's feedback loop. In my experience, this issue makes styling an absolute nightmare, even on very good hardware. In more
//!    complex apps, I often hit a wall where the incremental builds take multiple minutes to finish or give an error!

use xilem::{
    EventLoop, WidgetView, WindowOptions, Xilem, core::Edit, masonry::layout::AsUnit, palette::css,
    style::Style, view::text_button, winit::error::EventLoopError,
};

fn app_logic(count: &mut i32) -> impl WidgetView<Edit<i32>> + use<> {
    text_button(format!("{}", count), |count: &mut i32| *count += 1)
        .width(200.px())
        .padding(20.)
        .corner_radius(100.)
        .background_color(css::BLUE)
        .border_color(css::TRANSPARENT)
        .hovered_border_color(css::WHITE)
        .active_background_color(css::SKY_BLUE)
    // .color(css::WHITE)
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(0, app_logic, WindowOptions::new("Counter"));
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}
