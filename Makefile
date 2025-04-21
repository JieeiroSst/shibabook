build:
	anchor build

deploy:
	anchor deploy

test:
	anchor test

server:
	solana-test-validator

skip:
	anchor test --skip-local-validator