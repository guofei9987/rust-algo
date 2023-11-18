/*
ArenaList，相当于对象池
删除时节点置 None，常数复杂度

用 nodes: Vec<Option<T>> + nexts: Vec<Option<usize>> 来管理节点数据，以及节点指向的下一个序号
也可以用 nodes: Vec<NodeInfo<T>> 来管理，其中 NodeInfo{data, next_idx}

holes 用来存放孔洞，出现孔洞后，后续的插入优先使用孔洞。孔洞的使用是用 栈 的方式
*/


pub struct ArenaList<T> {
    pub nodes: Vec<Option<T>>,
    // 存放数据本身
    pub nexts: Vec<Option<usize>>,
    // 存放节点指向的下一个节点。若为 None，表示没有下一个节点
    pub holes: Vec<usize>,
    // 存放孔洞对应的 index
}

impl<T> ArenaList<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            nexts: Vec::new(),
            holes: Vec::new(),
        }
    }

    //     新增一个节点，并返回其索引号
    fn make_node(&mut self, data: Option<T>) -> usize {
        match self.holes.pop() {
            // 如果有空洞，新数据放到空洞上
            Some(new_idx) => {
                self.nodes[new_idx] = data;
                self.nexts[new_idx] = None;
                new_idx
            }
            // 如果没有空洞
            None => {
                self.nodes.push(data);
                self.nexts.push(None);
                self.nexts.len() - 1
            }
        }
    }
}

pub struct LinkedList<'a, T> {
    root: usize,
    //根节点id
    owner: &'a mut ArenaList<T>,
}

impl<'a, T> LinkedList<'a, T> {
    pub fn from_vec(arena_list: &'a mut ArenaList<T>, vec1: Vec<T>) -> Self {
        let dummy = arena_list.make_node(None);
        let mut prev = dummy;
        for data in vec1 {
            let curr = arena_list.make_node(Some(data));
            arena_list.nexts[prev] = Some(curr);
            prev = curr;
        }

        Self {
            root: dummy,
            owner: arena_list,
        }
    }

    fn clear() {}


    pub fn to_vec(&self) -> Vec<&T> {
        let mut res = Vec::new();
        let mut curr_idx = self.root;
        loop {
            match self.owner.nexts[curr_idx] {
                Some(next_idx) => {
                    // 应该不会出现对应 next 不为 None，但 nodes 为 None 的情况
                    match &self.owner.nodes[next_idx] {
                        Some(node_data) => res.push(node_data),
                        None => break
                    }
                    curr_idx = next_idx;
                }
                None => break
            }
        }
        res
    }

    pub fn get(&self, mut num: usize) -> &Option<T> {
        let mut curr_idx = self.root;
        loop {
            match self.owner.nexts[curr_idx] {
                Some(next_idx) => {
                    if num == 0 {
                        return &self.owner.nodes[next_idx];
                    }
                    curr_idx = next_idx;
                    num -= 1;
                }
                None => { return &None; }
            }
        }
    }

    pub fn insert(&mut self, mut num: usize, data: T) {
        let new_idx = self.owner.make_node(Some(data));
        let mut curr_idx = self.root;
        loop {
            match self.owner.nexts[curr_idx] {
                Some(next_idx) => {
                    if num == 0 {
                        self.owner.nexts[new_idx] = Some(next_idx);
                        self.owner.nexts[curr_idx] = Some(new_idx);
                        curr_idx = new_idx;
                        break;
                    }
                    curr_idx = next_idx;
                    num -= 1;
                }
                None => break
            }
        }
    }


    pub fn del(&mut self, mut num: usize) -> bool {
        let mut curr_idx = self.root;
        loop {
            match self.owner.nexts[curr_idx] {
                Some(next_idx) => {
                    if num == 0 {
                        self.owner.nexts[curr_idx] = self.owner.nexts[next_idx];
                        self.owner.nexts[next_idx] = None;
                        self.owner.nodes[next_idx] = None;
                        self.owner.holes.push(next_idx);
                        return true;
                    }
                    curr_idx = next_idx;
                    num -= 1;
                }
                None => {
                    return false;
                }
            }
        }
    }
}

impl<'a, T> LinkedList<'a, T> {
    // 示例：如何操作多个 Linked List
    // 多个 Linked List 的节点存放在同一个 arena_list。只是不同的 LinkedList 对象的 root 节点不一样
    pub fn split(&mut self, mut num: usize) -> LinkedList<T> {
        let dummy = self.owner.make_node(None);
        let mut curr_idx = self.root;
        loop {
            match self.owner.nexts[curr_idx] {
                Some(next_idx) => {
                    if num == 0 {
                        self.owner.nexts[dummy] = Some(next_idx);
                        self.owner.nexts[curr_idx] = None;
                        break;
                    }
                    curr_idx = next_idx;
                    num -= 1;
                }
                None => { break; }
            }
        }
        LinkedList { root: dummy, owner: self.owner }
    }
}
