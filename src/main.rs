use kasane_logic::{
    bit_vec::BitVec, space_time_id::SpaceTimeId, space_time_id_set::SpaceTimeIdSet,
};

fn main() {
    // let test1 = BitVec::from_vec(vec![0b10101010, 0b11000000]);

    // let (start, end) = test1.under_prefix();
    // println!("     :{}", test1);
    // println!("START:{}", start);
    // println!("END  :{}", end);

    let mut set = SpaceTimeIdSet::new();

    let id1 = SpaceTimeId::new(
        4,
        [Some(15), Some(15)],
        [Some(1), Some(3)],
        [Some(6), Some(14)],
        0,
        [None, None],
    )
    .unwrap();

    let id2 = SpaceTimeId::new(
        4,
        [Some(15), Some(15)],
        [Some(3), Some(3)],
        [Some(13), Some(14)],
        0,
        [None, None],
    )
    .unwrap();

    // let id1 = SpaceTimeId::random_z_max(5);
    // let id2 = SpaceTimeId::random_z_max(5);

    println!("{}", id1);
    println!(",{}", id2);
    println!("-----------");
    println!("-----------");

    set.insert(id2);
    set.insert(id1);

    for ele in set.get_all() {
        println!("{},", ele);
    }
}
