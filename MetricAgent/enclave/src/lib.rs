use automata_sgx_sdk::sgxlib::sgx_types::types::SgxDeviceStatus;

#[no_mangle]
pub extern "C" fn sign_metrics(data: &[u8]) -> Result<[u8; 64], SgxDeviceStatus> {
    Ok([0u8; 64])
}
