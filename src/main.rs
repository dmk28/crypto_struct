mod crypto;

fn main() {
    let new_block = crypto::Block::new("Zephyrus G14".to_string(), Some("prev_hash".to_string()));

    {
        let data_size = &new_block.data_size();
        println!("The size of the block is {} bytes", data_size);
    }

    {
        let creation_time = new_block.creation_time();
        println!("Hash was created at {}", creation_time);
    }
    

    
}
