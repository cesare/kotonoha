use indexmap::IndexMap;

type VocabularyId = u32;

#[derive(Default)]
pub struct Vocabulary {
    words: IndexMap<String, VocabularyId>,
}

impl Vocabulary {
    pub fn new() -> Vocabulary {
        Vocabulary {
            words: IndexMap::new(),
        }
    }

    pub fn add(&mut self, word: &str) -> VocabularyId {
        self.find_id(word).unwrap_or_else(|| self.register(word))
    }

    fn register(&mut self, word: &str) -> VocabularyId {
        let next_id = self.words.len() as VocabularyId;
        self.words.insert(word.to_owned(), next_id);
        next_id
    }

    pub fn find_id(&self, word: &str) -> Option<VocabularyId> {
        self.words.get(word).cloned()
    }

    pub fn find_word(&self, id: VocabularyId) -> Option<&str> {
        self.words.get_index(id as usize).map(|(w, _)| w.as_ref())
    }
}
