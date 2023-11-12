
use rust_algo::trie::Trie;


#[test]
fn func1() {
    let mut trie = Trie::new();

    trie.insert("hello");
    trie.insert("hell");
    trie.insert("你好");

    assert_eq!(trie.search("hello"),true);
    assert_eq!(trie.search("hello1"),false);
    assert_eq!(trie.search("hel1o"),false);
    assert_eq!(trie.search("你好"),true);

}