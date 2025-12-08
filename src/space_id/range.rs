use std::{fmt, i64};

use crate::{
    error::Error,
    space_id::{
        SpaceID,
        constants::{F_MAX, F_MIN, XY_MAX},
    },
};

pub struct RangeID {
    z: u8,
    f: [i64; 2],
    x: [u64; 2],
    y: [u64; 2],
}

impl fmt::Display for RangeID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.z,
            format_dimension(self.f),
            format_dimension(self.x),
            format_dimension(self.y),
        )
    }
}

fn format_dimension<T: PartialEq + fmt::Display>(dimension: [T; 2]) -> String {
    if dimension[0] == dimension[1] {
        format!("{}", dimension[0])
    } else {
        format!("{}:{}", dimension[0], dimension[1])
    }
}

impl RangeID {
    pub fn new(z: u8, f: [i64; 2], x: [u64; 2], y: [u64; 2]) -> Result<RangeID, Error> {
        if z > 63u8 {
            return Err(Error::ZoomLevelOutOfRange { zoom_level: z });
        }

        let f_min = F_MIN[z as usize];
        let f_max = F_MAX[z as usize];
        let xy_max = XY_MAX[z as usize];

        // fの範囲が逆転していないか
        if f[0] > f[1] {
            return Err(Error::FRangeReversed {
                start: f[0],
                end: f[1],
            });
        }

        // fの範囲チェック
        if f[0] < f_min || f[0] > f_max {
            return Err(Error::FOutOfRange { f: f[0], z });
        }
        if f[1] < f_min || f[1] > f_max {
            return Err(Error::FOutOfRange { f: f[1], z });
        }

        // xの範囲チェック
        if x[0] > xy_max {
            return Err(Error::XOutOfRange { x: x[0], z });
        }
        if x[1] > xy_max {
            return Err(Error::XOutOfRange { x: x[1], z });
        }

        // yの範囲チェック
        if y[0] > xy_max {
            return Err(Error::YOutOfRange { y: y[0], z });
        }
        if y[1] > xy_max {
            return Err(Error::YOutOfRange { y: y[1], z });
        }

        Ok(RangeID { z, f, x, y })
    }

    pub fn as_z(&self) -> &u8 {
        &self.z
    }

    pub fn as_f(&self) -> &[i64; 2] {
        &self.f
    }

    pub fn as_x(&self) -> &[u64; 2] {
        &self.x
    }

    pub fn as_y(&self) -> &[u64; 2] {
        &self.y
    }

    pub fn children(&self, difference: u8) -> Result<RangeID, Error> {
        todo!()
    }

    pub fn parent(&self, difference: u8) -> Option<RangeID> {
        todo!()
    }
}

impl SpaceID for RangeID {
    fn min_f(&self) -> i64 {
        F_MIN[self.z as usize]
    }

    fn max_f(&self) -> i64 {
        F_MAX[self.z as usize]
    }

    fn max_xy(&self) -> u64 {
        XY_MAX[self.z as usize]
    }

    fn move_up(&mut self, by: i64) -> Result<(), Error> {
        todo!()
    }

    fn move_down(&mut self, by: i64) -> Result<(), Error> {
        todo!()
    }

    fn move_north(&mut self, by: u64) {
        todo!()
    }

    fn move_south(&mut self, by: u64) {
        todo!()
    }

    fn move_east(&mut self, by: u64) {
        todo!()
    }

    fn move_west(&mut self, by: u64) {
        todo!()
    }

    fn set_f(&mut self, value: i64) -> Result<(), Error> {
        todo!()
    }

    fn set_x(&mut self, value: u64) -> Result<(), Error> {
        todo!()
    }

    fn set_y(&mut self, value: u64) -> Result<(), Error> {
        todo!()
    }

    fn into_encode(self) -> crate::encode_id::EncodeID {
        todo!()
    }
}
