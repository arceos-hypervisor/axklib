// Copyright 2025 The Axvisor Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Tests for axklib trait definitions.
//!
//! These tests verify that the `Klib` trait can be implemented correctly
//! and that the convenience modules re-export the expected functions.

use core::time::Duration;
use axklib::{AxResult, IrqHandler, Klib, PhysAddr, VirtAddr};
use axklib::klib;

/// Mock implementation of Klib for testing purposes.
struct MockKlib;

impl Klib for MockKlib {
    fn mem_iomap(addr: PhysAddr, size: usize) -> AxResult<VirtAddr> {
        // Simple mock: return a fake virtual address
        Ok(VirtAddr::from(addr.as_usize() + 0xffff_0000_0000_0000))
    }

    fn time_busy_wait(_dur: Duration) {
        // Mock: do nothing
    }

    fn irq_set_enable(_irq: usize, _enabled: bool) {
        // Mock: do nothing
    }

    fn irq_register(_irq: usize, _handler: IrqHandler) -> bool {
        // Mock: always succeed
        true
    }
}

#[test]
fn test_mem_iomap() {
    let paddr = PhysAddr::from(0x1000);
    let result = MockKlib::mem_iomap(paddr, 0x1000);
    assert!(result.is_ok());
    let vaddr = result.unwrap();
    assert_eq!(vaddr.as_usize(), 0x1000 + 0xffff_0000_0000_0000);
}

#[test]
fn test_irq_register() {
    fn dummy_handler() {}
    let result = MockKlib::irq_register(32, dummy_handler);
    assert!(result);
}

#[test]
fn test_irq_set_enable() {
    // Just verify it compiles and runs without panic
    MockKlib::irq_set_enable(32, true);
    MockKlib::irq_set_enable(32, false);
}

#[test]
fn test_time_busy_wait() {
    // Just verify it compiles and runs without panic
    MockKlib::time_busy_wait(Duration::from_micros(100));
}

#[test]
fn test_convenience_module_mem() {
    // Verify the mem module re-exports iomap
    let _ = axklib::mem::iomap;
}

#[test]
fn test_convenience_module_time() {
    // Verify the time module re-exports busy_wait
    let _ = axklib::time::busy_wait;
}

#[test]
fn test_convenience_module_irq() {
    // Verify the irq module re-exports register and set_enable
    let _ = axklib::irq::register;
    let _ = axklib::irq::set_enable;
}
