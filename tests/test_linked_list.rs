use crate::linked_list::{ArenaList, Node, NodeInfo};

#[test]
fn func1() {
    let vec1 = vec![1, 2, 3, 4, 5];
    let mut arena_list = ArenaList {
        nodes: vec![NodeInfo {
            data: Some(vec1[0]),
            next_sibling: Some(1),
            prev_sibling: Some(0),
        }],
        first_free: 0,
        len: 0,
    };


    let head = Node::init(&arena_list);




    // let vec1: Vec<i32> = vec![1, 2, 3, 4];
    // let head = Node::from_vec(vec1);
    //
    // let a = head.to_vec();
    // println!("{:?}", a);
}