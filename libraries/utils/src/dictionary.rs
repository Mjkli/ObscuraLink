
extern crate rand;


use rand::Rng;
use std::fs::read_to_string;
use std::path::Path;

pub struct Dictionary {
    words: Vec<String>,
}

impl Dictionary {
    pub fn new(name_path: &Path) -> Dictionary {
        let mut dic = Dictionary {
            words: Vec::new(),
        };

        let mut names = Vec::new();

        for line in read_to_string(name_path).unwrap().lines(){
            names.push(line.to_string());
        }

        dic.update_words(names);
        dic
    }

    pub fn len(&self) -> usize{
        self.words.len()
    }

    pub fn random(&self) -> &String{
        // get random number between 0 and len of words and select word from dictionary
        let num = rand::thread_rng().gen_range(0..self.len());
        &self.words[num]
    }
    
   
    // add words to dictionary
    pub fn update_words(&mut self, mut additions: Vec<String>){
        self.words.append(&mut additions);
    }

    pub fn print(&mut self) {
        println!("Words: {:?}", self.words);
        println!("length: {}", self.len());
    }

}

