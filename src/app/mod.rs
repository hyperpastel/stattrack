pub mod cli;
pub mod gui;

use cli::CliApplication;
use gui::GuiApplication;
use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize},
};
use tokio::{sync::Mutex, time};

#[derive(Debug)]
pub struct Context {
    pub work_time: u64,
    pub break_time: u64,

    pub elapsed: AtomicUsize,
    pub last_start: Mutex<time::Instant>,

    pub working: AtomicBool,
}

pub enum Application {
    Cli(CliApplication),
    Gui(GuiApplication),
}

impl Application {
    pub async fn run(&mut self, ctx: Arc<Context>) {
        match self {
            Application::Cli(app) => app.run(ctx).await,
            Application::Gui(app) => app.run(ctx).await,
        }
    }

    pub fn cli() -> Application {
        Application::Cli(cli::CliApplication::default())
    }

    pub fn gui() -> Application {
        Application::Gui(gui::GuiApplication::default())
    }
}
