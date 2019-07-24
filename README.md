# yunpian-marketing-rest-mock

# 云片& 营销云接口mock服务

[![Build Status](https://dev.azure.com/marketing-cloud/yunpian-marketing-rest-mock/_apis/build/status/foxundermoon.yunpian-marketing-rest-mock?branchName=master)](https://dev.azure.com/marketing-cloud/yunpian-marketing-rest-mock/_build/latest?definitionId=1&branchName=master)


目前完成了工程的搭建和 `v2 single_send` 接口
欢迎pull request 实现新的接口


cross compile by docker for linux

```bash

docker run --rm -v ~/.cargo/git:/usr/local/cargo/git \
  -v ~/.cargo/registry:/usr/local/cargo/registry \
  -v ~/.rustup/toolchains:/usr/local/rustup/toolchains \
  -v $(pwd):/app \
  -w /app/ \
  -t rustlang/rust:nightly \
  cargo build --release --target=x86_64-unknown-linux-gnu --bin yunpian-marketing-rest-mock --features 'standalone'


```