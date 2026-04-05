const MM_STATIC_SYSVARS: u64 = 0x05 << 32;
const SIGN_EXTENSION_MASK: u64 = 0xffffffff;

/// Murmur3-32 hash (seed=0), matching `solana_sbpf::ebpf::hash_symbol_name`.
///
/// Used at compile time to derive the murmur3 hash for each sysvar name.
const fn murmur3_global(data: &[u8]) -> u32 {
    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;
    let mut h1: u32 = 0; // seed
    let len = data.len();
    let nblocks = len / 4;

    let mut i = 0;
    while i < nblocks {
        let off = i * 4;
        let mut k1 = (data[off] as u32)
            | ((data[off + 1] as u32) << 8)
            | ((data[off + 2] as u32) << 16)
            | ((data[off + 3] as u32) << 24);

        k1 = k1.wrapping_mul(c1);
        k1 = (k1 << 15) | (k1 >> 17);
        k1 = k1.wrapping_mul(c2);

        h1 ^= k1;
        h1 = (h1 << 13) | (h1 >> 19);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);

        i += 1;
    }

    let tail = nblocks * 4;
    let remaining = len & 3;
    let mut k1: u32 = 0;
    if remaining >= 3 {
        k1 ^= (data[tail + 2] as u32) << 16;
    }
    if remaining >= 2 {
        k1 ^= (data[tail + 1] as u32) << 8;
    }
    if remaining >= 1 {
        k1 ^= data[tail] as u32;
        k1 = k1.wrapping_mul(c1);
        k1 = (k1 << 15) | (k1 >> 17);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }

    h1 ^= len as u32;

    // fmix32
    h1 ^= h1 >> 16;
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 ^= h1 >> 13;
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 ^= h1 >> 16;

    h1
}

/// Sysvar Address
///
/// Used at compile time to derive the static address of each sysvar name.
pub const fn sysvar_address(data: &[u8]) -> u64 {
    murmur3_global(data) as u64 & SIGN_EXTENSION_MASK | MM_STATIC_SYSVARS
}