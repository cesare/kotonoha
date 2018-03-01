use indexmap::IndexMap;

#[derive(Default)]
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
        self.find_id(word).unwrap_or_else(|| self.register(word))
    }

    fn register(&mut self, word: &str) -> u32 {
        let next_id = self.words.len() as u32;
        self.words.insert(word.to_owned(), next_id);
        next_id
    }

    pub fn find_id(&self, word: &str) -> Option<u32> {
        self.words.get(word).cloned()
    }

    pub fn find_word(&self, id: u32) -> Option<&str> {
        self.words.get_index(id as usize).map(|(w, _)| w.as_ref())
    }
}
