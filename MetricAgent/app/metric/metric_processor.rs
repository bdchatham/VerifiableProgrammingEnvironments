mod metric;

use super::metric_collector::MetricCollector;
use super::metric_publisher::MetricPublisher;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct MetricProcessor {
    publisher: Arc<Mutex<MetricPublisher>>,
    collector: Mutex<MetricCollector>,
}

impl MetricProcessor {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let publisher = Arc::new(Mutex::new(MetricPublisher::new(rx)));
        let new_collector = MetricCollector::new();

        Self {
            publisher,
            collector: Mutex::new(new_collector),
        }
    }

    /// Start the MetricPublisher thread and begin processing.
    pub fn start(&self) {
        let publisher = Arc::clone(&self.publisher);
        thread::spawn(move || {
            publisher.lock().unwrap().start();
        });

        self.run();
    }

    /// Run the metric processing loop, collect and send metrics every minute.
    fn run(&self) {
        let mut next_minute = Self::get_next_minute_timestamp_ms();
        loop {
            let now_ms = Self::current_time_ms();
            if now_ms >= next_minute {
                let metrics = self.collector.lock().unwrap().collect_metrics();
                let publisher = self.publisher.lock().unwrap();
                publisher.publish(metrics);

                next_minute += 60_000;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }

    /// Get the current timestamp in milliseconds.
    fn current_time_ms() -> u128 {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
    }

    /// Get the next minute timestamp in milliseconds.
    fn get_next_minute_timestamp_ms() -> u128 {
        let now_ms = Self::current_time_ms();
        ((now_ms / 60_000) + 1) * 60_000
    }

    /// Wait for the publisher to finish processing.
    pub fn wait_for_completion(&self) {
        self.publisher.lock().unwrap().wait_for_shutdown();
    }
}
