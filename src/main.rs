use enigo::*;
use rdev::{listen, Event, EventType};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const RESET_THRESHOLD: Duration = Duration::from_millis(500);

fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let last_direction = Arc::new(AtomicBool::new(true)); // true = up, false = down
    let last_scroll_time = Arc::new(AtomicU64::new(0));

    let callback = move |event: Event| {
        match event.event_type {
            EventType::Wheel { delta_x, delta_y } => {
                let current_time = Instant::now();
                let current_direction = delta_y > 0;
                let last_time = Instant::now()
                    .checked_sub(Duration::from_nanos(
                        last_scroll_time.load(Ordering::SeqCst),
                    ))
                    .unwrap_or(Instant::now());

                // if !current_direction {
                //     return;
                // }

                // println!(
                //     "current_time: {:?}, last_time: {:?}, current_direction: {}, last_direction: {}",
                //     current_time,
                //     last_time,
                //     current_direction,
                //     last_direction.load(Ordering::SeqCst)
                // );

                if current_time.duration_since(last_time) > RESET_THRESHOLD {
                    // Reset direction if inactive for the threshold duration
                    last_direction.store(current_direction, Ordering::SeqCst);
                } else if current_direction != last_direction.load(Ordering::SeqCst) {
                    // Corrected scroll event
                    enigo
                        .scroll(if current_direction { 1 } else { -1 }, Axis::Vertical)
                        .expect("Could not scroll");
                }

                last_scroll_time.store(current_time.elapsed().as_nanos() as u64, Ordering::SeqCst);
            }
            _ => (),
        }
    };

    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}
