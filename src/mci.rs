use crate::ImplError;

/// An MCI specific error
///
/// This error type contains errors specific to MCI (MultiMedia Card Interface) peripherals. Also it
/// has an `Impl` kind to pass through implementation specific errors occurring while trying to use
/// an MCI peripheral.
#[derive(Debug)]
#[non_exhaustive]
pub enum MciError {
    /// Data Error, can be a CRC problem, timeout or end bit problem
    DataError(CommandOrDataError),
    /// Commands are inhibited from being processed at the moment
    CommandInhibited,
    /// There was a problem sending the command
    CommandError(CommandOrDataError),
    /// ADMA error
    Adma,
    /// Function group trying to be accessed is busy
    GroupBusy,
    /// When trying to do get the CIA register could not find the correct tuple in the response
    CiaCouldNotFindTuple,
    /// Supplied data size is either 0 or more than 512 bytes
    IncorrectDataSize,
    /// Could not select and/or setup the card at the slot
    CouldNotSelectDevice,
    /// No card inserted
    NoCard,
    /// Card is unusable
    UnusableCard,
    /// Read error
    ReadError,
    /// Card is write protected
    WriteProtected,
    /// Write error
    WriteError,
    /// Error reading a pin's value
    PinLevelReadError,
    /// Setup error
    Setup(SetupError),
    /// Implementation specific error (shared across all peripheral specific error kinds)
    Impl(ImplError),
}

/// Enumeration used when setting up the device especially when installing MMC
#[derive(Debug)]
#[non_exhaustive]
pub enum SetupError {
    /// Could not set bus width
    CouldNotSetBusWidth,
    /// Could not set to high speed
    CouldNotSetToHighSpeed,
    /// Could not check if it is a high speed device
    CouldNotCheckIfIsHighSpeed,
}

/// When sending a command (or receiving its response) something can go wrong
#[derive(Debug)]
#[non_exhaustive]
pub enum CommandOrDataError {
    /// Timeout occurred
    Timeout,
    /// CRC check failed
    Crc,
    /// End bit error
    EndBit,
    /// Command index fault
    Index,
}
