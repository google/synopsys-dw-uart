// Copyright 2025 Google LLC
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

//! Types for the UART's MMIO registers.

use bitflags::bitflags;
use safe_mmio::fields::{ReadOnly, ReadPure, ReadPureWrite, ReadWrite, WriteOnly};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout};

/// A line control register value.
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(transparent)]
pub struct Lcr(u32);

bitflags! {
    impl Lcr: u32 {
        /// Divisor latch access bit.
        const DLAB = 1 << 7;
        /// Break control bit.
        const BC = 1 << 6;
        /// Even parity select.
        const EPS = 1 << 4;
        /// Parity enable.
        const PEN = 1 << 3;
        /// Number of stop bits.
        ///
        /// 0 means 1 stop bit, 1 means 1.5 stop bits.
        const STOP = 1 << 2;
    }
}

impl Lcr {
    /// 5 data bits.
    pub const DLS_5: Self = Self(0b00);
    /// 6 data bits.
    pub const DLS_6: Self = Self(0b01);
    /// 7 data bits.
    pub const DLS_7: Self = Self(0b10);
    /// 8 data bits.
    pub const DLS_8: Self = Self(0b11);
}

/// A modem control register value.
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(transparent)]
pub struct Mcr(u32);

bitflags! {
    impl Mcr: u32 {
        /// SIR mode enable.
        const SIRE = 1 << 6;
        /// Auto flow control enable.
        const AFCE = 1 << 5;
        //// Loopback mode.
        const LB = 1 << 4;
        /// User-designated output 2 (inverted).
        const OUT2 = 1 << 3;
        /// User-designated output 1 (inverted).
        const OUT1 = 1 << 2;
        /// Request to send.
        const RTS = 1 << 1;
        /// Data terminal ready.
        const DTR = 1 << 0;
    }
}

/// A line status register value.
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(transparent)]
pub struct Lsr(u32);

bitflags! {
    impl Lsr: u32 {
        /// Receiver FIFO error.
        const RFE = 1 << 7;
        /// Transmitter empty.
        const TEMT = 1 << 6;
        /// Transmit holding register empty.
        const THRE = 1 << 5;
        /// Break interrupt.
        const BI = 1 << 4;
        /// Framing error.
        const FE = 1 << 3;
        /// Parity error.
        const PE = 1 << 2;
        /// Overrun error.
        const OE = 1 << 1;
        /// Data ready.
        const DR = 1 << 0;
    }
}

/// A modem status register value.
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(transparent)]
pub struct Msr(u32);

bitflags! {
    impl Msr: u32 {
        /// Data carrier detect.
        const DCD = 1 << 7;
        /// Ring indicator.
        const RI = 1 << 6;
        /// Data set ready.
        const DSR = 1 << 5;
        /// Clear to send.
        const CTS = 1 << 4;
        /// Delta data carrier detect.
        const DDCD = 1 << 3;
        /// Trailing edge of ring indicator.
        const TERI = 1 << 2;
        /// Delta data set ready.
        const DDSR = 1 << 1;
        /// Delta clear to send.
        const DCTS = 1 << 0;
    }
}

/// A UART status register value.
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(transparent)]
pub struct Usr(u32);

bitflags! {
    impl Usr: u32 {
        /// Receive FIFO full.
        const RFF = 1 << 4;
        /// Receive FIFO not empty.
        const RFNE = 1 << 3;
        /// Transmit FIFO empty.
        const TFE = 1 << 2;
        /// Transmit FIFO not full.
        const TFNF = 1 << 1;
        /// UART busy.
        const BUSY = 1 << 0;
    }
}

/// A FIFO control register value.
#[derive(Copy, Clone, Debug, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(transparent)]
pub struct Fcr(u32);

bitflags! {
    impl Fcr: u32 {
        /// DMA mode.
        const DMAM = 1 << 3;
        /// TX FIFO reset.
        const XFIFOR = 1 << 2;
        /// RX FIFO reset.
        const RFIFOR = 1 << 1;
        /// FIFO enable.
        const FIFOE = 1 << 0;
    }
}

impl Fcr {
    pub const RT_1_CHAR: Self = Self(0b00 << 6);
    pub const RT_QUARTER_FULL: Self = Self(0b01 << 6);
    pub const RT_HALF_FULL: Self = Self(0b10 << 6);
    pub const RT_2_LESS: Self = Self(0b11 << 6);

    pub const TET_EMPTY: Self = Self(0b00 << 4);
    pub const TET_2_CHARS: Self = Self(0b01 << 4);
    pub const TET_QUARTER_FULL: Self = Self(0b10 << 4);
    pub const TET_HALF_FULL: Self = Self(0b11 << 4);
}

/// Registers of a Synopsys DesignWare DW_apb UART.
#[derive(Clone, Eq, FromBytes, Immutable, IntoBytes, KnownLayout, PartialEq)]
#[repr(C, align(4))]
pub struct Registers {
    pub rbr_thr_dll: ReadWrite<u32>,
    pub dlh_ier: ReadPureWrite<u32>,
    pub iir_fcr: ReadPureWrite<u32>,
    pub lcr: ReadPureWrite<Lcr>,
    pub mcr: ReadPureWrite<Mcr>,
    pub lsr: ReadOnly<Lsr>,
    pub msr: ReadOnly<Msr>,
    pub scr: ReadPureWrite<u32>,
    pub lpdll: ReadPureWrite<u32>,
    pub lpdlh: ReadPureWrite<u32>,
    reserved: [u32; 2],
    pub srbr_sthr: [ReadWrite<u32>; 16],
    pub far: ReadWrite<u32>,
    pub tfr: ReadOnly<u32>,
    pub rfw: WriteOnly<u32>,
    pub usr: ReadPure<Usr>,
    pub tfl: ReadPure<u32>,
    pub rfl: ReadPure<u32>,
    pub srr: WriteOnly<u32>,
    pub srts: ReadPureWrite<u32>,
    pub sbcr: ReadPureWrite<u32>,
    pub sdmam: ReadPureWrite<u32>,
    pub sfe: ReadPureWrite<u32>,
    pub srt: ReadPureWrite<u32>,
    pub stet: ReadPureWrite<u32>,
    pub htx: ReadPureWrite<u32>,
    pub dmasa: WriteOnly<u32>,
    unused_1: [u32; 5],
    pub dlf: ReadPureWrite<u32>,
    unused_2: [u32; 12],
    pub cpr: ReadPure<u32>,
    pub ucv: ReadPure<u32>,
    pub ctr: ReadPure<u32>,
}
