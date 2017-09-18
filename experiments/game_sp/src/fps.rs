
use std::time::{Instant, Duration};
use std::thread;

pub struct FpsTimer {
    // fps: f32,
    epoch: Instant,
    tick: Instant,
    frames: u64,
    fps_as_ns: f64,
    last_sec: u64,
    last_fps: u32,
    frame_time: Duration,
}

impl FpsTimer {
    pub fn new(fps: f32) -> FpsTimer {
        FpsTimer {
            // fps: fps,
            epoch: Instant::now(),
            tick: Instant::now(),
            frames: 0,
            frame_time: Duration::new(0,0),
            fps_as_ns: (1.0 / fps as f64) * 1_000_000_000.0,
            last_sec: 0,
            last_fps: 0,
        }
    }

    pub fn tick(&mut self) {
        // update current time
        let now = Instant::now();
        self.frame_time = now - self.tick;
        self.tick = now;
        // update FPS
        let sec = self.epoch.elapsed().as_secs();
        if sec > self.last_sec {
            self.last_fps = self.frames as u32;
            self.frames = 0;
            self.last_sec = sec;
        } else {
            self.frames += 1;
        }
    }

    pub fn get_frame_time(&self) -> Duration {
        self.frame_time
    }

    pub fn get_epoch(&self) -> Instant {
        self.epoch
    }

    pub fn get_last_fps(&self) -> u32 {
        self.last_fps
    }

    pub fn sleep_til_next_tick(&mut self) {
        let t = self.tick.elapsed();
        let frame_time = t.as_secs() * 1_000_000_000 + t.subsec_nanos() as u64;
        let diff = self.fps_as_ns - frame_time as f64;
        if diff > 0.0 {
            thread::sleep(Duration::new(0, diff as u32));
        }
    }
}
