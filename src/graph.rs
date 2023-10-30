/*
功能
- 增
    - 增加节点 add_node
    - 增加边 add_edge
- 查
    - get_node_by_idx
    - get_name_by_idx
    - TODO: 根据各种条件做查询
- 删
    - 删除节点，del_node_by_idx，del_node_by_name，
    - 删除边，del_edge_by_idx，del_edge_by_name
    - clear:清空所有数据
- 改
    - 改节点对应的值：add_node
- 高级功能
    - 遍历全部下游
    - 寻找最近链路
    - ？寻找关键链接
 */



use std::collections::{HashMap, HashSet, VecDeque};


pub struct Node<T> {
    name: String,
    data: T,
    next_idx: HashSet<usize>,
    prev_idx: HashSet<usize>,
    // 用 HashSet 防止边重复
}

pub struct ArenaList<T> {
    pub nodes_names: HashMap<String, usize>,
    // {node_name: idx}，用于快速检索边所在的 index
    pub nodes: Vec<Node<T>>,
    // 存放数据本身
}


impl<T> ArenaList<T> {
    fn new() -> Self {
        Self {
            nodes_names: HashMap::new(),
            nodes: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.nodes_names.clear();
        self.nodes.clear();
    }

    // 新增一个节点，并返回其索引号
    fn add_node(&mut self, name: &str, data: T) -> usize {
        // 如果已有节点名，则更新节点，如果没有则新建。返回节点号。
        match self.nodes_names.get(name) {
            Some(idx) => {
                self.nodes[*idx].data = data;
                *idx
            }
            None => {
                let len = self.nodes.len();
                self.nodes_names.insert(name.to_string(), len);
                self.nodes.push(Node {
                    name: name.to_string(),
                    data,
                    next_idx: HashSet::new(),
                    prev_idx: HashSet::new(),
                });
                len
            }
        }
    }

    fn get_node(&self, idx: usize) -> &Node<T> {
        return &self.nodes[idx];
    }

    // 根据 节点名字 返回 index
    fn get_idx_by_name(&self, name: &str) -> Option<&usize> {
        self.nodes_names.get(name)
    }
    fn get_name_by_idx(&self, idx: usize) -> &str {
        &self.nodes[idx].name
    }

    fn add_edge(&mut self, src_idx: usize, dst_idx: usize) {
        self.nodes[src_idx].next_idx.insert(dst_idx);
        self.nodes[dst_idx].prev_idx.insert(src_idx);
    }


    fn del_edge(&mut self, src_idx: usize, dst_idx: usize) -> bool {
        let res = self.nodes[src_idx].next_idx.remove(&dst_idx);
        self.nodes[dst_idx].next_idx.remove(&src_idx) && res
    }


    // 删除节点。移除指向该节点的边，然后移除该节点指向的边，最后移除节点
    fn del_node(&mut self, idx: usize) -> bool {
        if idx >= self.nodes.len() {
            return false;
        }

        let last_idx = self.nodes.len() - 1;

        // 要被删除的节点
        let node_to_del = self.nodes.swap_remove(idx);

        // step1:上游节点不再指向它
        for src_idx in node_to_del.prev_idx {
            if src_idx == last_idx {
                //如果上游节点恰好是最后一个，要注意它已经移动到新位置了
                self.nodes[idx].next_idx.remove(&idx);
            } else {
                self.nodes[src_idx].next_idx.remove(&idx);
            }
        }

        // step2:下游节点也不再指向它
        for dst_idx in node_to_del.next_idx {
            if dst_idx == last_idx {
                self.nodes[idx].prev_idx.remove(&idx);
            } else { self.nodes[dst_idx].prev_idx.remove(&idx); }
        }
        self.nodes_names.remove(&node_to_del.name);

        // 如果要删除的恰好是最后一个，到此结束
        if idx == last_idx {
            return true;
        }

        // step3: 更正其上游向它的指向
        for src_idx in self.nodes[idx].prev_idx.clone() {
            if src_idx == last_idx { // 上游节点是自己
                self.nodes[idx].next_idx.remove(&last_idx);
                self.nodes[idx].next_idx.insert(idx);
            } else {
                self.nodes[src_idx].next_idx.remove(&last_idx);
                self.nodes[src_idx].next_idx.insert(idx);
            }
        }
        // step4: 更正其下游向它的指向
        for dst_idx in self.nodes[idx].next_idx.clone() {
            if dst_idx == last_idx {
                self.nodes[idx].prev_idx.remove(&last_idx);
                self.nodes[idx].prev_idx.insert(idx);
            } else {
                self.nodes[dst_idx].prev_idx.remove(&last_idx);
                self.nodes[dst_idx].prev_idx.insert(idx);
            }
        }

        // step5: 更新 nodes_names
        // 不用 insert，因为会有 key 字符串复制
        match self.nodes_names.get_mut(&self.nodes[idx].name) {
            None => { panic!("del_node 不应该出现的错误") }
            Some(node_idx) => { *node_idx = idx }
        }
        true
    }
}


pub struct Graph<'a, T> {
    owner: &'a mut ArenaList<T>,
}

impl<'a, T> Graph<'a, T> {
    fn new(arena_list: &'a mut ArenaList<T>) -> Self {
        Self {
            owner: arena_list
        }
    }

    // 增加节点
    fn add_node(&mut self, name: &str, data: T) -> usize {
        self.owner.add_node(name, data)
    }

    // 增加边
    fn add_edge(&mut self, src_idx: usize, dst_idx: usize) {
        self.owner.add_edge(src_idx, dst_idx);
    }

    // 增加边
    fn add_node_and_edge(&mut self, src_name: &str, src_data: T, dst_name: &str, dst_data: T) {
        let src_idx = self.add_node(src_name, src_data);
        let dst_idx = self.add_node(dst_name, dst_data);
        self.add_edge(src_idx, dst_idx);
    }


    // 获取节点
    fn get_node_by_idx(&self, idx: usize) -> &Node<T> {
        return self.owner.get_node(idx);
    }

    fn get_name_by_idx(&self, idx: usize) -> &String {
        return &self.get_node_by_idx(idx).name;
    }

    fn get_idx_by_name(&self, name: &str) -> Option<&usize> {
        self.owner.get_idx_by_name(name)
    }


    // 获取所有的边，其中的节点以 index 的形式给出
    fn get_all_edges(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for src_idx in 0..self.owner.nodes.len() {
            for idx in &self.owner.nodes[src_idx].next_idx {
                res.push((src_idx, *idx));
            }
        }
        res
    }

    // 打印所有节点的 name
    fn print_nodes(&self) {
        println!("{:?}", self.owner.nodes.iter().map(|x| x.name.clone()).collect::<Vec<String>>());
    }
    // 打印所有的边
    fn print_edges(&self) {
        // 获取所有的边(返回值是节点的 index)
        let edges = self.get_all_edges();
        for (src_idx, dst_idx) in edges {
            println!("{:?}->{:?}", self.get_name_by_idx(src_idx), self.get_name_by_idx(dst_idx));
        }
    }

    // 删除节点
    fn del_node_by_idx(&mut self, idx: usize) -> bool { self.owner.del_node(idx) }
    // 删除边
    fn del_edge_by_idx(&mut self, src_idx: usize, dst_idx: usize) -> bool { self.owner.del_edge(src_idx, dst_idx) }

    // 删除节点
    fn del_node_by_name(&mut self, name: &str) -> bool {
        match self.get_idx_by_name(name) {
            None => { false }
            Some(i) => { self.del_node_by_idx(*i) }
        }
    }

    // 删除边
    fn del_edge_by_name(&mut self, src_name: &str, dst_name: &str) -> bool {
        let src_idx = self.get_idx_by_name(src_name);
        let dst_idx = self.get_idx_by_name(dst_name);
        if let (Some(src_idx), Some(dst_idx)) = (src_idx, dst_idx) {
            self.del_edge_by_idx(*src_idx, *dst_idx)
        } else { false }
    }

    fn clear(&mut self) { self.owner.clear() }

    // 保存
    fn save(&self) {}

    // 加载
    fn load(&mut self) {}
}

impl<'a, T> Graph<'a, T> {
    // 找到某个节点的全部下游节点。返回的数据结构是 {level: [idx1, idx2, ...]} 的 HashMap，存放的是下游层数、节点号
    fn get_downstream(&self, batch_idx: Vec<usize>, max_level: usize) -> HashMap<usize, Vec<usize>> {
        let mut res: HashMap<usize, Vec<usize>> = HashMap::new();
        let mut q: Vec<usize> = batch_idx.clone();
        let mut searched = HashSet::new(); // 存放已经被遍历到的节点。用来排除掉环状节点
        let mut level = 0;
        while !q.is_empty() && level < max_level {
            res.insert(level, q.clone());
            searched.extend(q.clone());
            q = q.iter().flat_map(|&node_idx| {
                self.owner.nodes[node_idx].next_idx.iter()
                    .filter(|&next_idx| !searched.contains(next_idx)).copied()
            }).collect();
            level += 1;
        }
        res
    }

    // 计算两个节点之间的最短距离
    fn get_shortest(&self, src_idx: usize, dst_idx: usize, max_level: usize) -> Option<usize> {
        let mut q: Vec<usize> = vec![src_idx];
        let mut searched = HashSet::new(); // 存放已经被遍历到的节点。用来识别并排除掉环状节点
        let mut level = 0;

        while !q.is_empty() && level < max_level {
            if q.contains(&dst_idx) {
                return Some(level);
            }
            searched.extend(q.clone());
            q = q.iter().flat_map(|&node_idx| {
                self.owner.nodes[node_idx].next_idx.iter()
                    .filter(|&next_idx| !searched.contains(next_idx)).copied()
            }).collect();
            level += 1;
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::graph::{ArenaList, Graph};



    #[test]
    fn test1() {
        let mut arena_list = ArenaList::new();
        let mut graph = Graph::new(&mut arena_list);
        let vec1 = vec![
            ("John", "Emma")
            , ("Sophia", "Tom")
            , ("Isabella", "Emma")
            , ("Tom", "Isabella")
            , ("Tom", "John")
            , ("Tom", "Michael")
            , ("John", "Emma")
            , ("Tom", "Sophia")
            , ("Oliver", "Emma")
            , ("Michael", "Daniel")
            , ("Michael", "Lucy")
            , ("Sophia", "Michael")
            , ("Oliver", "Lucy")
            , ("Sophia", "Emily")
            , ("Michael", "Daniel")
            , ("Sophia", "Michael")
            , ("Michael", "Sophia")
            , ("John", "Emma")
            , ("Tom", "Sophia")
            , ("Sophia", "John")]
            ;

        for (src_name, dst_name) in vec1 {
            graph.add_node_and_edge(
                src_name, src_name.to_string(),
                dst_name, dst_name.to_string());
        }

        // 获取所有的边(返回值是节点的 index)
        let edges = graph.get_all_edges();

        // 打印所有的边
        graph.print_edges();

        // 打印所有节点
        graph.print_nodes();

        graph.del_edge_by_name("Michael", "Lucy");
        println!("======after del edge [Sophia]-> [Lucy]：======");
        graph.print_edges();

        graph.del_node_by_name("Sophia");
        println!("======after del node 【Sophia】：======");
        graph.print_edges();
        graph.clear();
    }


    #[test]
    fn test2() {
        let mut arena_list = ArenaList::new();
        let mut graph = Graph::new(&mut arena_list);
        let vec1 = vec![
            ("John", "Emma")
            , ("Sophia", "Tom")
            , ("Isabella", "Emma")
            , ("Tom", "Isabella")
            , ("Tom", "John")
            , ("Tom", "Michael")
            , ("John", "Emma")
            , ("Tom", "Sophia")
            , ("Oliver", "Emma")
            , ("Michael", "Daniel")
            , ("Michael", "Lucy")
            , ("Sophia", "Michael")
            , ("Oliver", "Lucy")
            , ("Sophia", "Emily")
            , ("Michael", "Daniel")
            , ("Sophia", "Michael")
            , ("Michael", "Sophia")
            , ("John", "Emma")
            , ("Tom", "Sophia")
            , ("Sophia", "John")]
            ;

        for (src_name, dst_name) in vec1 {
            graph.add_node_and_edge(
                src_name, src_name.to_string(),
                dst_name, dst_name.to_string());
        }

        let idxes = vec![2];
        let level_order = graph.get_downstream(idxes, 100000000);
        for level in 0..level_order.len() {
            println!("[level = {}], idx = {:?}", level, level_order.get(&level));
        }

        println!("=====print names=====");
        for level in 0..level_order.len() {
            let node_names: Vec<&String> =
                level_order.get(&level).unwrap()
                    .iter().map(|idx| graph.get_name_by_idx(*idx))
                    .collect();
            println!("[level = {}], names = {:?}", level, node_names);
        }

        println!("2 到 7 的最短路径长度为： {}",graph.get_shortest(2,7,1000000).unwrap());
    }
}