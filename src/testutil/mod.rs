use crate::consensus::ConsensusNode;


pub fn run_node(config_index: i32, path_to_config_file: String, bootstrap_timestamp: u64) {
    let consensus_node: ConsensusNode<'_> = ConsensusNode::new_consensus_node(config_index, path_to_config_file, bootstrap_timestamp);
    consensus_node.launch();
}