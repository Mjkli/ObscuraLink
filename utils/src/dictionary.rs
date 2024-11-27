

pub struct Dictionary {
    words: Vec<String>,
    length: usize
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary {
            words: Vec::new(),
            length: 0
        }
    }

    pub fn random(&self) -> String{
        // get random number and select random word from dictionary
        "".to_string()
    }
    
    pub fn update_words(&mut self, additions: Vec<String>){
        for w in additions {
            self.words.push(w);
            self.update_num();
        }
    }

    pub fn update_num(&mut self) {
        self.length = self.words.len();
    }
    
    pub fn print(&mut self) {
        println!("Words: {:?}", self.words);
        println!("length: {}", self.length);
    }

}

