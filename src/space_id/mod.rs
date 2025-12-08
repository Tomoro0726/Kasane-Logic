use crate::{encode_id::EncodeID, error::Error};

pub mod constants;
pub mod encode;
pub mod range_id;
pub mod single_id;

trait SpaceID {
    fn min_f(&self) -> i64;
    fn max_f(&self) -> i64;
    fn max_xy(&self) -> u64;

    fn move_up(&mut self, by: i64) -> Result<(), Error>;
    fn move_down(&mut self, by: i64) -> Result<(), Error>;
    fn move_north(&mut self, by: u64);
    fn move_south(&mut self, by: u64);

    fn move_east(&mut self, by: u64);
    fn move_west(&mut self, by: u64);

    fn set_f(&mut self, value: i64) -> Result<(), Error>;
    fn set_x(&mut self, value: u64) -> Result<(), Error>;
    fn set_y(&mut self, value: u64) -> Result<(), Error>;

    fn to_encode(&self) -> EncodeID;
    fn into_encode(self) -> EncodeID;
}
