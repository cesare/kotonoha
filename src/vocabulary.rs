use indexmap::IndexMap;

pub struct Vocabulary {
    words: IndexMap<String, u32>,
}

impl Vocabulary {
    pub fn new() -> Vocabulary {
        Vocabulary {
            words: IndexMap::new(),
        }
    }

    pub fn add(&mut self, word: &str) -> u32 {
        if !self.words.contains_key(word) {
            let next_id = self.words.len();
            self.words.insert(word.to_owned(), next_id as u32);
        }
        *self.words.get(word).unwrap()
    }

    pub fn find_id(&self, word: &str) -> Option<u32> {
        self.words.get(word).map(|&n| n)
    }

    pub fn find_word(&self, id: u32) -> Option<&str> {
        self.words.get_index(id as usize).map(|(w, _)| w.as_ref())
    }
}

#[cfg(test)]
mod tests {
    use vocabulary::Vocabulary;

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
}
