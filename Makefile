CURTIME := $(shell date +%s) # current system time in UNIX

# launch launches a protocol with NODES amount of nodes, tolerating F Byzantine nodes
# note: set variables in all cases, use launch-default for pre-set values
# example: make NODES=10 F=8 launch 
launch:
	@rm -rf output.txt
	@./scripts/generate_config.sh ${NODES}
	./scripts/launch_with_X_nodes.sh ${NODES} ${F} ${CURTIME} 

# launch-default launches a protocol with 1 leader and 9 followers
# sets F to 8 (hence, 8 Byzantine nodes can be tolerated)
launch-default:
	@rm -rf output.txt
	@./scripts/generate_config.sh 10
	./scripts/launch_with_X_nodes.sh 10 8 ${CURTIME} 

# launch-null-proposal works the same as launch, but sets the leader strategy to null proposal
# In null_proposal strategy, a leader does not propose any value
# In this scenario, all honest followers should output default NULL value
launch-null-proposal:
	@rm -rf output.txt
	@./scripts/generate_config.sh ${NODES}
	@./scripts/set_leader_strategy.sh leader_null_proposal
	./scripts/launch_with_X_nodes.sh ${NODES} ${F} ${CURTIME} 

# launch-null-proposal-default launches a protocol with null_proposal byzantine leader and 9 followers
launch-null-proposal-default:
	@rm -rf output.txt
	@./scripts/generate_config.sh 10
	@./scripts/set_leader_strategy.sh leader_null_proposal
	./scripts/launch_with_X_nodes.sh 10 8 ${CURTIME}

# generate-config generates a config file with NODES amount of nodes
# example: make NODES=10 generate-config
generate-config:
	@./scripts/generate_config.sh ${NODES}

# set-leader-strategy sets the leader strategy to STRATEGY
# example: make STRATEGY=leader_null_proposal set-leader-strategy
# available strategies: leader (aka honest), leader_null_proposal
set-leader-strategy:
	@./scripts/set_leader_strategy.sh ${STRATEGY}

# launch-custom-config launches a protocol with a custom config file
# example: make NODES=10 F=8 launch-custom-config
# Prior to running this target:
#   * Generate a custom config with `make generate-config` 
#   * Set a desired leader strategy with `make set-leader-strategy`
# NOTE: not working yet
launch-custom-config:
	@rm -rf output.txt
	./scripts/launch_with_X_nodes.sh ${NODES} ${F} ${CURTIME}

test-unit:
	@clear
	@cargo test

build:
	@clear
	@cargo build

# Generates the types
protogen:
	@clear
	@cd proto; buf generate
