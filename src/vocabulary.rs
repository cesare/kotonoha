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
