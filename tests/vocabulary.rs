extern crate kotonoha;

use kotonoha::vocabulary::Vocabulary;

#[test]
fn test_add() {
    let mut vocabulary = Vocabulary::new();
    assert_eq!(0, vocabulary.add("zero"));
    assert_eq!(0, vocabulary.add("zero"));
    assert_eq!(1, vocabulary.add("one"));
}

#[test]
fn test_find_id() {
    let mut vocabulary = Vocabulary::new();
    vocabulary.add("zero");
    vocabulary.add("one");

    assert_eq!(Some(0), vocabulary.find_id("zero"));
    assert_eq!(Some(1), vocabulary.find_id("one"));
    assert_eq!(None, vocabulary.find_id("two"));
}

#[test]
fn test_find_word() {
    let mut vocabulary = Vocabulary::new();
    vocabulary.add("zero");
    vocabulary.add("one");

    assert_eq!(Some("zero"), vocabulary.find_word(0));
    assert_eq!(Some("one"), vocabulary.find_word(1));
    assert_eq!(None, vocabulary.find_word(2));
}
