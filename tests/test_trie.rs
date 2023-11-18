use rust_algo::trie::Trie;
use rust_algo::array_trie::ArrayTrie;


#[test]
fn func1() {
    let mut trie = Trie::new();

    trie.insert("hello");
    trie.insert("hell");
    trie.insert("你好");

    assert_eq!(trie.search("hello"), true);
    assert_eq!(trie.search("hello1"), false);
    assert_eq!(trie.search("hel1o"), false);
    assert_eq!(trie.search("hell"), true);
    assert_eq!(trie.search("hel"), false);
    assert_eq!(trie.search("你好"), true);
    assert_eq!(trie.search("aaa"), false);
    assert_eq!(trie.search("a"), false);
    assert_eq!(trie.search(""), false);
}



#[test]
fn func2() {
    let mut array_trie = ArrayTrie::new();

    array_trie.insert("hello");
    array_trie.insert("hell");
    array_trie.insert("你好");

    assert_eq!(array_trie.search("hello"), true);
    assert_eq!(array_trie.search("hello1"), false);
    assert_eq!(array_trie.search("hel1o"), false);
    assert_eq!(array_trie.search("hell"), true);
    assert_eq!(array_trie.search("hel"), false);
    assert_eq!(array_trie.search("你好"), true);
    assert_eq!(array_trie.search("aaa"), false);
    assert_eq!(array_trie.search("a"), false);
    assert_eq!(array_trie.search(""), false);
}
