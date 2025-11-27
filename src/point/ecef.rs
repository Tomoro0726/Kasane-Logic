#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "ts-rs")]
use ts_rs::TS;

use super::{coordinate::Coordinate, Point};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "ts-rs", derive(TS))]
pub struct ECEF {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl ECEF {
    /// 新しいECEF座標を作成
    pub fn new(x: f64, y: f64, z: f64) -> ECEF {
        ECEF { x, y, z }
    }
}

impl Point for ECEF {
    fn to_coordinate(&self) -> Coordinate {
        let a = 6378137.0_f64; // 長半径
        let inv_f = 298.257223563_f64;
        let f = 1.0 / inv_f;
        let b = a * (1.0 - f);
        let e2 = 1.0 - (b * b) / (a * a);

        let x = self.x;
        let y = self.y;
        let z = self.z;

        let lon = y.atan2(x);
        let p = (x * x + y * y).sqrt();

        // 緯度の初期値（Bowring の公式）
        let mut lat = (z / p).atan2(1.0 - f);
        let mut h = 0.0;

        // Newton-Raphson 反復
        for _ in 0..10 {
            let sin_lat = lat.sin();
            let n = a / (1.0 - e2 * sin_lat * sin_lat).sqrt();
            h = p / lat.cos() - n;
            let new_lat = (z + e2 * n * sin_lat).atan2(p);

            // 収束チェック（1e-12 ≈ 数 mm）
            if (new_lat - lat).abs() < 1e-12 {
                lat = new_lat;
                break;
            }
            lat = new_lat;
        }

        Coordinate {
            latitude: lat.to_degrees(),
            longitude: lon.to_degrees(),
            altitude: h,
        }
    }

    fn to_ecef(&self) -> ECEF {
        *self
    }
}
