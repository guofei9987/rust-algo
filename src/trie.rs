/*
前缀树
1. 基于 byte，从而支持中文等各种语言的各种编码
2. 基本元素为 char，不需要用 T 来做泛型
3. 不考虑删除关键词，因此也不需要 prev 来管理孔洞
4. 不考虑多树合并、拆分，因此也不需要用 arena list 来管理
5. idx = 0 就是 跟节点
 */
use std::collections::HashMap;

struct Node {
    next_idx: HashMap<u8, usize>,
    is_word: bool,
}


pub struct Trie {
    nodes: Vec<Node>,
}

impl Trie {
    pub fn new() -> Self {
        let mut res = Self {
            nodes: Vec::new()
        };
        res.add_node();
        res
    }

    fn add_node(&mut self) -> usize {
        let len = self.nodes.len();
        self.nodes.push(Node { next_idx: HashMap::new(), is_word: false });
        len
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr_idx: usize = 0;
        for chr in word.as_bytes() {
            if !self.nodes[curr_idx].next_idx.contains_key(chr) {
                let new_idx = self.add_node();
                self.nodes[curr_idx].next_idx.insert(*chr, new_idx);
            }
            curr_idx = self.nodes[curr_idx].next_idx[chr]
        }
        self.nodes[curr_idx].is_word = true;
        return;
    }

    pub fn search(&self, text: &str) -> bool {
        let mut curr_idx = 0;
        let mut idx = 0;
        let text = text.as_bytes();
        while self.nodes[curr_idx].next_idx.len() > 0 && idx < text.len() {
            match self.nodes[curr_idx].next_idx.get(&text[idx]) {
                Some(next_idx) => curr_idx = *next_idx,
                None => return false
            }
            idx += 1;
        }

        if idx < text.len() {
            return false;
        }

        return self.nodes[curr_idx].is_word;
    }
}