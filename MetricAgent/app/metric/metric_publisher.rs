mod metric;

use std::collections::HashMap;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

use crate::MyEnclave;

pub struct MetricPublisher {
    shutdown: bool,
    receiver: Receiver<HashMap<&'static str, u64>>,
}

impl MetricPublisher {
    pub fn new(receiver: Receiver<HashMap<&'static str, u64>>) -> Self {
        Self {
            shutdown: false,
            receiver,
        }
    }

    pub fn start(&mut self) {
        println!("MetricPublisher started.");

        while !self.shutdown {
            match self.receiver.try_recv() {
                Ok(metrics) => {
                    self.publish(metrics); // Publish signed metrics
                }
                Err(TryRecvError::Empty) => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(TryRecvError::Disconnected) => {
                    println!("Channel disconnected. Shutting down.");
                    self.shutdown = true;
                }
            }
        }
    }

    /// Sign the metrics using the SGX enclave before publishing.
    pub fn publish(&self, metrics: HashMap<&'static str, u64>) {
        let metrics_json = serde_json::to_string(&metrics).unwrap();

        // Use the enclave to sign the metrics
        let signature = MyEnclave::sign_metrics(metrics_json.as_bytes()).unwrap();

        println!(
            "Publishing signed metrics: {:?}, Signature: {:?}",
            metrics, signature
        );
    }

    pub fn is_shutdown(&self) -> bool {
        self.shutdown
    }

    pub fn wait_for_shutdown(&self) {
        while !self.shutdown {
            thread::sleep(Duration::from_millis(10));
        }
        println!("Shutdown complete.");
    }
}
