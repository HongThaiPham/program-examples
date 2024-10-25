use steel::*;

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u32)]
pub enum TokenSwapError {
    #[error("Invalid fee, must be between 0 and 10000")]
    InvalidFee = 0,
}

error!(TokenSwapError);
