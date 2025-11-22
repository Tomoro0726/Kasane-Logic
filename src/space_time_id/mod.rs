use serde::Serialize;
pub mod format;
pub mod random;
pub mod z_range;

use crate::{
    error::Error,
    space_time_id::z_range::{F_MAX, F_MIN, XY_MAX},
};

/// 時空間ID（4次元：3次元空間+時間）
///
/// 各次元は範囲として表現され、ズームレベルによって解像度が決まる
/// - z: ズームレベル (0-60)
/// - f: 高度方向の範囲 (符号付き整数)
/// - x: 経度方向の範囲 (符号なし整数)
/// - y: 緯度方向の範囲 (符号なし整数)
/// - i: インデックス
/// - t: 時間範囲
#[derive(Serialize, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpaceTimeId {
    pub z: u8,
    pub f: [i64; 2],
    pub x: [u64; 2],
    pub y: [u64; 2],
    pub i: u64,
    pub t: [u64; 2],
}
impl SpaceTimeId {
    pub fn new(
        z: u8,
        f: [i64; 2],
        x: [u64; 2],
        y: [u64; 2],
        i: u64,
        t: [u64; 2],
    ) -> Result<Self, Error> {
        if z > 60 {
            return Err(Error::ZoomLevelOutOfRange { zoom_level: z });
        }

        let f_max = F_MAX[z as usize];
        let f_min = F_MIN[z as usize];
        let xy_max = XY_MAX[z as usize];

        // 空間の次元を全て値に変換
        let new_f = normalize_dimension(f, f_min, f_max, valid_range_f, z)?;
        let new_x = normalize_dimension(x, 0, xy_max, valid_range_x, z)?;
        let new_y = normalize_dimension(y, 0, xy_max, valid_range_y, z)?;

        // 時間軸をスケーリング（オーバーフローチェック付き）
        let start_t = t[0]
            .checked_mul(i)
            .ok_or_else(|| Error::TimeOverflow { t: t[0], i })?;
        let end_t = t[1]
            .checked_mul(i)
            .ok_or_else(|| Error::TimeOverflow { t: t[1], i })?;

        Ok(SpaceTimeId {
            z,
            f: new_f,
            x: new_x,
            y: new_y,
            i,
            t: [start_t, end_t],
        })
    }
}

/// 次元の値を正規化する関数
fn normalize_dimension<T>(
    dim: [T; 2],
    min: T,
    max: T,
    validate: impl Fn(T, T, T, u8) -> Result<(), Error>,
    z: u8,
) -> Result<[T; 2], Error>
where
    T: PartialOrd + Copy,
{
    // 値が範囲内なのかをチェックする
    validate(dim[0], min, max, z)?;
    validate(dim[1], min, max, z)?;

    // 順序を正しくする
    if dim[1] > dim[0] {
        Ok(dim)
    } else {
        Ok([dim[1], dim[0]])
    }
}

/// Fの範囲が正しいかを確認する
fn valid_range_f(num: i64, min: i64, max: i64, z: u8) -> Result<(), Error> {
    if (min..=max).contains(&num) {
        Ok(())
    } else {
        Err(Error::FOutOfRange { f: num, z })
    }
}

/// Xの範囲が正しいかを確認する
fn valid_range_x(num: u64, min: u64, max: u64, z: u8) -> Result<(), Error> {
    if (min..=max).contains(&num) {
        Ok(())
    } else {
        Err(Error::XOutOfRange { x: num, z })
    }
}

/// Yの範囲が正しいかを確認する
fn valid_range_y(num: u64, min: u64, max: u64, z: u8) -> Result<(), Error> {
    if (min..=max).contains(&num) {
        Ok(())
    } else {
        Err(Error::YOutOfRange { y: num, z })
    }
}
