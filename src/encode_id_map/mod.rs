use std::collections::{BTreeMap, HashMap, HashSet};

use crate::{bit_vec::BitVec, encode_id::EncodeID};

///Set内部におけるIDの一意な番号
type Index = usize;

pub mod difference;
pub mod get;
pub mod insert;
pub mod intersection;
pub mod iterator;
pub mod remove;
pub mod union;
pub mod utils;

/// 階層ごとの情報を保持する構造体
#[derive(Debug, Clone)]
pub struct LayerInfo {
    //その階層が持つ実際のIDのIndex
    pub index: HashSet<Index>,

    //その階層の下にあるIDの個数
    pub count: usize,
}

/// 時空間IDから任意の値へのマッピングを効率的に管理するデータ構造
/// 重複する範囲を自動的に統合し、階層構造を用いて効率的に格納する。
#[derive(Debug, Clone)]
pub struct EncodeIDMap<V> {
    //各次元の範囲を保存するためのBTreeMap
    f: BTreeMap<BitVec, LayerInfo>,
    x: BTreeMap<BitVec, LayerInfo>,
    y: BTreeMap<BitVec, LayerInfo>,
    index: usize,
    reverse: HashMap<Index, (EncodeID, V)>,
}
impl<V> EncodeIDMap<V> {
    /// 新しい空の時空間IDマップを作成
    pub fn new() -> Self {
        Self {
            f: BTreeMap::new(),
            x: BTreeMap::new(),
            y: BTreeMap::new(),
            index: 0,
            reverse: HashMap::new(),
        }
    }
}

impl<V> Default for EncodeIDMap<V> {
    fn default() -> Self {
        Self::new()
    }
}
