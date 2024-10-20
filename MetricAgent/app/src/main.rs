use crate::metric;

use automata_sgx_sdk::enclave;
use automata_sgx_sdk::sgxlib::sgx_types;
use metric_processor::MetricProcessor;

enclave! {
    name: SgxEnclave,
    ecall: {
        fn sign_metrics(data: &[u8]) -> Result<[u8; 64], sgx_types::types::SgxDeviceStatus>;
    }
}

fn main() {
    // Create and start the metric processor
    let metric_processor = metric::MetricProcessor::new();
    metric_processor.start();

    // Wait for the processor to complete
    metric_processor.wait_for_completion();
}
