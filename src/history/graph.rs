/*
功能
- 增
    - 增加节点
    - 增加边
- 查
    - 根据 src 的条件，查询 srv
    - 根据边，查边
    - 根据 (src)-[edge]->(dst)，返回结构化数据
- 删
    - 删除边/节点
    - clear
- 改
    - 改边
    - 改节点属性值
- 高级功能
    - 遍历全部下游
    - 寻找最近链路
    - ？寻找关键链接

不再把 nodes、nexts 等作为不同的元素分开管理，而是用 Nodes 来管理。唯一目的是提高代码可读性（可能会降低性能）

 */

use std::collections::{HashMap, HashSet};
use std::process::id;


pub struct Node<T> {
    name: String,
    data: T,
    next_idx: HashSet<usize>,
    // 用 HashSet 防止边重复
}

pub struct ArenaList<T> {
    pub nodes_names: HashMap<String, usize>,
    // {node_name: idx}，用于快速检索边所在的 index
    pub nodes: Vec<Option<Node<T>>>,
    // 存放数据本身
    pub holes: Vec<usize>,
    // 存放孔洞对应的 index
}


impl<T> ArenaList<T> {
    fn new() -> Self {
        Self {
            nodes_names: HashMap::new(),
            nodes: Vec::new(),
            holes: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.nodes_names.clear();
        self.nodes.clear();
        self.holes.clear();
    }

    // 新增一个节点，并返回其索引号
    fn make_or_get_node(&mut self, name: &str, data: T) -> usize {
        // 如果已有节点名，不新建节点，而是返回其序号。如果没有节点名，新造节点。
        if self.nodes_names.contains_key(name) {
            return *(self.nodes_names.get(name).unwrap());
        } else {
            let len = self.nodes.len();
            self.nodes_names.insert(name.to_string(), len);
            self.nodes.push(Some(Node {
                name: name.to_string(),
                data,
                next_idx: HashSet::new(),
            }));
            len
        }
    }

    // 根据 节点名字 返回 index
    fn get_idx_by_name(&self, name: &str) -> Option<&usize> {
        self.nodes_names.get(name)
    }

    // 新增边
    fn add_next_idx(&mut self, src_idx: usize, dst_idx: usize) -> bool {
        match self.nodes[src_idx] {
            None => { false }
            //  ref
            Some(ref mut node) => {
                node.next_idx.insert(dst_idx);
                true
            }
        }
    }

    // 删除节点。直接置 None，而不处理其上游指向
    fn del_node_by_idx(&mut self, idx: usize) -> bool {
        match self.nodes[idx] {
            None => { false }
            Some(ref mut node) => {
                self.nodes_names.remove(&node.name);
                self.nodes[idx] = None;
                self.holes.push(idx);
                true
            }
        }
    }


    // 删除边
    fn del_edge_by_idx(&mut self, src_idx: usize, dst_idx: usize) -> bool {
        match self.nodes[src_idx] {
            None => false,
            Some(ref mut node) => {
                node.next_idx.remove(&dst_idx)
            }
        }
    }
    fn compact(&mut self) {
        // 消除空洞
    }

    // fn compact(&mut self) {
    //     self.holes.sort();// 排序
    //
    //     while !self.holes.is_empty() {
    //         let last_node = self.nodes.pop();
    //         match last_node {
    //             None => break, // self.nodes 已空，就跳出
    //             Some(last_node) => {
    //                 match last_node {
    //                     None => {
    //                         // 如果最后一个是空节点，那么继续下次循环，直到遇到非空节点
    //
    //                         // TODO: holes 需要改成 let holes=    VecDeque::from(&self.holes);
    //                         // 遇到空节点，必然 holes 有对应的数（assert 一下）
    //                         // 对应的数要删除
    //                         self.holes.contains(),
    //                         continue;
    //                     }
    //                     Some(last_node) => {
    //                         let
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     let hole = self.holes.pop();
    //     match hole {
    //         None => continue,
    //         Some(hole_idx) => {}
    //     }
    // }
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
        self.owner.make_or_get_node(name, data)
    }


    // 增加边
    fn add_edge(&mut self, src_name: &str, src_data: T, dst_name: &str, dst_data: T) {
        let src_idx = self.add_node(src_name, src_data);
        let dst_idx = self.add_node(dst_name, dst_data);
        self.owner.add_next_idx(src_idx, dst_idx);
    }


    // 获取节点
    fn get_node_by_idx(&self, idx: usize) -> Option<&Node<T>> {
        // TODO:这个把 &Option<Node<T>>，变成 Option<&Node<T>>
        return (&self.owner.nodes[idx]).as_ref();
    }

    fn get_name_by_idx(&self, idx: usize) -> Option<&String> {
        match self.get_node_by_idx(idx) {
            None => None,
            Some(node) => { Some(&node.name) }
        }
    }

    fn get_idx_by_name(&self, name: &str) -> Option<&usize> {
        self.owner.get_idx_by_name(name)
    }

    // 获取所有的边，其中的节点以 index 的形式给出
    fn get_edges(&self) -> Vec<(usize, usize)> {
        let mut res = vec![];
        for (_, src_idx) in &self.owner.nodes_names {
            match &self.owner.nodes[*src_idx] {
                None => continue,
                Some(node) => {
                    for idx in &node.next_idx {
                        res.push((*src_idx, *idx));
                    }
                }
            }
        }
        res
    }

    // 打印所有的边
    fn print_edges(&self) {
        // 获取所有的边(返回值是节点的 index)
        let edges = self.get_edges();

        // TODO：这里遇到空节点的情况
        for (src_idx, dst_idx) in edges {
            let src_node = self.get_node_by_idx(src_idx);
            let dst_node = self.get_node_by_idx(dst_idx);
            match dst_node {
                None => {}
                Some(dst_node) => {
                    println!("{:?}->{:?}", src_node.unwrap().name, dst_node.name);
                }
            }
        }
    }

    // 删除节点
    fn del_node_by_idx(&mut self, idx: usize) -> bool { self.owner.del_node_by_idx(idx) }
    // 删除边
    fn del_edge_by_idx(&mut self, src_idx: usize, dst_idx: usize) -> bool { self.owner.del_edge_by_idx(src_idx, dst_idx) }

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
}


#[cfg(test)]
mod tests {
    use crate::history::graph::{ArenaList, Graph};


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
            graph.add_edge(
                src_name, src_name.to_string(),
                dst_name, dst_name.to_string());
        }

        // 获取所有的边(返回值是节点的 index)
        let edges = graph.get_edges();

        // 打印所有的边
        graph.print_edges();


        graph.del_edge_by_name("Michael", "Lucy");
        println!("======after del edge [Sophia]-> [Lucy]：======");
        graph.print_edges();

        graph.del_node_by_name("Sophia");
        println!("======after del node 【Sophia】：======");
        graph.print_edges();
    }
}