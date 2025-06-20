use crate::app::Context;
use notify_rust::Notification;
use std::io::Read;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use tokio::time;

#[derive(Debug, Default)]
pub struct CliApplication {}

impl CliApplication {
    fn notify_work_done() {
        let _ = Notification::new()
            .summary("Pomodoro time's up!")
            .body("Let's reward you with a break")
            .show();
    }

    fn notify_break_done() {
        let _ = Notification::new()
            .summary("Break's up!")
            .body("Let's get back on track!")
            .show();
    }

    pub async fn run(&self, ctx: Arc<Context>) {
        let work_time = time::Duration::from_mins(ctx.work_time);
        let break_time = time::Duration::from_mins(ctx.break_time);

        loop {
            let start = time::Instant::now();

            {
                let mut lock = ctx.last_start.lock().await;
                *lock = start;
            }

            // TODO User input here blocks the entire execution and as such also the select,
            // meaning that even if we press ctrl+c while the input request is up, it won't end.
            // only after will the block stop, and the relevant flag will be set, resulting in
            // exiting

            ctx.working.store(true, Ordering::Relaxed);
            time::sleep(work_time).await;

            ctx.working.store(false, Ordering::Relaxed);
            CliApplication::notify_work_done();

            let _ = ctx
                .elapsed
                .fetch_add(start.elapsed().as_secs() as usize, Ordering::Relaxed);

            println!("Press key to continue");
            let _ = std::io::stdin().read(&mut [0]).unwrap();

            time::sleep(break_time).await;
            CliApplication::notify_break_done();

            println!("Press key to continue");
            let _ = std::io::stdin().read(&mut [0]).unwrap();
        }
    }
}
