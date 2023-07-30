use std::time::Duration;
#[cfg(not(test))]
use std::time::Instant;

use clip_mash_types::Progress;
use float_cmp::approx_eq;
#[cfg(test)]
use mock_instant::Instant;

pub struct ProgressTracker {
    work_total: f64,
    work_done: f64,
    started_at: Instant,
    message: String,
}

impl ProgressTracker {
    pub fn new(work_todo: f64) -> Self {
        ProgressTracker {
            work_done: 0.0,
            started_at: Instant::now(),
            work_total: work_todo,
            message: String::new(),
        }
    }

    /// Increment work done by a given amount.
    pub fn inc_work_done_by(&mut self, units: f64, message: &str) {
        self.work_done += units;
        self.message = message.into();
    }

    pub fn eta(&self) -> Duration {
        if self.work_done == 0.0 || self.work_total == 0.0 || self.work_total <= self.work_done {
            return Duration::ZERO;
        }
        let work_not_done = self.work_total - self.work_done;
        let not_done_to_done_ratio = work_not_done / self.work_done;
        let seconds_since_start = Instant::now() - self.started_at;
        let eta_seconds = not_done_to_done_ratio * seconds_since_start.as_secs_f64();

        assert!(
            eta_seconds.is_finite(),
            "eta_seconds is NaN or infinite: {}",
            eta_seconds
        );

        Duration::from_secs_f64(eta_seconds)
    }

    pub fn progress(&self) -> Progress {
        Progress {
            items_finished: self.work_done,
            items_total: self.work_total,
            eta_seconds: self.eta().as_secs_f64(),
            done: self.work_total != 0.0
                && (approx_eq!(f64, self.work_done, self.work_total, epsilon = 0.01)
                    || self.work_done >= self.work_total),
            message: self.message.clone(),
        }
    }
}

#[cfg(test)]
mod test {
    use std::time::Duration;

    use float_cmp::assert_approx_eq;
    use mock_instant::MockClock;

    use crate::progress::ProgressTracker;

    #[test]
    fn test_progress_tracker_eta() {
        let mut tracker = ProgressTracker::new(100.0);
        tracker.inc_work_done_by(10.0, "");
        MockClock::advance(Duration::from_secs(1));
        let progress = tracker.progress();
        assert_eq!(10.0, progress.items_finished);
        assert_eq!(9.0, progress.eta_seconds);

        MockClock::advance(Duration::from_secs(2));
        tracker.inc_work_done_by(10.0, "");

        let eta = tracker.eta().as_secs_f64();
        assert!(eta >= 12.0);

        MockClock::advance(Duration::from_secs(5));
        tracker.inc_work_done_by(80.0, "");

        let eta = tracker.eta().as_secs_f64();
        assert_approx_eq!(f64, eta, 0.0, ulps = 2);
        assert!(tracker.progress().done);
    }
}
