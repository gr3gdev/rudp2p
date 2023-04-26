#!/bin/bash
echo "##### Unit TESTS #####"
cargo test --color=always --package rudp2p --lib server::tests --no-fail-fast
cargo test --color=always --package rudp2p --lib peer::event::tests --no-fail-fast

echo "##### Integration TESTS #####"
cargo test --color=always --package rudp2p --test peer
