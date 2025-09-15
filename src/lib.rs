// Copyright 2025 Google LLC
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Driver for a Synopsys DesignWare DW_apb UART.

#![no_std]
#![deny(clippy::undocumented_unsafe_blocks)]
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(feature = "embedded-io")]
mod embedded_io;
pub mod registers;

use crate::registers::{Fcr, Lcr, Lsr, Registers, Usr};
use core::{fmt, hint::spin_loop};
use safe_mmio::{UniqueMmioPointer, field, field_shared};
use thiserror::Error;

/// Driver for a Synopsys DesignWare DW_apb UART.
pub struct SynopsysUart<'a> {
    registers: UniqueMmioPointer<'a, Registers>,
}

impl<'a> SynopsysUart<'a> {
    /// Creates a new instance of the UART driver.
    pub const fn new(registers: UniqueMmioPointer<'a, Registers>) -> Self {
        Self { registers }
    }

    /// Configures the UART with the given baud rate, 8 data bits, no parity, and 1 stop bit.
    ///
    /// Also enables the transmit and receive FIFOs.
    ///
    /// This first waits until the UART is not busy, so may block.
    pub fn configure(&mut self, baud_rate: u32, serial_clock: u32) {
        // Wait until the UART is not busy.
        while field_shared!(self.registers, usr)
            .read()
            .contains(Usr::BUSY)
        {
            spin_loop();
        }

        // Enable divisor latch access.
        field!(self.registers, lcr).write(Lcr::DLAB);

        // Set the baud rate.
        let divisor = serial_clock / (16 * baud_rate);
        let fractional = (serial_clock % (16 * baud_rate)) / baud_rate;
        field!(self.registers, dlf).write(fractional);
        field!(self.registers, rbr_thr_dll).write(divisor & 0xff);
        field!(self.registers, dlh_ier).write(divisor >> 8);

        // Configure 8N1 and disable divisor latch access.
        field!(self.registers, lcr).write(Lcr::DLS_8);

        // Enable TX and RX FIFOs.
        field!(self.registers, iir_fcr).write(Fcr::FIFOE.bits());
    }

    /// Returns whether the TX FIFO is currently full.
    ///
    /// If this returns true, `write_word` will block.
    pub fn is_tx_fifo_full(&self) -> bool {
        !field_shared!(self.registers, usr)
            .read()
            .contains(Usr::TFNF)
    }

    /// Writes a single byte to the UART.
    ///
    /// This blocks until there is room in the transmit FIFO to write the byte, but doesn't wait for
    /// the byte to be transmitted.
    pub fn write_word(&mut self, byte: u8) {
        // Wait until the transmit FIFO has space.
        while self.is_tx_fifo_full() {
            spin_loop();
        }

        field!(self.registers, rbr_thr_dll).write(byte.into());
    }

    /// Blocks until all previously written bytes have been transmitted.
    pub fn flush(&self) {
        while !field_shared!(self.registers, usr).read().contains(Usr::TFE) {
            spin_loop();
        }
    }

    /// Returns true if the RX FIFO is currently empty.
    ///
    /// If this returns true then `read_word` will return `None`.
    pub fn is_rx_fifo_empty(&self) -> bool {
        !field_shared!(self.registers, usr)
            .read()
            .contains(Usr::RFNE)
    }

    /// Reads a single byte from the UART.
    ///
    /// If no data is available to be read this will return `Ok(None)`. If there is an error
    /// condition then it will be cleared and an error will be returned.
    pub fn read_word(&mut self) -> Result<Option<u8>, UartError> {
        let lsr = field!(self.registers, lsr).read();

        // The order of these checks is important. In particular, we must check for BI before FE or
        // PE, as a framing error will also set those bits.
        if lsr.contains(Lsr::BI) {
            Err(UartError::Break)
        } else if lsr.contains(Lsr::FE) {
            Err(UartError::Framing)
        } else if lsr.contains(Lsr::PE) {
            Err(UartError::Parity)
        } else if lsr.contains(Lsr::OE) {
            Err(UartError::Overrun)
        } else if !lsr.contains(Lsr::DR) {
            Ok(None)
        } else {
            Ok(Some(field!(self.registers, rbr_thr_dll).read() as u8))
        }
    }
}

// SAFETY: A `&SynopsysUart` only allows operations which read registers, which can safely be done
// from multiple threads simultaneously.
unsafe impl Sync for SynopsysUart<'_> {}

impl fmt::Write for SynopsysUart<'_> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.as_bytes() {
            self.write_word(*byte);
        }
        Ok(())
    }
}

/// A UART read error.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum UartError {
    /// A framing error was detected by the receiver.
    #[error("Framing error")]
    Framing,
    /// A parity error was detected by the receiver.
    #[error("Parity error")]
    Parity,
    /// The receive FIFO overflowed and data was lost.
    #[error("Overrun")]
    Overrun,
    /// A break sequence was detected.
    #[error("Break")]
    Break,
}
