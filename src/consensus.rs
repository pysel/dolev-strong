use crate::communication::peer::Peer;
use crate::communication::{Communication, self};
use crate::utils;
use self::genesis::strategy::GenesisStrategy;
use self::sync::{Synchrony, new_synchrony};

pub mod genesis;
pub mod sync;

pub struct ConsensusNode<'a> {
    pub communication: Communication,
    pub genesis_strategy: Option<&'a dyn GenesisStrategy>,
    self_is_leader: bool,
    round_leader: Option<Peer>,
    synchrony: Synchrony, // will be used for synchrony
}

impl<'a> ConsensusNode<'a> {
    pub fn new_consensus_node(config_index: i32, path_to_config_file: String, bootstrap_timestamp: u64) -> ConsensusNode<'a> {
        let keypair = utils::crypto::gen_keypair();
        let mut communication: Communication = communication::new_node(keypair, config_index, path_to_config_file);
        communication.setup(); // setup communications

        let (round_leader, self_is_leader) = match communication.get_round_leader() {
            Some(peer) => (Some(peer), false),
            None => (None, true)
        };

        let synchrony: Synchrony = new_synchrony(bootstrap_timestamp);

        println!("{:?}", synchrony);

        let mut consensus_node: ConsensusNode<'_> = ConsensusNode{
            communication, 
            genesis_strategy: None, 
            self_is_leader, 
            round_leader, 
            synchrony, 
        };
        consensus_node.setup_genesis_strategy(); // set genesis strategy for this node

        consensus_node
    }

    fn set_genesis_strategy(&mut self, strategy: &'a dyn GenesisStrategy) {
        self.genesis_strategy = Some(strategy);
    }

    fn rwait(&self, r: i64) {
        self.synchrony.rwait(r)
    }

    pub fn launch(&self) {
        if let Some(strategy) = self.genesis_strategy {
            strategy.genesis_round(self);
        } else {
            panic!("trying to launch a node without specifying it's genesis strategy")
        }

        
    }
}