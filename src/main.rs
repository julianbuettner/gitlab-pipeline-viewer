mod config;
mod emoji;
mod git;
mod gitlabbing;
mod render;
use crossterm::{execute, terminal};
use std::io::stdout;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use std::{thread, time};

fn abortable_sleep(dur: Duration, repetitions: usize, running: &Arc<AtomicBool>) {
    let mut i = 0;
    while running.load(Ordering::SeqCst) && i < repetitions {
        thread::sleep(dur);
        i += 1;
    }
}

fn main() {
    let conf = config::read_config();
    if conf.is_err() {
        println!("Could not read config file: {}", conf.err().unwrap());
        return;
    }
    let conf = conf.unwrap();

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("Quit");
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    execute!(stdout(), terminal::EnterAlternateScreen)
        .expect("Your terminal does not support alternate screens.");

    while running.load(Ordering::SeqCst) {
        let repo = git::get_local_repository(&conf);

        if repo.is_err() {
            render::render_error(repo.err().unwrap());
        } else {
            let stuff = gitlabbing::get_gitlab_pipelines(&repo.unwrap(), &conf);
            if stuff.is_err() {
                render::render_error(stuff.err().unwrap());
            } else {
                let lines = render::render(&stuff.unwrap());
                render::clear_screen();
                for l in lines {
                    println!("{}", l);
                }
            }
        }
        let duration = time::Duration::from_millis(50);
        let repetitions = conf.cooldown.unwrap_or(5.0) / 0.05;
        abortable_sleep(duration, repetitions.floor().abs() as usize, &running);
    }

    execute!(stdout(), terminal::LeaveAlternateScreen)
        .expect("Your terminal does not support alternate screens.");
}
