
extern crate rand;


use rand::Rng;


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

    pub fn random(&self) -> &String{
        // get random number between 0 and len of words and select word from dictionary
        let num = rand::thread_rng().gen_range(0..self.length);
        &self.words[num]
    }
    
    
    pub fn update_words(&mut self, additions: Vec<String>){
        for w in additions {
            self.words.push(w);
            self.update_num();
        }
    }
    pub fn print(&mut self) {
        println!("Words: {:?}", self.words);
        println!("length: {}", self.length);
    }
    
    fn update_num(&mut self) {
        self.length = self.words.len();
    }

}

