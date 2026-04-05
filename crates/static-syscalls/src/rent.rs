#![allow(unused)]

use crate::hash::sysvar_address;

pub const ACCOUNT_OVERHEAD: u64 = 128;

pub struct StaticRent {
    lamports_per_byte: u64,
}

impl StaticRent {
    /// VM address for SOL_RENT_SYSVAR
    const SYSVAR_ADDRESS: *const u8 = sysvar_address(b"SOL_RENT_SYSVAR") as *const u8; // 0x5_494df715

    #[inline(always)]
    pub fn get() -> StaticRent {
        // SAFETY: single immutable borrow of the static sysvar data; address is guaranteed to be valid
        unsafe { (Self::SYSVAR_ADDRESS as *const StaticRent).read_volatile() }
    }

    #[inline(always)]
    pub fn lamports_per_byte() -> u64 {
        // SAFETY: single immutable borrow of the static sysvar data; address is guaranteed to be valid
        unsafe { (Self::SYSVAR_ADDRESS as *const u64).read_volatile() }
    }

    #[inline(always)]
    pub fn minimum_balance(data_len: usize) -> u64 {
        Self::lamports_per_byte() * (ACCOUNT_OVERHEAD + data_len as u64)
    }

}