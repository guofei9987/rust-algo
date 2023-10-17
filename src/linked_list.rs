/*
删除时节点置 None，常数复杂度
first_free 之后的位置可以使用，不用额外 alloc，相当于对象池
free nodes 也可以提前构造和存下来，用空间换时间
空洞太多手动执行 compact，缓存友好
生命周期结束后集体释放，总体性能更好
*/


pub struct ArenaList<T> {
     nodes: Vec<NodeInfo<T>>,
     first_free: usize,
     len: usize,
}

struct NodeInfo<T> {
    data: Option<T>,
    next_sibling: Option<usize>,
    prev_sibling: Option<usize>,
}

pub struct Node<'a, T> {
    id: usize,
    owner: &'a ArenaList<T>,
}

impl<'a, T> Node<'a, T> {
    fn init(arena_list: &'a ArenaList<T>) -> Self {
        Node {
            id: 0,
            owner: arena_list,
        }
    }

}


#[cfg(test)]
mod tests {

}

