use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Block {
    index: u64,
    timestamp: u64,
    data: String,
    hash: String,
    prev_hash: String
}


impl Block {

   pub fn new(data: String, prev_hash: Option<String>) -> Self {

        let now = SystemTime::now();
        let since_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        let timestamp = since_epoch.as_secs() as u64;
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..=u64::MAX);
        let hash = format!("{:x}", rng.gen::<u128>());
        let prev_hash = match prev_hash {
            Some(prev_hash) => prev_hash,
            None => "NULL".to_string()

        };


        Self {
            index,
            timestamp, 
            data,
            hash,
            prev_hash,
        }



    }

    pub fn data_size(&self) -> usize {
            //should return the length of the data contained in the block
        self.data.len()
    }

    pub fn creation_time(&self) -> u64 {
        //should return its creation date
        self.timestamp
    }

}