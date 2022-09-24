//! easy to use isp tool
//!
//! [ISP](https://github.com/PrimLabs/ISP-SDK/blob/main/README.md) (IC storage protocol) is a storage protocol built on IC. [ICSP](https://github.com/PrimLabs/ICSP/blob/main/README.md) acts as an index canister to distribute files to the attached Bucket canister. It supports multiple parallel uploading of large-capacity files, automatic expansion of Bucket, and Http forwarding download and get files, support basic rights management. In addition, there is another storage system that focuses on log records. In addition to supporting basic log upload and download, you can obtain verifiable root records of logs stored on the IC to ensure the traceability of logs. The above construction content is related to The SDK has been released for use.
//!
pub mod icsp;
pub mod isp;
pub mod isp_certified_log;
