use tracing::info;
use web_time::{Duration, Instant};

// std::time::Instant is not available on wasm32-unknown-unknown targets at time of writing.
// The web_time crate provides drop-in replacements for std::time functionality on platforms that
// do not natively support these functions, like wasm32-unknown-unknown. On other platforms it
// behaves as a transparent wrapper around std::time and therefore adds no overhead beyond an
// additional dependency.

pub(crate) struct FrameCounter {
    total_frames: u64,
    last_update_time: web_time::Instant,
    last_update_frame: u64,
    last_frames_per_second: f32,
    last_frame_time_ms: f32,
}

pub(crate) struct FrameStatistics {
    pub frames_per_second: f32,
    pub average_frame_time_ms: f32,
}

const FRAME_PRINT_INTERVAL: Duration = Duration::from_secs(1);

impl FrameCounter {
    pub(crate) fn new() -> Self {
        Self {
            total_frames: 0,
            last_update_time: Instant::now(),
            last_update_frame: 0,
            last_frames_per_second: 0.0,
            last_frame_time_ms: 0.0,
        }
    }

    pub(crate) fn statistics(&self) -> FrameStatistics {
        FrameStatistics {
            frames_per_second: self.last_frames_per_second,
            average_frame_time_ms: self.last_frame_time_ms,
        }
    }

    pub(crate) fn print_frame_statistics(&mut self) {
        let statistics = self.statistics();
        let frame_time_ms = statistics.average_frame_time_ms;
        let frames_per_second = statistics.frames_per_second;
        info!("Average frame time: {frame_time_ms:.2}ms ({frames_per_second:.2}fps)");
    }

    pub(crate) fn count_frame(&mut self) {
        self.total_frames += 1;
        let now = web_time::Instant::now();
        let elapsed = now.duration_since(self.last_update_time);
        if now.duration_since(self.last_update_time) >= FRAME_PRINT_INTERVAL {
            let elapsed_seconds = elapsed.as_secs_f32();
            let frames_since_last_update = self.total_frames - self.last_update_frame;
            self.last_frames_per_second = frames_since_last_update as f32 / elapsed_seconds;
            self.last_frame_time_ms = elapsed_seconds * 1000.0 / frames_since_last_update as f32;
            self.print_frame_statistics();
            self.last_update_time = now;
            self.last_update_frame = self.total_frames;
        }
    }
}
