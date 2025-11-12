use kasane_logic::{space_time_id::SpaceTimeId, space_time_id_set::SpaceTimeIdSet};

fn main() {
    let mut set = SpaceTimeIdSet::new();
    let id = SpaceTimeId::new(
        4,
        [Some(3), Some(2)],
        [Some(3), Some(5)],
        [Some(3), Some(5)],
        0,
        [None, None],
    )
    .unwrap();

    let id2 = SpaceTimeId::new(
        5,
        [Some(3), Some(2)],
        [Some(3), Some(5)],
        [Some(3), Some(5)],
        0,
        [None, None],
    )
    .unwrap();

    set.insert(id);
    set.insert(id2);

    println!("{}", id);
    println!("{}", id2);

    println!("-------------");

    for ele in set.get_all() {
        println!("{},", ele);
    }
}
