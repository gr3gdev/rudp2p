# RUDP2P
[![Rust tests](https://github.com/gr3gdev/rudp2p/actions/workflows/rust.yml/badge.svg)](https://github.com/gr3gdev/rudp2p/actions/workflows/rust.yml)
[![Rust Docs](https://img.shields.io/badge/docs.rs-rudp2p-green)](https://docs.rs/rudp2p/)

RUDP2P is a Rust library for P2P exchange with the UDP protocol.

## Features

- [X] Dispatch connections
    - [X] Reception of connection and disconnection events

- [X] Block peers
    - [X] A block peer does not receive any messages until he has unblock
    - [X] A block peer can not send any messages until he has unblock

- [X] Exchange messages
    - [X] A peer sends a text to all peers
    - [X] A peer sends a file to a peer

