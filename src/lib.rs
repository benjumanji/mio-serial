//! # mio-serial - Serial port I/O for mio
//!
//! This crate provides a serial port implementation compatable with mio.
//!
//! **At this time this crate ONLY provides a unix implementation**
//!
//! ## Links
//!   - repo:  https://github.com/berkowski/mio-serial
//!   - docs:  https://docs.rs/mio-serial
#![deny(missing_docs)]

extern crate mio;
extern crate serialport;

#[cfg(windows)]
extern crate mio_named_pipes;
#[cfg(unix)]
extern crate nix;
#[cfg(windows)]
extern crate winapi;

// Enums, Structs, and Traits from the serialport crate
pub use serialport::{
    // Enums
    ClearBuffer,
    DataBits,
    // Structs
    Error,
    ErrorKind,
    FlowControl,
    Parity,
    // Traits
    SerialPort,

    SerialPortInfo,
    SerialPortSettings,

    StopBits,
};

// Re-export port-enumerating utility function.
pub use serialport::available_ports;

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

#[cfg(unix)]
pub use unix::Serial;

#[cfg(windows)]
pub use windows::Serial;

/// A type for results generated by interacting with serial ports.
pub type Result<T> = serialport::Result<T>;
