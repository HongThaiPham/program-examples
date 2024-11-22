use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum CreateAccountInstruction {
    CreateSystemAccount = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateSystemAccount {}

instruction!(CreateAccountInstruction, CreateSystemAccount);