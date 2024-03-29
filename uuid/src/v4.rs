use super::{Specs, Uuid, Variant, Version};
use convert::*;
use rand::core::RngCore;
use rand::pcg::Mcg128Xsl64 as Pcg64;
use std::fmt;

struct Internal([u8; 16]);

pub fn new() -> Uuid {
    Uuid(Internal::new().transmute())
}

impl Internal {
    pub(crate) fn new() -> Self {
        // TODO fast, true & secure random 128 bit state without deps on OS
        // maybe we need to:
        // ? use Jitter or ChaChaRng as seeder
        //      (to initialize/update seed with some fast CSPRNG),
        // ? use .from_seed() instead of .new()
        // ? use Xoshiro256** as fast PRNG
        // ? mind about streaming options in some RNGs (ChaChaX as example)
        let state = 0u128;
        let mut rng = Pcg64::new(state);
        let mut bytes = [0u8; 16];
        rng.fill_bytes(bytes.as_mut());
        bytes[6] = bytes[6] & 0x0fu8 | 0x40u8;
        bytes[8] = bytes[8] & 0x3fu8 | 0x80u8;
        Internal(bytes)
    }
}

impl_transmute!([u8; 16], Internal);
impl_transmute!(Internal, [u8; 16]);

impl Specs for Internal {
    fn version(&self) -> Version {
        Version::RANDOM
    }

    fn variant(&self) -> Variant {
        Variant::RFC4122
    }
}

impl fmt::Debug for Internal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Uuid'{:#x?}'", &self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UUIDS: &[u128] = &[
        0x434511c9932b4b58a0f9cedf28d44a35u128,
        0xb8a6077771f64ba09216fd6059395cf9u128,
        0x550e8400e29b41d4a716446655440000u128,
        0x67e5504410b1426f9247bb680e5fe0c8u128,
    ];

    fn generate_uuids() -> Vec<Uuid> {
        let mut arr = vec![];
        for uuid in UUIDS {
            let uuid = (*uuid).to_be().transmute();
            arr.push(Uuid(uuid));
        }
        for _ in 0..10 {
            arr.push(new());
        }
        arr
    }

    #[test]
    fn check_struct_size() {
        use std::mem::size_of_val;

        for uuid in generate_uuids().iter() {
            let size = size_of_val(uuid);
            assert_eq!(size, 16, "{:?} wrong uuid data size", size);
        }
    }

    #[test]
    fn check_bytes_ordering() {
        let count = UUIDS.len();
        for (i, uuid) in generate_uuids().iter().take(count).enumerate() {
            let bytes = uuid.bytes();
            let origin: [u8; 16] = UUIDS[i].to_be().transmute();
            assert_eq!(
                bytes, origin,
                "{:?} vs {:?} wrong bytes ordering",
                bytes, origin
            );
        }
    }

    #[test]
    fn check_variant() {
        for uuid in generate_uuids().iter() {
            // 10x high 2 bits of 8 octet
            let byte = uuid.bytes()[8];
            assert_eq!(byte >> 6, 2u8,
                "{:?} wrong variant in 2 most significant bits of 8 octet {:#x?} 'clk_seq_hi_res' field",
                uuid, byte);
            assert_eq!(uuid.variant(), Variant::RFC4122);
        }
    }

    #[test]
    fn check_version() {
        for uuid in generate_uuids().iter() {
            // 0100 high 4 bits of 6 octet
            let byte = uuid.bytes()[6];
            assert_eq!(byte >> 4, 0x4u8,
                "{:?} wrong version in 4 most significant bits of 6 octet {:#x?} 'time_hi_and_version' field",
                uuid, byte);
            assert_eq!(uuid.version(), Version::RANDOM);
        }
    }
}
