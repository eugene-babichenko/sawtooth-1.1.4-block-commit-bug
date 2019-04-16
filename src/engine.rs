use sawtooth_sdk::consensus::{engine::*, service::Service};
use std::sync::mpsc::Receiver;

#[derive(PartialEq)]
pub enum BugType {
    SimultaneousCommits,
    CommitAfterCommit,
}

pub struct ConsensusEngine {
    bug_type: BugType,
}

impl ConsensusEngine {
    pub fn new(bug_type: BugType) -> Self {
        Self { bug_type }
    }
}

impl Engine for ConsensusEngine {
    fn version(&self) -> String {
        "0.1".into()
    }

    fn name(&self) -> String {
        "consensus".into()
    }

    fn start(
        &mut self,
        updates: Receiver<Update>,
        service: Box<Service>,
        _startup_state: StartupState,
    ) -> Result<(), Error> {
        let mut service = service;

        service.initialize_block(None);
        loop {
            service.summarize_block();
            if let Ok(_) = service.finalize_block(vec![]) {
                println!("Block is ready");
                break;
            }
        }

        for update in updates.iter() {
            match update {
                Update::BlockNew(block) => {
                    println!("Block was received by the engine");
                    service.check_blocks(vec![block.block_id]);
                }
                Update::BlockValid(block_id) => {
                    println!("Block passed validation");
                    service.commit_block(block_id.clone());
                    // Reproduce the case when there is  two calls to
                    // commit_block and the first commit is not complete.
                    if self.bug_type == BugType::SimultaneousCommits {
                        println!("Trying to do SimultaneousCommits");
                        service.commit_block(block_id);
                    }
                }
                Update::BlockCommit(block_id) => {
                    println!("Block committed");
                    // Reproduce the case when there is a call to commit a block
                    // that is already committed.
                    if self.bug_type == BugType::CommitAfterCommit {
                        println!("Trying to do CommitAfterCommit");
                        service.commit_block(block_id);
                    }

                    // Prepare the next block
                    if let Err(_) = service.initialize_block(None) {
                        continue;
                    }
                    loop {
                        service.summarize_block();
                        if let Ok(_) = service.finalize_block(vec![]) {
                            println!("Block is ready");
                            break;
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}
