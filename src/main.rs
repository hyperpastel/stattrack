#![feature(duration_constructors_lite)]

use std::sync::{
    Arc,
    atomic::{AtomicBool, AtomicUsize, Ordering},
};

use app::{Application, Context};
use tokio::{sync::Mutex, time};

mod app;
mod command;
mod projects;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = command::build_cli().get_matches();

    let work_time: u64 = *args.get_one("work-time").unwrap();
    let break_time: u64 = *args.get_one("break-time").unwrap();
    let project_name = args.get_one::<String>("project-name").unwrap().clone();

    let elapsed = AtomicUsize::new(0);
    let last_start = Mutex::new(time::Instant::now());
    let working = AtomicBool::new(false);

    let ctx = Context {
        work_time,
        break_time,
        elapsed,
        last_start,
        working,
    };

    let ctx = Arc::new(ctx);
    let cctx = Arc::clone(&ctx);

    let terminate = Arc::new(AtomicBool::new(false));
    let cterminate = Arc::clone(&terminate);

    ctrlc_async::set_async_handler(async move {
        terminate.store(true, Ordering::SeqCst);

        let mut elapsed = cctx.elapsed.load(Ordering::Relaxed);

        // if we are cancelling while not on break, we have to add the time so far too
        if cctx.working.load(Ordering::Acquire) {
            let elapsed_at_stop = cctx.last_start.lock().await.elapsed().as_secs();
            elapsed += elapsed_at_stop as usize;
        }

        let Ok(mut projects) = projects::load_projects() else {
            return;
        };

        projects
            .entry(project_name)
            .and_modify(|f| *f += elapsed)
            .or_insert(elapsed);

        let _ = projects::write_projects(&projects);
    })?;

    // For now, we just use the CLI version, but we will allow to choose between GUI later
    let dummy = true;

    let mut app = if dummy {
        Application::cli()
    } else {
        Application::gui()
    };

    tokio::select! {
        _ = async {
            app.run(ctx).await
        }=> {},
        _ = async {
            let mut check_interval = time::interval(time::Duration::from_millis(100));
            while !cterminate.load(Ordering::Relaxed) {
                check_interval.tick().await;
            }
        } => {},
    };

    Ok(())
}
