//! 時空間ID操作のトレイトとその実装
//!
//! このモジュールは、時空間IDの基本的な操作を定義する [`SpaceId`] トレイトと、
//! その実装（単一ID、範囲ID）を提供します。

use crate::error::Error;

pub mod constants;
pub mod encode;
pub mod helpers;
pub mod range;
pub mod segment;
pub mod single;

///本ライブラリが扱う空間IDは2種類存在します。二つの空間ID型はIPAの定めた標準的な空間ID形状を満たし、また相互に変換が可能です。
/// 求められるパフォーマンスに応じて二つのIDを使い分けることを推奨します。また、混在しても基本的に問題ない設計になっています。
///
/// SpaceID Traitは全ての空間ID型が満たすべき基本的な共通性質を定義しています。
///
/// # [SingleID]と[RangeID]の違い
///
/// # 移動の`bound`と`warp`について
///
/// # 使い分けのガイドライン
///
/// # 混在が可能かどうか
/// /// <details>
/// <summary>クリックして展開（日本語）</summary>
///
/// ここに詳細な説明を書く。
///
/// </details>
pub trait SpaceID {
    //そのIDの各次元の最大と最小を返す
    fn min_f(&self) -> i64;
    fn max_f(&self) -> i64;
    fn max_xy(&self) -> u64;

    //WEBメルカトル法や高度の上限に来るとエラーを出す
    fn bound_up(&mut self, by: i64) -> Result<(), Error>;
    fn bound_down(&mut self, by: i64) -> Result<(), Error>;
    fn bound_north(&mut self, by: u64) -> Result<(), Error>;
    fn bound_south(&mut self, by: u64) -> Result<(), Error>;
    fn bound_east(&mut self, by: u64) -> Result<(), Error>;
    fn bound_west(&mut self, by: u64) -> Result<(), Error>;

    //WEBメルカトル法や高度の上限に来ると反対側に循環する
    fn wrap_up(&mut self, by: i64);
    fn wrap_down(&mut self, by: i64);
    fn wrap_north(&mut self, by: u64);
    fn wrap_south(&mut self, by: u64);
    fn wrap_east(&mut self, by: u64);
    fn wrap_west(&mut self, by: u64);

    //中心を求める関数

    //頂点を求める関数
}
