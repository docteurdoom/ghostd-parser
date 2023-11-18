# ghostd-parser
## RPC-based parser for [Ghost Coin](https://github.com/ghost-coin/ghost-core)

### Prerequisites:

1. Rust toolchain: https://rustup.rs/
2. Ghost Daemon: https://github.com/ghost-coin/ghost-core/releases/latest
3. SurrealDB: https://surrealdb.com/install
4. Git CLI

### Install and run SurrealDB:

	DB="/your/path/to/store/the/database"
	surreal start --log trace --user root --pass root file:${DB}

### Install Git and Rust toolchain, clone and compile the parser:
	
	git clone https://github.com/docteurdoom/ghostd-parser.git
	cd ghostd-parser
	mkdir -p ~/.ghost/ && mv -v ./configs/ghost.conf ~/.ghost/
	cargo install --force --verbose --path ${PWD}

### Download and extract the archive and run Ghost Daemon:

	VER=$(curl -s https://api.github.com/repos/ghost-coin/ghost-core/releases/latest | jq -r '.tag_name' | sed "s/v//")
	URL=$(curl -s https://api.github.com/repos/ghost-coin/ghost-core/releases/latest | jq -r '.assets[] | .browser_download_url' | grep "x86_64-pc-linux-gnu")
	wget ${URL}
	tar -xvf ghost-${VER}-x86_64-pc-linux-gnu.tar.gz
	cd ghost-${PV} && ./ghostd --daemon

### Run the parser:

	ghostd-parser --rpc-ip 127.0.0.1:51725 --rpc-user user --rpc-password password --stage example --surrealdb-ip 127.0.0.1:8000

### Run SurrealQL on the database:

	surreal start --log trace --user root --pass root file:/path/to/store/the/new/database
	surreal sql --conn http://localhost:8000 --user root --password root
	use ns example db example
	select * from blocks where coldstaking != none
	math::max(select count(tx) from blocks)

### More info about SurrealQL is in [SurrealDB Docs](https://surrealdb.com/docs/introduction/start).
