use crate::bit_vec::BitVec;

impl BitVec {
    pub fn division(mut target: BitVec, mut division: Vec<BitVec>) -> Vec<BitVec> {
        if division.is_empty() {
            return vec![target];
        }

        let mut result = Vec::new();

        // 最初の division を使って target を分割
        if let Some(mut first) = division.pop() {
            let splitted = BitVec::split_dimension(&target, &mut first);
            result.extend(splitted);
        }

        // 残りの division について順番に分割
        while let Some(mut div) = division.pop() {
            let mut new_result = Vec::new();

            for bit in result.into_iter() {
                let (start, end) = bit.under_prefix();
                let (d_start, d_end) = div.under_prefix();

                // 範囲が重なれば分割
                if d_start < end && start < d_end {
                    let splitted = BitVec::split_dimension(&bit, &mut div);
                    new_result.extend(splitted);
                } else {
                    // 重ならなければそのまま残す
                    new_result.push(bit);
                }
            }

            result = new_result;
        }

        result
    }

    pub fn split_dimension(top: &BitVec, under: &mut BitVec) -> Vec<BitVec> {
        let mut result = vec![];

        // 無限ループ防止
        if under.0.len() < top.0.len() {
            panic!("under が top より小さすぎます");
        }

        while top != under {
            under.reverse_bottom_layer();
            result.push(under.clone());
            under.remove_bottom_layer();

            if under.0.is_empty() {
                panic!("top に到達せず BitVec が空になりました");
            }
        }

        result
    }
}
