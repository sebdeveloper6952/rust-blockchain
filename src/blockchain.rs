extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use self::sha2::{Sha256, Digest};
use std::fmt::Write;

#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32
}

#[derive(Debug, Serialize)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    difficulty: u32,
    prev_hash: String,
    merkle: String,
}

#[derive(Serialize, Debug)]
pub struct Block {
    header: Blockheader,
    count: u32,
    transactions:  Vec<Transaction>,
}

pub struct Blockchain {
    blocks: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}

impl Blockchain {
    pub fn new(miner_addr: String, difficulty: u32) -> Blockchain {
        let mut blockchain = Blockchain {
            blocks: Vec::new(),
            curr_trans: Vec::new(),
            difficulty,
            miner_addr,
            reward: 100.0,
        };

        // genesis block
        blockchain.generate_block();

        // return the new initialized blockchain
        return blockchain;
    }

    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool {
        self.curr_trans.push(Transaction{
            sender,
            receiver,
            amount,
        });

        return true;
    }

    pub fn last_hash(&self) -> String {
        let block = match self.blocks.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48, 64]).unwrap()
        };

        return Blockchain::hash(&block.header);
    }

    pub fn update_difficulty(&mut self, difficulty: u32) -> bool {
        self.difficulty = difficulty;

        return true;
    }

    pub fn update_reward(&mut self, reward: f32) -> bool {
        self.reward = reward;
        
        return true;
    }

    pub fn generate_block(&mut self) -> bool {
        let header = Blockheader {
            timestamp: time::now().to_timespec().sec,
            nonce: 0,
            difficulty: self.difficulty,
            prev_hash: self.last_hash(),
            merkle: String::new(),
        };

        let reward_trans = Transaction {
            sender: String::from("Root"),
            receiver: self.miner_addr.clone(),
            amount: self.reward,
        };

        let mut block = Block {
            header: header,
            count: 0,
            transactions: vec![],
        };

        block.transactions.push(reward_trans);
        block.transactions.append(&mut self.curr_trans);
        block.count = block.transactions.len() as u32;
        block.header.merkle = Blockchain::get_merkle(block.transactions.clone());
        Blockchain::proof_of_work(&mut block.header);

        println!("{:#?}", &block);
        self.blocks.push(block);
        
        return true;
    }

    fn get_merkle(curr_trans: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for t in &curr_trans {
            let hash = Blockchain::hash(t);
            merkle.push(hash);
        }

        if merkle.len() % 2 == 1 {
            let last = merkle.last().cloned().unwrap();
            merkle.push(last);
        }

        while merkle.len() > 1 {
            let mut h1 = merkle.remove(0);
            let mut h2 = merkle.remove(0);
            h1.push_str(&mut h2);
            let nh = Blockchain::hash(&h1);
            merkle.push(nh);
        }

        // the last hash is the merkle root
        return merkle.pop().unwrap();
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Blockchain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val != 0 {
                        header.nonce += 1;
                    } else {
                        println!("Block hash: {}", hash);
                        break;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        }
    }

    pub fn hash<T: serde::Serialize>(item: &T) -> String {
        let input = serde_json::to_string(&item).unwrap();
        let mut hasher = Sha256::default();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        let vec_res = result.to_vec();

        return Blockchain::hex_to_string(vec_res.as_slice());
    }

    pub fn hex_to_string(vec_res: &[u8]) -> String {
        let mut s = String::new();
        for byte in vec_res {
            write!(&mut s, "{:x}", byte).expect("unable to write");
        }

        return s;
    }
}

