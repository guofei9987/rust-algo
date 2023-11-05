use rust_algo::double_linked_list::{ArenaList, DoubleLinkedList};


#[test]
fn func1() {
    let mut arena_list = ArenaList::new();
    let mut double_linked_list = DoubleLinkedList::new(&mut arena_list);

    println!("{:?}", double_linked_list.to_vec());

    let vec1 = vec![1, 2, 4, 5, 6, 7];
    let mut double_linked_list = DoubleLinkedList::from_vec(&mut arena_list, vec1);

    println!("root_idx:{}", double_linked_list.root_idx);
    let vec2 = double_linked_list.to_vec();
    println!("{:?}", vec2);

    double_linked_list.insert(3, 5);
    println!("{:?}", double_linked_list.to_vec());


    let data1 = double_linked_list.get(3);
    println!("get data from num = 3 : {:?}", data1);


    double_linked_list.del(3);
    println!("{:?}", double_linked_list.to_vec());
    double_linked_list.del(2);
    println!("{:?}", double_linked_list.to_vec());


}