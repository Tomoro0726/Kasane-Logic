pub mod fmt;

use std::i64;

use crate::{
    error::Error,
    space_id::{
        SpaceID,
        constants::{F_MAX, F_MIN, XY_MAX},
    },
};

pub struct SingleID {
    z: u8,
    f: i64,
    x: u64,
    y: u64,
}

///SingleSpaceIDの独自メゾットを定義している
impl SingleID {
    pub fn new(z: u8, f: i64, x: u64, y: u64) -> Result<SingleID, Error> {
        //ズームレベルが範囲内であることを検証する
        if z > 63 as u8 {
            return Err(Error::ZoomLevelOutOfRange { zoom_level: z });
        };

        //各次元の範囲を定数配列から読み込む
        let f_max = F_MAX[z as usize];
        let f_min = F_MIN[z as usize];
        let xy_max = XY_MAX[z as usize];

        //各次元の範囲が正しいことをチェックする
        if !(f_min <= f && f <= f_max) {
            return Err(Error::FOutOfRange { f, z });
        };

        if !(x <= xy_max) {
            return Err(Error::XOutOfRange { x, z });
        };

        if !(y <= xy_max) {
            return Err(Error::YOutOfRange { y, z });
        };

        return Ok(SingleID { z, f, x, y });
    }

    pub fn as_z(&self) -> &u8 {
        &self.z
    }

    pub fn as_f(&self) -> &i64 {
        &self.f
    }

    pub fn as_x(&self) -> &u64 {
        &self.x
    }

    pub fn as_y(&self) -> &u64 {
        &self.y
    }

    pub fn parent(&self, difference: u8) -> Option<SingleID> {
        todo!()
    }

    pub fn children(&self, difference: u8) -> Result<[SingleID; 8], Error> {
        todo!()
    }
}

impl SpaceID for SingleID {
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
        self.f = match self.f.checked_add(by) {
            Some(f) => {
                if self.min_f() <= f && f <= self.max_f() {
                    f
                } else {
                    return Err(Error::FOutOfRange { f: f, z: self.z });
                }
            }
            None => {
                return Err(Error::FOutOfRange {
                    f: i64::MAX,
                    z: self.z,
                });
            }
        };
        Ok(())
    }

    fn move_down(&mut self, by: i64) -> Result<(), Error> {
        self.f = match self.f.checked_sub(by) {
            Some(f) => {
                if self.min_f() <= f && f <= self.max_f() {
                    f
                } else {
                    return Err(Error::FOutOfRange { f: f, z: self.z });
                }
            }
            None => {
                return Err(Error::FOutOfRange {
                    f: i64::MIN,
                    z: self.z,
                });
            }
        };
        Ok(())
    }

    fn move_north(&mut self, by: u64) {
        self.y = (self.y.wrapping_add(by)) % self.max_xy();
    }

    fn move_south(&mut self, by: u64) {
        self.y = (self.y.wrapping_sub(by)) % self.max_xy();
    }

    fn move_east(&mut self, by: u64) {
        self.x = (self.x.wrapping_sub(by)) % self.max_xy();
    }

    fn move_west(&mut self, by: u64) {
        self.x = (self.x.wrapping_add(by)) % self.max_xy();
    }

    fn set_f(&mut self, value: i64) -> Result<(), Error> {
        if self.min_f() <= value && value <= self.max_f() {
            self.f = value;
        } else {
            return Err(Error::FOutOfRange {
                f: value,
                z: self.z,
            });
        }
        Ok(())
    }

    fn set_x(&mut self, value: u64) -> Result<(), Error> {
        if value <= self.max_xy() {
            self.x = value;
        } else {
            return Err(Error::XOutOfRange {
                x: value,
                z: self.z,
            });
        }
        Ok(())
    }

    fn set_y(&mut self, value: u64) -> Result<(), Error> {
        if value <= self.max_xy() {
            self.y = value;
        } else {
            return Err(Error::YOutOfRange {
                y: value,
                z: self.z,
            });
        }
        Ok(())
    }

    fn to_encode(&self) -> crate::encode_id::EncodeID {
        todo!()
    }

    fn into_encode(self) -> crate::encode_id::EncodeID {
        todo!()
    }
}
