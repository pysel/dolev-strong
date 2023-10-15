CURTIME := $(shell date +%s) # current system time in UNIX

launch:
	@rm -rf output.txt
	@./scripts/generate_config.sh ${NODES}
	./scripts/launch_with_X_nodes.sh ${NODES} ${CURTIME} &

launch-10:
	@rm -rf output.txt
	@./scripts/generate_config.sh 10
	./scripts/launch_with_X_nodes.sh 10 ${CURTIME}
	
test-unit:
	@clear
	@cargo test

