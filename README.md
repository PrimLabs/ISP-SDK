# ISP-SDK

[![Documentation](https://docs.rs/ISP-SDK/badge.svg)](https://docs.rs/ISP-SDK/)
[![Crates.io](https://img.shields.io/crates/v/ISP-SDK.svg)](https://crates.io/crates/ISP-SDK)
[![Downloads](https://img.shields.io/crates/d/ISP-SDK.svg)](https://crates.io/crates/ISP-SDK)
[![License](https://img.shields.io/crates/l/ISP-SDK.svg)](https://github.com/PrimLabs/ISP-SDK/blob/main/LICENSE)

**Easy to use isp tool**

ISP (IC storage protocol) is a scalable storage protocol built on IC. [ICSP](https://github.com/PrimLabs/ICSP/blob/main/README.md) acts as an index canister to distribute files to the attached Bucket canister. It supports multiple parallel uploading of large-capacity files, automatic expansion of Bucket, and Http forwarding download and get files, support basic rights management. In addition, there is another storage system that focuses on log records. In addition to supporting basic log upload and download, you can obtain verifiable root records of logs stored on the IC to ensure the traceability of logs. The above construction content is related to The SDK has been released for use.

ISP（IC storage protocol）是一个构建于IC上的可扩展的存储协议，ICSP作为索引canister将文件分发给下挂的Bucket canister中，支持多片并行上传大容量文件，以及Bucket的自动扩容，支持Http转发进行下载获取文件，支持基本的权限管理，此外，另有专注于日志记录的存储系统，除了支持基本的日志上传和下载以外，可以获取在IC上存储的对日志的可验证的树根记录以保证日志的溯源安全性，以上构建内容相关SDK已发布可供使用。

# Usage
### English
1. Make sure you have enough ICP in your ISP's subAccount to create an ICSP canister
2. Make sure you have enough [XTC](https://github.com/Psychedelic/dank/tree/develop/xtc) in your pem account to recharge Cycles to ICSP canister, which you can find at [sonic](https://app.sonic.ooo/swap) to exchange
3. Call ``isp_sdk::isp::create_icsp`` to create your ICSP
4. call ``isp_sdk::icsp::store_file`` to store the file
5. Call other interfaces to complete related operations

### Chinese

1. 确保你在ISP的子账户(subAccount)有足够的ICP用来创建ICSP canister 
2. 确保你的pem的账户有足够的[XTC](https://github.com/Psychedelic/dank/tree/develop/xtc)来给ICSP canister充值Cycles，你可以在[sonic](https://app.sonic.ooo/swap)进行兑换
3. 调用 ``isp_sdk::isp::create_icsp``来创建你的ICSP
4. 调用 ``isp_sdk::icsp::store_file``来储存文件
5. 调用 其他接口来完成相关操作


## Architecture
![](https://kh4t2-waaaa-aaaal-qbhbq-cai.raw.ic0.app/file/wjD6l7ZodvfnUBZVyxHpu)


