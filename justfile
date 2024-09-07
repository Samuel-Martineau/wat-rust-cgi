_default:
	@just --list

build:
	nix build

deploy: build
	scp result/bin/wat-rust-cgi mfcf:/home/s83marti/public_html/index.cgi

