pub struct Node<T> {
    data: Option<T>,
    next_idx: Option<usize>,
    prev_idx: Option<usize>,
}

pub struct ArenaList<T> {
    pub nodes: Vec<Node<T>>,
    pub len: usize,
}


pub struct DoubleLinkedList<'a, T> {
    pub root_idx: usize,
    owner: &'a mut ArenaList<T>,
}


impl<T> ArenaList<T> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            len: 0,
        }
    }

    fn clear(&mut self) {
        self.nodes.clear();
    }

    // 新增一个节点
    fn add_node(&mut self, data: Option<T>) -> usize {
        self.nodes.push(
            Node { data, next_idx: None, prev_idx: None }
        );
        self.len += 1;
        self.len - 1
    }

    // 连接两个节点
    fn add_link(&mut self, src_idx: usize, dst_idx: usize) {
        self.nodes[src_idx].next_idx = Some(dst_idx);
        self.nodes[dst_idx].prev_idx = Some(src_idx);
    }

    //返回节点
    fn get_node(&self, idx: usize) -> &Node<T> {
        return &self.nodes[idx];
    }


    // 删除节点
    fn del_node(&mut self, idx: usize) -> bool {
        if idx >= self.nodes.len() {
            return false;
        }

        let last_idx = self.nodes.len() - 1;

        let mut node_to_del = self.nodes.swap_remove(idx);

        // step1: 上游节点不再指向它
        match node_to_del.prev_idx {
            None => { panic!("不可能") }
            Some(prev_idx) => {
                if prev_idx == last_idx {
                    // 如果上游节点是最后一个，注意它已经被移动到 idx 位置了
                    self.nodes[idx].next_idx = None;
                } else {
                    self.nodes[prev_idx].next_idx = None;
                }
            }
        }

        // step2：下游节点不再指向他
        match node_to_del.next_idx {
            None => {}
            Some(next_idx) => {
                if next_idx == last_idx {
                    self.nodes[idx].prev_idx = None;
                } else {
                    self.nodes[next_idx].prev_idx = None;
                }
            }
        }

        if idx == last_idx {
            return true;
        }


        // step3:被移动的节点，其上下游也要调整
        match self.nodes[idx].prev_idx {
            None => {}
            Some(prev_idx) => {
                if prev_idx == last_idx {
                    //     上游节点是自己
                    self.nodes[idx].next_idx = Some(idx);
                } else { self.nodes[prev_idx].next_idx = Some(idx); }
            }
        }

        match self.nodes[idx].next_idx {
            None => {}
            Some(next_idx) => {
                if next_idx == last_idx {
                    self.nodes[idx].prev_idx = Some(idx)
                } else { self.nodes[next_idx].prev_idx = Some(idx); }
            }
        }


        self.len -= 1;
        true
    }
}

impl<'a, T> DoubleLinkedList<'a, T> {
    pub fn new(arena_list: &'a mut ArenaList<T>) -> Self {
        let mut res = Self {
            root_idx: 0,
            owner: arena_list,
        };
        res.owner.add_node(None);
        res
    }

    pub fn from_vec(arena_list: &'a mut ArenaList<T>, vec1: Vec<T>) -> DoubleLinkedList<T> {
        let mut res = Self {
            root_idx: arena_list.len,
            owner: arena_list,
        };

        let mut curr_idx = res.root_idx;
        res.owner.add_node(None);// dummy


        for data in vec1 {
            let next_idx = res.owner.add_node(Some(data));
            res.owner.nodes[curr_idx].next_idx = Some(next_idx);
            res.owner.nodes[next_idx].prev_idx = Some(curr_idx);
            curr_idx = next_idx
        }
        res
    }

    pub fn to_vec(&self) -> Vec<&T> {
        let mut curr_idx = self.root_idx;

        let mut res = Vec::new();
        loop {
            match self.owner.nodes[curr_idx].next_idx {
                None => { break; }
                Some(next_idx) => {
                    match &self.owner.nodes[next_idx].data {
                        None => { break; }
                        Some(data) => { res.push(data); }
                    }
                    curr_idx = next_idx;
                }
            }
        }
        res
    }


    // 增加节点
    pub fn add_node(&mut self, data: Option<T>) -> usize {
        self.owner.add_node(data)
    }


    // 获取节点
    pub fn get_node_by_idx(&self, idx: usize) -> &Node<T> {
        return self.owner.get_node(idx);
    }


    pub fn insert(&mut self, num: usize, data: T) {
        let mut curr_idx = self.root_idx;
        let mut num = num;
        let new_idx = self.add_node(Some(data));

        loop {
            if num <= 0 {
                self.owner.nodes[new_idx].next_idx = self.owner.nodes[curr_idx].next_idx;
                self.owner.nodes[curr_idx].next_idx = Some(new_idx);
                break;
            }

            match self.owner.nodes[curr_idx].next_idx {
                None => break,
                Some(next_idx) => {
                    curr_idx = next_idx;
                }
            }

            num -= 1;
        }
    }

    pub fn get(&self, num: usize) {
        let mut curr_idx = self.root_idx;
    }

    pub fn del() {}
}
