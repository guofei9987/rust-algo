/*
删除时节点置 None，常数复杂度
first_free 之后的位置可以使用，不用额外 alloc，相当于对象池
free nodes 也可以提前构造和存下来，用空间换时间
空洞太多手动执行 compact，缓存友好
生命周期结束后集体释放，总体性能更好
*/



pub struct ArenaList<T> {
    pub nodes: Vec<NodeInfo<T>>,
    first_free: usize,
    len: usize,
}

pub struct NodeInfo<T> {
    pub data: Option<T>,
    pub next_idx: Option<usize>,
    // prev_sibling: Option<usize>,
}

pub struct Node<'a, T> {
    id: usize,
    // dummy 对应的位置
    owner: &'a mut ArenaList<T>,
}

impl<T> ArenaList<T> {
    fn new() -> Self {
        return ArenaList {
            nodes: Vec::new(),
            first_free: 0,
            len: 0,
        };
    }

    // 新增一个节点，并返回其索引号
    fn make_node(&mut self, data: Option<T>) -> usize {
        self.nodes.push(NodeInfo { data, next_idx: None });
        self.first_free += 1;
        return self.first_free - 1;
    }

    fn compact(&mut self) {
        //     消灭空洞，整理 ArenaList
        // 问题：如果有多个链表，是启动一个 ArenaList 还是多个？
        // 如果启动多个，链表合并/链表拆分算法如何做？
        // 如果启动1个，compact 会比较麻烦
        // 结论：启动1个，在做链表合并/拆分的时候，额外生成。

        // compact 过程：
        // 考虑到 图/树 很可能是没有顺序的，因此 compact 过程不用按照顺序整理
        // 可以维护一个 表示空洞的 queue/vec，使其利用率提高
    }

    fn clear(&mut self) {
        self.first_free = 0;
        self.nodes.clear();
    }
}


impl<'a, T> Node<'a, T> {
    fn init_with_dummy(arena_list: &'a mut ArenaList<T>) -> Self {
        let idx = arena_list.make_node(None);
        Self {
            id: idx,
            owner: arena_list,
        }
    }

    fn push_by_vec(&mut self, vec: Vec<T>) {
        let mut prev = self.id; // dummy，其实应该是 tail
        for data in vec {
            let curr = self.owner.make_node(Some(data));
            self.owner.nodes[prev].next_idx = Some(curr);
            prev = curr;
        }
    }
    fn from_vec(arena_list: &'a mut ArenaList<T>, vec: Vec<T>) -> Self {
        let mut res = Self::init_with_dummy(arena_list);
        res.push_by_vec(vec);
        res
    }

    fn to_vec(&self) -> Vec<&T> {
        let mut res: Vec<&T> = Vec::new();
        let mut curr = self.id;
        loop {
            match self.owner.nodes[curr].next_idx {
                Some(next_node) => {
                    curr = next_node;
                    res.push(self.owner.nodes[curr].data.as_ref().unwrap());
                }
                None => break
            }
        }
        res
    }

    fn insert(&mut self, mut idx: usize, val: T) -> bool {
        // 插入成功返回 true，插入失败（也就是idx溢出）返回 false
        let insert_idx = self.owner.make_node(Some(val));
        let mut curr = self.id;
        loop {
            if idx == 0 {
                self.owner.nodes[insert_idx].next_idx = self.owner.nodes[curr].next_idx;
                self.owner.nodes[curr].next_idx = Some(insert_idx);
                return true;
            }

            match self.owner.nodes[curr].next_idx {
                Some(next_node) => {
                    curr = next_node;
                    idx -= 1;
                }
                None => { return false; }
            }
        }
    }

    fn get(&self, mut idx: usize) -> &Option<T> {
        let mut curr_idx = self.id;
        loop {
            match self.owner.nodes[curr_idx].next_idx {
                Some(next_node) => {
                    if idx == 0 {
                        return &self.owner.nodes[next_node].data;
                    }
                    curr_idx = next_node;
                    idx -= 1;
                }
                None => { return &None; }
            }
        }
    }

    fn del(&mut self, mut idx: usize) -> bool {
        let mut curr_idx = self.id;
        loop {
            match self.owner.nodes[curr_idx].next_idx {
                Some(next_idx) => {
                    if idx == 0 {
                        self.owner.nodes[curr_idx].next_idx = self.owner.nodes[next_idx].next_idx;
                        //     之后 next_idx 已经不被引用了，但是不在这里释放其内存空间
                        // 而是在空洞太多时使用 compact 批量释放
                        return true;
                    }

                    curr_idx = next_idx;
                    idx -= 1;
                }
                None => { return false; }
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::history::linked_list::{ArenaList, Node};

    #[test]
    fn test2() {
        let mut arena_list: ArenaList<i32> = ArenaList::new();
        let vec1 = vec![1, 3, 2, 7, 6, 5];

        let mut node = Node::from_vec(&mut arena_list, vec1);
        let vec2 = node.to_vec();
        println!("{:?}", vec2);

        node.insert(3, 9);
        println!("{:?}", node.to_vec());

        node.insert(0, 0);
        println!("{:?}", node.to_vec());

        let status = node.insert(999, 0);
        println!("{:?}｜insert status:{}", node.to_vec(), status);

        let idx = 3;
        let val = node.get(idx);
        match val {
            None => { println!("get None at idx = {}", idx) }
            Some(x) => { println!("idx = {}, get val = {}", idx, x) }
        }

        node.del(3);
        println!("After del: {:?}", node.to_vec());
    }
}



