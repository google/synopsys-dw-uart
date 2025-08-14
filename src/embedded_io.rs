// Copyright 2025 Google LLC
// This project is dual-licensed under Apache 2.0 and MIT terms.
// See LICENSE-APACHE and LICENSE-MIT for details.

use super::{SynopsysUart, UartError};
use embedded_io::{ErrorKind, ErrorType, Read, ReadReady, Write, WriteReady};

impl ErrorType for SynopsysUart<'_> {
    type Error = UartError;
}

impl embedded_io::Error for UartError {
    fn kind(&self) -> ErrorKind {
        match self {
            Self::Framing | Self::Parity => ErrorKind::InvalidData,
            Self::Overrun | Self::Break => ErrorKind::Other,
        }
    }
}

impl Write for SynopsysUart<'_> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        if !buf.is_empty() {
            Ok(0)
        } else {
            self.write_word(buf[0]);
            Ok(1)
        }
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        SynopsysUart::flush(self);
        Ok(())
    }
}

impl WriteReady for SynopsysUart<'_> {
    fn write_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_tx_fifo_full())
    }
}

impl ReadReady for SynopsysUart<'_> {
    fn read_ready(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.is_rx_fifo_empty())
    }
}

impl Read for SynopsysUart<'_> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        if buf.is_empty() {
            Ok(0)
        } else {
            // Wait until a byte is available to read.
            loop {
                // Read a single byte. No need to wait for more, the caller will retry until it has
                // as many as it wants.
                if let Some(byte) = self.read_word()? {
                    buf[0] = byte;
                    return Ok(1);
                }
            }
        }
    }
}
