use std::fmt;

use crate::space_id::single_id::SingleID;

impl fmt::Display for SingleID {
    ///読み方の定義をした
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}/{}/{}", self.z, self.f, self.x, self.y)
    }
}
