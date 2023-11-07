use rust_algo::double_linked_list;
use rust_algo::double_linked_list::{ArenaList, DoubleLinkedList};


#[test]
fn func1() {
    let mut arena_list = ArenaList::new();
    let mut double_linked_list = DoubleLinkedList::new(&mut arena_list);

    // double_linked_list.add_node(Some(3));


    let vec1 = vec![1, 2, 4, 5, 6, 7];
    let mut double_linked_list = DoubleLinkedList::from_vec(&mut arena_list, vec1);


    println!("root_idx:{}", double_linked_list.root_idx);
    let vec2 = double_linked_list.to_vec();
    println!("{:?}", vec2);

    double_linked_list.insert(3, 5);
    println!("{:?}", double_linked_list.to_vec());
}