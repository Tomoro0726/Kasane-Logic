use crate::{bit_vec::BitVec, space_time_id_set::single::convert_bitvec_xy::convert_bitmask_xy};

///FをBitVecに変換する
pub fn convert_bitmask_f(z: u8, f: i64) -> BitVec {
    if f >= 0 {
        convert_bitmask_xy(z, f as u64)
    } else {
        let u = if f == i64::MIN {
            (i64::MAX as u64) + 1
        } else {
            f.abs() as u64
        };

        let mut converted = convert_bitmask_xy(z, u);
        let masked: u8 = 0b11000000;
        converted.0[0] |= masked;

        converted
    }
}
