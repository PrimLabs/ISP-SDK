# ISP-SDK

[![Documentation](https://docs.rs/ISP-SDK/badge.svg)](https://docs.rs/ISP-SDK/)
[![Crates.io](https://img.shields.io/crates/v/ISP-SDK.svg)](https://crates.io/crates/ISP-SDK)
[![Downloads](https://img.shields.io/crates/d/ISP-SDK.svg)](https://crates.io/crates/ISP-SDK)
[![License](https://img.shields.io/crates/l/ISP-SDK.svg)](https://github.com/PrimLabs/ISP-SDK/blob/main/LICENSE)

**Easy to use isp tool**

ISP (IC storage protocol) is a storage protocol built on IC. [ICSP](https://github.com/PrimLabs/ICSP/blob/main/README.md) acts as an index canister to distribute files to the attached Bucket canister. It supports multiple parallel uploading of large-capacity files, automatic expansion of Bucket, and Http forwarding download and get files, support basic rights management. In addition, there is another storage system that focuses on log records. In addition to supporting basic log upload and download, you can obtain verifiable root records of logs stored on the IC to ensure the traceability of logs. The above construction content is related to The SDK has been released for use.

ISP（IC storage protocol）是一个构建于IC上的存储协议，ICSP作为索引canister将文件分发给下挂的Bucket canister中，支持多片并行上传大容量文件，以及Bucket的自动扩容，支持Http转发进行下载获取文件，支持基本的权限管理，此外，另有专注于日志记录的存储系统，除了支持基本的日志上传和下载以外，可以获取在IC上存储的对日志的可验证的树根记录以保证日志的溯源安全性，以上构建内容相关SDK已发布可供使用。

## Architecture
![](http://scf3f-cyaaa-aaaal-aas3q-cai.raw.ic0.app/fk/iQVVEdXicmv88nJSwmWnc)


