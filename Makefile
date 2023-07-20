# runs a default instance of a node. for development purposes.
rund:
	cargo run leader 8000 1 ${shell pwd}/peers.txt
