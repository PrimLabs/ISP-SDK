//! **Easy to use isp tool**
//!
//! [ISP](https://github.com/PrimLabs/ISP-SDK/blob/main/README.md) (IC storage protocol) is a storage protocol built on IC. [ICSP](https://github.com/PrimLabs/ICSP/blob/main/README.md) acts as an index canister to distribute files to the attached Bucket canister. It supports multiple parallel uploading of large-capacity files, automatic expansion of Bucket, and Http forwarding download and get files, support basic rights management. In addition, there is another storage system that focuses on log records. In addition to supporting basic log upload and download, you can obtain verifiable root records of logs stored on the IC to ensure the traceability of logs. The above construction content is related to The SDK has been released for use.
//!
//! # Usage
//! 1. Make sure you have enough ICP in your ISP's subAccount to create an ICSP canister
//! 2. Make sure you have enough [XTC](https://github.com/Psychedelic/dank/tree/develop/xtc) in your pem account to recharge Cycles to ICSP canister, which you can find at [sonic](https://app.sonic.ooo/swap) to exchange
//! 3. Call ``isp_sdk::isp::create_icsp`` to create your ICSP
//! 4. call ``isp_sdk::icsp::store_file`` to store the file
//! 5. Call other interfaces to complete related operations
//!
//! ## Architecture
//! ![](https://scf3f-cyaaa-aaaal-aas3q-cai.raw.ic0.app/fk/VOhoOCto-8SRSfYZ1jKhE)
//!
pub mod icsp;
pub mod isp;
pub mod isp_certified_log;
