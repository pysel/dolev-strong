# runs a default instance of a node. for development purposes.
rund:
	@clear
	@echo 
	@echo Launching default node...
	@echo
	@cargo run $(CURDIR)/config.txt 0

