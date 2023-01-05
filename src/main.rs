use tracing_subscriber::{self, fmt, prelude::*, EnvFilter};

use event::SmashState;
use calc::Calc;

mod parser;
mod event;
mod calc;

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let calc=Calc::new();    
    SmashState::new(calc).run();
}