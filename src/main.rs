mod settings;
mod state;
mod tomata;
mod widget;

use druid::{AppLauncher, PlatformError, WindowDesc};

use widget::TomataApp;
use state::TomataState;

fn main() -> Result<(), PlatformError> {
    let window = WindowDesc::new(TomataApp::new)
        .title("tomata")
        .window_size((250., 100.))
        .resizable(false);
    let state = TomataState::default();
    AppLauncher::with_window(window).launch(state)?;
    Ok(())
}
