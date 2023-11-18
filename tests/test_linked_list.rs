use rust_algo::linked_list::{ArenaList, LinkedList};

#[test]
fn func1() {}


#[test]
fn test1() {
    let mut arena_list = ArenaList::new();
    let vec1 = vec![1, 2, 3, 4, 5, 6];
    let mut linked_list = LinkedList::from_vec(&mut arena_list, vec1);
    println!("{:?}", linked_list.to_vec());
    linked_list.insert(3, 9);
    linked_list.insert(0, 99);
    println!("{:?}", linked_list.to_vec());
    println!("index = {}, val = {:?}", 0, linked_list.get(0));
    println!("index = {}, val = {:?}", 3, linked_list.get(3));
    println!("index = {}, val = {:?}", 8, linked_list.get(8));
    linked_list.del(3);
    linked_list.del(2);
    println!("{:?}", linked_list.to_vec());
}

#[test]
fn test2() {
    let mut arena_list = ArenaList::new();
    let vec1 = vec![1, 2, 3, 4, 5, 6];
    let mut linked_list1 = LinkedList::from_vec(&mut arena_list, vec1);
    let mut linked_list2 = linked_list1.split(3);
    println!("{:?}", linked_list2.to_vec());
    println!("{:?}", linked_list1.to_vec());
    //     颠倒过来会发生生命周期冲突，之后解决
}

#[test]
fn test3() {
    let mut arena_list = ArenaList::new();
    let vec1 = vec!["Tom", "Lucy", "Lily", "Bob", "Li", "Wang"];

    // 从向量新建一个链表
    let mut linked_list = LinkedList::from_vec(&mut arena_list, vec1);

    // 链表数据放入向量
    println!("{:?}", linked_list.to_vec());
    // 插入
    linked_list.insert(3, "Zhao");
    linked_list.insert(0, "Zhang");
    println!("{:?}", linked_list.to_vec());

    // 获取数据
    println!("index = {}, val = {:?}", 0, linked_list.get(0));
    println!("index = {}, val = {:?}", 3, linked_list.get(3));
    println!("index = {}, val = {:?}", 8, linked_list.get(8));

    // 删除数据
    linked_list.del(3);
    linked_list.del(2);
    println!("{:?}", linked_list.to_vec());
}
