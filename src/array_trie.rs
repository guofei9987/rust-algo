/*
数组前缀树
使用 Vec 来表示树
 */

pub struct ArrayTrie {
    next_idx: Vec<[usize;256]>,
    is_word:Vec<bool>
}

impl ArrayTrie {
    pub fn new() -> Self {
        let mut res = Self {
            next_idx: Vec::new(),
            is_word:Vec::new()
        };
        res.add_node();
        res
    }

    fn add_node(&mut self) -> usize {
        self.next_idx.push([0; 256]);
        self.is_word.push(false);
        self.is_word.len()-1
    }

    pub fn insert(&mut self, word: &str) {
        let mut curr_idx: usize = 0;
        for chr in word.bytes() {
            if self.next_idx[curr_idx][chr as usize] == 0 {
                self.next_idx[curr_idx][chr as usize] = self.add_node();
            }
            curr_idx = self.next_idx[curr_idx][chr as usize]
        }
        self.is_word[curr_idx]= true;
        return;
    }

    pub fn search(&self, text: &str) -> bool {
        let mut curr_idx = 0;
        for &chr in text.as_bytes() {
            let next_idx = self.next_idx[curr_idx][chr as usize];
            if next_idx == 0 { return false; }
            curr_idx = next_idx;
        }
        self.is_word[curr_idx]
    }
}