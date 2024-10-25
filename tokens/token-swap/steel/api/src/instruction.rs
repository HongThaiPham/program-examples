use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, TryFromPrimitive)]
pub enum TokenSwapInstruction {
    CreateAmm = 0,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct CreateAmm {
    pub id: Pubkey,
    pub fee: [u8; 2],
}

instruction!(TokenSwapInstruction, CreateAmm);
