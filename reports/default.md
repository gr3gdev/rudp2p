# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/149s-360ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/26s-625ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-37ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-66ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-74ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-33ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-97ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-9ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-120ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-31ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-112ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-70ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-74ms-blue)

```
Unable to read errors
```
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/119s-264ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/26s-589ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-48ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-65ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-62ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-20ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-33ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/31s-98ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/31s-121ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-28ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-112ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-70ms-blue)

```
Unable to read errors
```
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/89s-142ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/26s-584ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-26ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-63ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-73ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-28ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-30ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-95ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-7ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-120ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/60s-180ms-blue)

```
Unable to read errors
```
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/58s-988ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/26s-559ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-39ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-63ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-73ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-20ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-34ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/31s-97ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-19ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-106ms-blue)

```
Unable to read errors
```
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/26s-866ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/26s-563ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-29ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-38ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-64ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-62ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-24ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-30ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/32s-98ms-blue)

```
Unable to read errors
```
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-31T08:58:46.166341Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.180496Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.192628Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.210154Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.203049Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.211762Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213294Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213976Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213976Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.214503Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.214549Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.215105Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.216015Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.216770Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.217579Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.218215Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.218806Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.219359Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.219544Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.611208Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.611210Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.612927Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.614235Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.614987Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615245Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615698Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615590Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.616478Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.618123Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619018Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619570Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619710Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.620023Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.620573Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.621152Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.413892Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.414159Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.416170Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.417832Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.417659Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.418844Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.419411Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.419854Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.420735Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.421390Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.421782Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.422336Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.422881Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423532Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423726Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423850Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.017263Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.017527Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.020631Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.021188Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.021848Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.022460Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.023721Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.023305Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.024575Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.025667Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.026363Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.028914Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.027071Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029539Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029657Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029739Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.218361Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.218365Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.221625Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.221743Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.222396Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.222965Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.224440Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.224808Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.225002Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.226292Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.226919Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230058Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230121Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230426Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230679Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230916Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.619593Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.619593Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622614Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622672Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622953Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.623703Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.625509Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.625978Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.626055Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.627506Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.627547Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.632613Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634019Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634019Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634531Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634589Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:59:11.717000Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.724454Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.731941Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.737749Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.746637Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.747065Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.058524Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.061643Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.388208Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.405080Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.423977Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.682446Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.688148Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.689338Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.708039Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:13.013749Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:13.017591Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:13.022942Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:45.118756Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:45.120510Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:15.267474Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:15.269520Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:15.270444Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:45.372299Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:01:15.444105Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:01:15.452203Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:58:46.166341Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.180496Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.192628Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.210154Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.203049Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.211762Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213294Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213976Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213976Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.214503Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.214549Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.215105Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.216015Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.216770Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.217579Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.218215Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.218806Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.219359Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.219544Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.611208Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.611210Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.612927Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.614235Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.614987Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615245Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615698Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615590Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.616478Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.618123Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619018Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619570Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619710Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.620023Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.620573Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.621152Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.413892Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.414159Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.416170Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.417832Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.417659Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.418844Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.419411Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.419854Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.420735Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.421390Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.421782Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.422336Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.422881Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423532Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423726Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423850Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.017263Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.017527Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.020631Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.021188Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.021848Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.022460Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.023721Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.023305Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.024575Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.025667Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.026363Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.028914Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.027071Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029539Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029657Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029739Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.218361Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.218365Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.221625Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.221743Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.222396Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.222965Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.224440Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.224808Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.225002Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.226292Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.226919Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230058Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230121Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230426Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230679Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230916Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.619593Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.619593Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622614Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622672Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622953Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.623703Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.625509Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.625978Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.626055Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.627506Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.627547Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.632613Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634019Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634019Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634531Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634589Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:59:11.717000Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.724454Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.731941Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.737749Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.746637Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.747065Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.058524Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.061643Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.388208Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.405080Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.423977Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.682446Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.688148Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.689338Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.708039Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:13.013749Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:13.017591Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:13.022942Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:45.118756Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:45.120510Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:15.267474Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:15.269520Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:15.270444Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:45.372299Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:01:15.444105Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:01:15.452203Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:58:46.166341Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.180496Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.192628Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.210154Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.203049Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.211762Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213294Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213976Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213976Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.214503Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.214549Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.215105Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.216015Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.216770Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.217579Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.218215Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.218806Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.219359Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.219544Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.611208Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.611210Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.612927Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.614235Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.614987Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615245Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615698Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615590Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.616478Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.618123Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619018Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619570Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619710Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.620023Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.620573Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.621152Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.413892Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.414159Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.416170Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.417832Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.417659Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.418844Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.419411Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.419854Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.420735Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.421390Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.421782Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.422336Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.422881Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423532Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423726Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423850Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.017263Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.017527Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.020631Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.021188Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.021848Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.022460Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.023721Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.023305Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.024575Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.025667Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.026363Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.028914Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.027071Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029539Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029657Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029739Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.218361Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.218365Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.221625Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.221743Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.222396Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.222965Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.224440Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.224808Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.225002Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.226292Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.226919Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230058Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230121Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230426Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230679Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230916Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.619593Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.619593Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622614Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622672Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622953Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.623703Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.625509Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.625978Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.626055Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.627506Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.627547Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.632613Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634019Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634019Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634531Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634589Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:59:11.717000Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.724454Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.731941Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.737749Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.746637Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.747065Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.058524Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.061643Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.388208Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.405080Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.423977Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.682446Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.688148Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.689338Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.708039Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:13.013749Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:13.017591Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:13.022942Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:45.118756Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:45.120510Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:15.267474Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:15.269520Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:15.270444Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-31T09:00:45.372299Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:58:46.166341Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.180496Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.192628Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.210154Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.203049Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.211762Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213294Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213976Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213976Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.214503Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.214549Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.215105Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.216015Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.216770Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.217579Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.218215Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.218806Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.219359Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.219544Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.611208Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.611210Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.612927Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.614235Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.614987Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615245Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615698Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615590Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.616478Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.618123Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619018Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619570Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619710Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.620023Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.620573Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.621152Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.413892Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.414159Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.416170Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.417832Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.417659Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.418844Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.419411Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.419854Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.420735Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.421390Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.421782Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.422336Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.422881Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423532Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423726Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423850Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.017263Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.017527Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.020631Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.021188Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.021848Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.022460Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.023721Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.023305Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.024575Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.025667Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.026363Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.028914Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.027071Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029539Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029657Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029739Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.218361Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.218365Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.221625Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.221743Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.222396Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.222965Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.224440Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.224808Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.225002Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.226292Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.226919Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230058Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230121Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230426Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230679Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230916Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.619593Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.619593Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622614Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622672Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622953Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.623703Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.625509Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.625978Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.626055Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.627506Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.627547Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.632613Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634019Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634019Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634531Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634589Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:59:11.717000Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.724454Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.731941Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.737749Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.746637Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.747065Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.058524Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.061643Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.388208Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.405080Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.423977Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.682446Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.688148Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.689338Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.708039Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:13.013749Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:13.017591Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:13.022942Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:45.118756Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:45.120510Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:58:46.166341Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.180496Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.192628Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:58:46.210154Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.203049Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.211762Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213294Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213976Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.213976Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.214503Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.214549Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.215105Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.216015Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.216770Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.217579Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.218215Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.218806Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.219359Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.219544Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.611208Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.611210Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.612927Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.614235Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.614987Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615245Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615698Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.615590Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.616478Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.618123Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619018Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619570Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.619710Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.620023Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.620573Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:46.621152Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.413892Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.414159Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.416170Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.417832Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.417659Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.418844Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.419411Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.419854Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.420735Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.421390Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.421782Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.422336Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.422881Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423532Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423726Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:47.423850Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.017263Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.017527Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.020631Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.021188Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.021848Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.022460Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.023721Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.023305Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.024575Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.025667Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.026363Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.028914Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.027071Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029539Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029657Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:49.029739Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.218361Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.218365Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.221625Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.221743Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.222396Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.222965Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.224440Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.224808Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.225002Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.226292Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.226919Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230058Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230121Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230426Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230679Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:52.230916Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.619593Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.619593Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622614Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622672Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.622953Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.623703Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.625509Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.625978Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.626055Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.627506Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.627547Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.632613Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634019Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634019Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634531Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:58:58.634589Z ERROR r2d2: IoError { server disconnected }
    at /home/gregory.tardivel/.cargo/registry/src/index.crates.io-6f17d22bba15001f/r2d2-0.8.10/src/lib.rs:121

  2023-10-31T08:59:11.717000Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.724454Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.731941Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.737749Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.746637Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:11.747065Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.058524Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.061643Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.388208Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.405080Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.423977Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.682446Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.688148Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.689338Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:12.708039Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-31T08:59:13.013749Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:13.017591Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:13.022942Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:45.118756Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-31T08:59:45.120510Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126


```
</details>

