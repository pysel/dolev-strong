CURTIME := $(shell date +%s) # current system time in UNIX

# launch launches a protocol with NODES amount of nodes, tolerating F Byzantine nodes
# note: set variables in all cases, use launch-default for pre-set values
# example: make NODES=10 F=8 launch 
launch:
	@rm -rf output.txt
	@./scripts/generate_config.sh ${NODES}
	./scripts/launch_with_X_nodes.sh ${NODES} ${F} ${CURTIME} &

# launch-default launches a protocol with 1 leader and 9 followers
# sets F to 8 (hence, 8 Byzantine nodes can be tolerated)
launch-default:
	@rm -rf output.txt
	@./scripts/generate_config.sh 10
	./scripts/launch_with_X_nodes.sh 10 8 ${CURTIME} 
	
test-unit:
	@clear
	@cargo test

