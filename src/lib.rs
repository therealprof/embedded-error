#![no_std]

pub mod mci;

/// This crate contains a variety of universal error types which can be used to universally model
/// conditions which can typically arise for certain peripherals.
///
/// When used by HAL implementations, they allow drivers and applications alike to generically
/// handle those situations without the error handling being specific to the hardware it is
/// supposed to run on (which is usually not possible to implement in drivers).
///
/// All of the enums in this crate are marked as `#[non_exhaustive]` to allow for additions of new
/// error kinds without requiring a breaking change and version bump.

/// A GPIO (General input/output) specific error.
///
/// This error type contains errors specific to GPIO peripherals. Also it has an `Impl` kind to
/// pass through implementation specific errors occuring while trying to use a GPIO peripheral.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum GpioError {
    /// The peripheral is in the wrong operational mode for the intended operation
    WrongMode,
    /// Implementation specific error (shared across all peripheral specific error kinds)
    Impl(ImplError),
}


/// A USB specific error.
///
/// This error type contains errors specific to USB peripherals. Also it has an `Impl` kind to pass
/// through implementation specific errors occuring while trying to use a USB peripheral.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum UsbError {
    /// An operation would block because the device is currently busy or there is no data available.
    WouldBlock,
    /// Parsing failed due to invalid input.
    ParseError,
    /// A buffer too short for the data to read was passed, or provided data cannot fit within
    /// length constraints.
    BufferOverflow,
    /// Classes attempted to allocate more endpoints than the peripheral supports.
    EndpointOverflow,
    /// Classes attempted to allocate more packet buffer memory than the peripheral supports. This
    /// can be caused by either a single class trying to allocate a packet buffer larger than the
    /// peripheral supports per endpoint, or multiple allocated endpoints together using more memory
    /// than the peripheral has available for the buffers.
    EndpointMemoryOverflow,
    /// The endpoint address is invalid or already used.
    InvalidEndpoint,
    /// Operation is not supported by device or configuration.
    Unsupported,
    /// Operation is not valid in the current state of the object.
    InvalidState,
    /// Implementation specific error (shared across all peripheral specific error kinds)
    Impl(ImplError),
}


/// A SPI specific error.
///
/// This error type contains errors specific to SPI peripherals. Also it has an `Impl` kind to pass
/// through implementation specific errors occuring while trying to use a SPI peripheral.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SpiError {
    /// The peripheral receive buffer was overrun
    Overrun,
    /// Multiple devices on the SPI bus are trying across each other, e.g. in a multi-master setup
    ModeFault,
    /// CRC does not match the received data
    CRCError,
    /// Received data does not conform to the peripheral configuration
    FrameFormat,
    /// Implementation specific error (shared across all peripheral specific error kinds)
    Impl(ImplError),
}

/// A Serial specific error.
///
/// This error type contains errors specific to Serial peripherals. Also it has an `Impl` kind to pass
/// through implementation specific errors occurring while trying to use a Serial peripheral.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum SerialError {
    /// The peripheral receive buffer was overrun.
    Overrun,
    /// Received data does not conform to the peripheral configuration.
    /// Can be caused by a misconfigured device on either end of the serial line.
    FrameFormat,
    /// Parity check failed.
    Parity,
    /// Serial line is too noisy to read valid data.
    Noise,
    /// Implementation specific error (shared across all peripheral specific error kinds).
    Impl(ImplError),
}

/// An I2C specific error.
///
/// This error type contains errors specific to I2C peripherals. Also it has an `Impl` kind to pass
/// through implementation specific errors occurring while trying to use an I2C peripheral.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum I2cError {
    /// An unspecific bus error occured
    Bus,
    /// The arbitration was lost, e.g. electrical problems with the clock signal
    ArbitrationLoss,
    /// A bus operation received a NACK, e.g. due to the addressed device not being available on
    /// the bus or device not being ready to process any requests at the moment
    NACK,
    /// The peripheral receive buffer was overrun
    Overrun,
    /// The peripheral send buffer ran out of data
    Underrun,
    /// SMBus Error checking byte mismatch
    PacketErrorChecking,
    /// SMBus Timeout error
    Timeout,
    /// SMBus Alert received
    Alert,
    /// Implementation specific error (shared across all peripheral specific error kinds)
    Impl(ImplError),
}

/// A universal implementation specific error.
///
/// These error kinds can be used to signal implementation specific errors unrelated to the
/// specific peripheral. This will be used for all sorts of connectivity problems, e.g. if an
/// adapter to the peripheral is used or the target peripheral is connected to indirectly (like bus
/// expanders) or an operating system is controlling the access and denying access.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ImplError {
    /// Unspecified internal driver error
    Internal,
    /// Connection lost, e.g. device adapter was unplugged
    Disconnected,
    /// Ran out of memory while trying to allocate required buffers
    OutOfMemory,
    /// Operation timed out, please retry
    TimedOut,
    /// Peripheral is sleeping or in standby
    Asleep,
    /// Peripheral is powered down
    PowerDown,
    /// The peripheral cannot work with the specified settings
    InvalidConfiguration,
    /// Could not open connection to peripheral
    CouldNotOpen,
    /// No sufficient permissions to connect to peripheral
    PermissionDenied,
}
