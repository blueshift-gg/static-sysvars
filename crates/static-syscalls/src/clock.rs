#![allow(unused)]

use crate::hash::sysvar_address;

pub const ACCOUNT_OVERHEAD: u64 = 128;

pub struct StaticClock {
    pub slot: u64,
    pub epoch_start_timestamp: i64,
    pub epoch: u64,
    pub leader_schedule_epoch: u64,
    pub unix_timestamp: i64,
}

impl StaticClock {
    /// VM address for SOL_CLOCK_SYSVAR
    const SYSVAR_ADDRESS: *const u8 = sysvar_address(b"SOL_CLOCK_SYSVAR") as *const u8; // 0x5_ff395088

    pub fn get() -> StaticClock {
        // SAFETY: single immutable borrow of the static sysvar data; address is guaranteed to be valid
        unsafe { (Self::SYSVAR_ADDRESS as *const StaticClock).read_volatile() }
    }

    pub fn get_slot() -> u64 {
        // SAFETY: single immutable borrow of the static sysvar data; address is guaranteed to be valid
        unsafe { (Self::SYSVAR_ADDRESS as *const u64).read_volatile() }
    }

    pub fn get_epoch_start_timestamp() -> i64 {
        // SAFETY: single immutable borrow of the static sysvar data; address is guaranteed to be valid
        unsafe { ((Self::SYSVAR_ADDRESS as *const u8).add(8) as *const i64).read_volatile() }
    }

    pub fn get_epoch() -> u64 {
        // SAFETY: single immutable borrow of the static sysvar data; address is guaranteed to be valid
        unsafe { ((Self::SYSVAR_ADDRESS as *const u8).add(16) as *const u64).read_volatile() }
    }

    pub fn get_leader_schedule_epoch() -> u64 {
        // SAFETY: single immutable borrow of the static sysvar data; address is guaranteed to be valid
        unsafe { ((Self::SYSVAR_ADDRESS as *const u8).add(24) as *const u64).read_volatile() }
    }

    pub fn get_unix_timestamp() -> i64 {
        // SAFETY: single immutable borrow of the static sysvar data; address is guaranteed to be valid
        unsafe { ((Self::SYSVAR_ADDRESS as *const u8).add(32) as *const i64).read_volatile() }
    }
}