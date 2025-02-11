/// number of ranks
pub const NUMBER_OF_RANKS: usize = 13;

/// number of ranks
pub const NUMBER_OF_CARDS: usize = 4 * NUMBER_OF_RANKS;

/// determines perfect hash function. adjust this parameter to modify the offset table
pub const OFFSET_SHIFT: usize = 11;

/// rank keys that guarantee a unique sum for every rank combination of 0-7 cards - taken from OMP Eval
pub const RANK_BASES: [u64; NUMBER_OF_RANKS] = [0x2000, 0x8001, 0x11000, 0x3a000, 0x91000, 0x176005, 0x366000, 0x41a013, 0x47802e, 0x479068, 0x48c0e4, 0x48f211, 0x494493];

/// max rank key value (4 aces + 3 kings)
pub const MAX_RANK_KEY: u64 =
    4 * RANK_BASES[NUMBER_OF_RANKS - 1] + 3 * RANK_BASES[NUMBER_OF_RANKS - 2];

/// suit keys (club, diamond, heart, spade)
pub const SUIT_BASES: [u64; 4] = [0x1000, 0x0100, 0x0010, 0x0001];

/// shift value for suit
pub const SUIT_SHIFT: usize = 48;

/// bit mask for checking flush
pub const FLUSH_MASK: u64 = 0x8888 << SUIT_SHIFT;

/// (card key, bit mask) of cards
#[rustfmt::skip]
pub const CARDS: [(u64, u64); NUMBER_OF_CARDS] = [
    /* 2c */ (RANK_BASES[0] + (SUIT_BASES[0] << SUIT_SHIFT), 0x1),
    /* 2d */ (RANK_BASES[0] + (SUIT_BASES[1] << SUIT_SHIFT), 0x10000),
    /* 2h */ (RANK_BASES[0] + (SUIT_BASES[2] << SUIT_SHIFT), 0x100000000),
    /* 2s */ (RANK_BASES[0] + (SUIT_BASES[3] << SUIT_SHIFT), 0x1000000000000),
    /* 3c */ (RANK_BASES[1] + (SUIT_BASES[0] << SUIT_SHIFT), 0x2),
    /* 3d */ (RANK_BASES[1] + (SUIT_BASES[1] << SUIT_SHIFT), 0x20000),
    /* 3h */ (RANK_BASES[1] + (SUIT_BASES[2] << SUIT_SHIFT), 0x200000000),
    /* 3s */ (RANK_BASES[1] + (SUIT_BASES[3] << SUIT_SHIFT), 0x2000000000000),
    /* 4c */ (RANK_BASES[2] + (SUIT_BASES[0] << SUIT_SHIFT), 0x4),
    /* 4d */ (RANK_BASES[2] + (SUIT_BASES[1] << SUIT_SHIFT), 0x40000),
    /* 4h */ (RANK_BASES[2] + (SUIT_BASES[2] << SUIT_SHIFT), 0x400000000),
    /* 4s */ (RANK_BASES[2] + (SUIT_BASES[3] << SUIT_SHIFT), 0x4000000000000),
    /* 5c */ (RANK_BASES[3] + (SUIT_BASES[0] << SUIT_SHIFT), 0x8),
    /* 5d */ (RANK_BASES[3] + (SUIT_BASES[1] << SUIT_SHIFT), 0x80000),
    /* 5h */ (RANK_BASES[3] + (SUIT_BASES[2] << SUIT_SHIFT), 0x800000000),
    /* 5s */ (RANK_BASES[3] + (SUIT_BASES[3] << SUIT_SHIFT), 0x8000000000000),
    /* 6c */ (RANK_BASES[4] + (SUIT_BASES[0] << SUIT_SHIFT), 0x10),
    /* 6d */ (RANK_BASES[4] + (SUIT_BASES[1] << SUIT_SHIFT), 0x100000),
    /* 6h */ (RANK_BASES[4] + (SUIT_BASES[2] << SUIT_SHIFT), 0x1000000000),
    /* 6s */ (RANK_BASES[4] + (SUIT_BASES[3] << SUIT_SHIFT), 0x10000000000000),
    /* 7c */ (RANK_BASES[5] + (SUIT_BASES[0] << SUIT_SHIFT), 0x20),
    /* 7d */ (RANK_BASES[5] + (SUIT_BASES[1] << SUIT_SHIFT), 0x200000),
    /* 7h */ (RANK_BASES[5] + (SUIT_BASES[2] << SUIT_SHIFT), 0x2000000000),
    /* 7s */ (RANK_BASES[5] + (SUIT_BASES[3] << SUIT_SHIFT), 0x20000000000000),
    /* 8c */ (RANK_BASES[6] + (SUIT_BASES[0] << SUIT_SHIFT), 0x40),
    /* 8d */ (RANK_BASES[6] + (SUIT_BASES[1] << SUIT_SHIFT), 0x400000),
    /* 8h */ (RANK_BASES[6] + (SUIT_BASES[2] << SUIT_SHIFT), 0x4000000000),
    /* 8s */ (RANK_BASES[6] + (SUIT_BASES[3] << SUIT_SHIFT), 0x40000000000000),
    /* 9c */ (RANK_BASES[7] + (SUIT_BASES[0] << SUIT_SHIFT), 0x80),
    /* 9d */ (RANK_BASES[7] + (SUIT_BASES[1] << SUIT_SHIFT), 0x800000),
    /* 9h */ (RANK_BASES[7] + (SUIT_BASES[2] << SUIT_SHIFT), 0x8000000000),
    /* 9s */ (RANK_BASES[7] + (SUIT_BASES[3] << SUIT_SHIFT), 0x80000000000000),
    /* Tc */ (RANK_BASES[8] + (SUIT_BASES[0] << SUIT_SHIFT), 0x100),
    /* Td */ (RANK_BASES[8] + (SUIT_BASES[1] << SUIT_SHIFT), 0x1000000),
    /* Th */ (RANK_BASES[8] + (SUIT_BASES[2] << SUIT_SHIFT), 0x10000000000),
    /* Ts */ (RANK_BASES[8] + (SUIT_BASES[3] << SUIT_SHIFT), 0x100000000000000),
    /* Jc */ (RANK_BASES[9] + (SUIT_BASES[0] << SUIT_SHIFT), 0x200),
    /* Jd */ (RANK_BASES[9] + (SUIT_BASES[1] << SUIT_SHIFT), 0x2000000),
    /* Jh */ (RANK_BASES[9] + (SUIT_BASES[2] << SUIT_SHIFT), 0x20000000000),
    /* Js */ (RANK_BASES[9] + (SUIT_BASES[3] << SUIT_SHIFT), 0x200000000000000),
    /* Qc */ (RANK_BASES[10] + (SUIT_BASES[0] << SUIT_SHIFT), 0x400),
    /* Qd */ (RANK_BASES[10] + (SUIT_BASES[1] << SUIT_SHIFT), 0x4000000),
    /* Qh */ (RANK_BASES[10] + (SUIT_BASES[2] << SUIT_SHIFT), 0x40000000000),
    /* Qs */ (RANK_BASES[10] + (SUIT_BASES[3] << SUIT_SHIFT), 0x400000000000000),
    /* Kc */ (RANK_BASES[11] + (SUIT_BASES[0] << SUIT_SHIFT), 0x800),
    /* Kd */ (RANK_BASES[11] + (SUIT_BASES[1] << SUIT_SHIFT), 0x8000000),
    /* Kh */ (RANK_BASES[11] + (SUIT_BASES[2] << SUIT_SHIFT), 0x80000000000),
    /* Ks */ (RANK_BASES[11] + (SUIT_BASES[3] << SUIT_SHIFT), 0x800000000000000),
    /* Ac */ (RANK_BASES[12] + (SUIT_BASES[0] << SUIT_SHIFT), 0x1000),
    /* Ad */ (RANK_BASES[12] + (SUIT_BASES[1] << SUIT_SHIFT), 0x10000000),
    /* Ah */ (RANK_BASES[12] + (SUIT_BASES[2] << SUIT_SHIFT), 0x100000000000),
    /* As */ (RANK_BASES[12] + (SUIT_BASES[3] << SUIT_SHIFT), 0x1000000000000000),
];
