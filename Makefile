CURTIME := $(shell date +%s) # current system time in UNIX

# runs a default instance of a node. for development purposes.
run-default:
	@echo 
	@echo Launching default node...
	@echo 
	@cargo run $(CURDIR)/config.txt 0 ${CURTIME}

run-nd:
	@echo
	@echo Launching peer...
	@echo
	@cargo run $(CURDIR)/config.txt 1 ${CURTIME}  > ./peer-log.txt

run-mult: run-default run-nd

test-unit:
	@clear
	@cargo test

build:
	@docker-compose build --quiet

up:
	@clear
	@echo
	@echo Starting up...
	@echo
	@BOOTSTRAPPING_TIME=${CURTIME} docker-compose up -d
	@./scripts/start-services.sh 

ckillall:
	-@docker kill $(shell docker ps -q)

cbu: ckillall build up