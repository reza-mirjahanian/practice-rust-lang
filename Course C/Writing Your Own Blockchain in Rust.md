
## Writing Your Own Simple Blockchain in Rust



### Project Setup

1. **Creating the Project**
    - Open a terminal and run:
      ```sh
      cargo new reza_chain
      ```
    - This creates a new Rust project template named `hip_zip_chain`.

2. **Updating Dependencies**
    - Open `Cargo.toml` and add the following dependencies:
      ```toml
      [dependencies]
      sha2 = "0.9"
      libp2p = "0.35"
      tokio = "1.8"
      hex = "0.4"
      log = "0.4"
      ```

### Defining the Blockchain Structure

1. **Creating the Main Rust File**
    - Navigate to `src/main.rs` and remove the default main function.
    - Define the blockchain structure:
      ```rust
      use serde::{Serialize, Deserialize};
      
      #[derive(Serialize, Deserialize, Debug)]
      pub struct Block {
          id: u64,
          hash: String,
          previous_hash: String,
          timestamp: i64,
          data: String,
          nonce: u64,
      }

      #[derive(Serialize, Deserialize, Debug)]
      pub struct Blockchain {
          blocks: Vec<Block>,
      }
      ```

2. **Implementing the Blockchain**
    - Add methods to `Blockchain`:
      ```rust
      impl Blockchain {
          pub fn new() -> Self {
              let mut blockchain = Blockchain { blocks: vec![] };
              blockchain.add_block("Genesis Block".to_string());
              blockchain
          }

          pub fn add_block(&mut self, data: String) {
              let previous_hash = match self.blocks.last() {
                  Some(block) => &block.hash,
                  None => "0".to_string(),
              };
              let block = Block {
                  id: self.blocks.len() as u64,
                  hash: Blockchain::hash(data.clone(), previous_hash.clone()),
                  previous_hash,
                  timestamp: Blockchain::current_timestamp(),
                  data,
                  nonce: 0,
              };
              self.blocks.push(block);
          }

          fn hash(data: String, previous_hash: String) -> String {
              // Implement hashing logic using sha2 crate
              format!("{:x}", sha2::Sha256::digest(format!("{}{}", data, previous_hash)))
          }

          fn current_timestamp() -> i64 {
              // Get the current timestamp
              chrono::Utc::now().timestamp()
          }
      }
      ```

### Adding Consensus Mechanism

1. **Implementing Chain Validation**
    - Add validation methods:
      ```rust
      impl Blockchain {
          // ... existing methods

          pub fn is_chain_valid(&self) -> bool {
              for i in 1..self.blocks.len() {
                  let current_block = &self.blocks[i];
                  let previous_block = &self.blocks[i - 1];

                  if current_block.previous_hash != previous_block.hash {
                      return false;
                  }

                  if current_block.hash != Blockchain::hash(current_block.data.clone(), current_block.previous_hash.clone()) {
                      return false;
                  }
              }
              true
          }
      }
      ```

2. **Adding a Mining Function**
    - Implement mining logic:
      ```rust
      impl Blockchain {
          // ... existing methods

          pub fn mine_block(&mut self, data: String) {
              let mut nonce = 0;
              loop {
                  let hash = Blockchain::hash(format!("{}{}", data, nonce), self.blocks.last().unwrap().hash.clone());
                  if hash.starts_with("0000") {
                      let new_block = Block {
                          id: self.blocks.len() as u64,
                          hash,
                          previous_hash: self.blocks.last().unwrap().hash.clone(),
                          timestamp: Blockchain::current_timestamp(),
                          data,
                          nonce,
                      };
                      self.blocks.push(new_block);
                      break;
                  }
                  nonce += 1;
              }
          }
      }
      ```

### Testing the Blockchain

1. **Creating the Blockchain and Mining Blocks**
    - In the `main` function:
      ```rust
      fn main() {
          let mut blockchain = Blockchain::new();

          blockchain.mine_block("Block 1 Data".to_string());
          blockchain.mine_block("Block 2 Data".to_string());

          for block in blockchain.blocks {
              println!("{:?}", block);
          }
      }
      ```

2. **Running the Code**
    - Compile and run your Rust project:
      ```sh
      cargo run
      ```

