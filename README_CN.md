<h1 align="center">axklib</h1>

<p align="center">小型内核辅助抽象库</p>

<div align="center">

[![Crates.io](https://img.shields.io/crates/v/axklib.svg)](https://crates.io/crates/axklib)
[![Docs.rs](https://docs.rs/axklib/badge.svg)](https://docs.rs/axklib)
[![Rust](https://img.shields.io/badge/edition-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](https://github.com/arceos-hypervisor/axklib/blob/main/LICENSE)

</div>

[English](README.md) | 中文

# Introduction

`axklib` 是一个用于微内核及相关底层组件的小型 `#![no_std]` 内核辅助抽象库。它定义了由平台层实现的 trait，用于 MMIO 映射、忙等待计时和 IRQ 注册，并额外提供了一组便于调用的轻量级辅助模块。

该库导出以下核心项：

- **`Klib`** - 由平台或板级层实现的外部 trait
- **`IrqHandler`** - IRQ 回调注册使用的函数指针类型
- **`AxResult`**、**`PhysAddr`**、**`VirtAddr`** - 重新导出的结果类型与地址类型

此外还提供三个便捷模块：

- **`mem`** - 重新导出 `iomap`
- **`time`** - 重新导出 `busy_wait`
- **`irq`** - 重新导出 `register` 和 `set_enable`

## Quick Start

### Requirements

- Rust nightly 工具链
- Rust 组件：rust-src、clippy、rustfmt

```bash
# 安装 rustup（如果尚未安装）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 安装 nightly 工具链与所需组件
rustup install nightly
rustup component add rust-src clippy rustfmt --toolchain nightly
```

### Run Check and Test

```bash
# 1. 进入仓库目录
cd axklib

# 2. 代码检查
./scripts/check.sh

# 3. 运行测试
./scripts/test.sh
```

## Integration

### Installation

将以下依赖加入 `Cargo.toml`：

```toml
[dependencies]
axklib = "0.3.0"
```

### Example

```rust
use axklib::{AxResult, IrqHandler, Klib, PhysAddr, VirtAddr};
use core::time::Duration;

struct MockKlib;

impl Klib for MockKlib {
    fn mem_iomap(addr: PhysAddr, _size: usize) -> AxResult<VirtAddr> {
        Ok(VirtAddr::from(addr.as_usize() + 0xffff_0000_0000_0000))
    }

    fn time_busy_wait(_dur: Duration) {}

    fn irq_set_enable(_irq: usize, _enabled: bool) {}

    fn irq_register(_irq: usize, _handler: IrqHandler) -> bool {
        true
    }
}

fn my_irq_handler() {}

fn main() {
    let paddr = PhysAddr::from(0x1000);
    let vaddr = MockKlib::mem_iomap(paddr, 0x1000).unwrap();
    assert_eq!(vaddr.as_usize(), 0x1000 + 0xffff_0000_0000_0000);

    MockKlib::time_busy_wait(Duration::from_micros(100));
    assert!(MockKlib::irq_register(32, my_irq_handler));
    MockKlib::irq_set_enable(32, true);
}
```

### Documentation

生成并查看 API 文档：

```bash
cargo doc --no-deps --open
```

在线文档： [docs.rs/axklib](https://docs.rs/axklib)

# Contributing

1. Fork 仓库并创建分支
2. 本地运行检查：`./scripts/check.sh`
3. 本地运行测试：`./scripts/test.sh`
4. 提交 PR 并通过 CI 检查

# License

本项目基于 Apache License 2.0 许可证发布。详见 [LICENSE](LICENSE)。
