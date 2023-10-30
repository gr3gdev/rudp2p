# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/93s-284ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-686ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-24ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-49ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-199ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-78ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-189ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/60s-273ms-blue)

```
Unable to read errors
```
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/123s-472ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-686ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-24ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-40ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-199ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-2ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/31s-79ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/60s-378ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-84ms-blue)

```
Unable to read errors
```
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/63s-94ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-685ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-24ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-49ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-199ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/32s-75ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/90s-462ms-blue)

```
Unable to read errors
```
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/33s-2ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-684ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-24ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-41ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-198ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-78ms-blue)

```
Unable to read errors
```
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/0s-797ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-684ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-24ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-40ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-199ms-blue)

```
Unable to read errors
```
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-30T14:22:43.598089Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.598143Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.599275Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.777118Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.778316Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.778317Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779455Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779461Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779594Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.977255Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.981027Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.148363Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.149756Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.150682Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.278772Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.280147Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.280159Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.281530Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.416445Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:44.424441Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:44.516952Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:23:16.599575Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:23:46.690565Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:23:46.791150Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:16.881282Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:16.892841Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:16.981838Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:47.068959Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:43.598089Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.598143Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.599275Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.777118Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.778316Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.778317Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779455Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779461Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779594Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.977255Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.981027Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.148363Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.149756Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.150682Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.278772Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.280147Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.280159Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.281530Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.416445Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:44.424441Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:44.516952Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:23:16.599575Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:23:46.690565Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:23:46.791150Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:16.881282Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:16.892841Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:16.981838Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:47.068959Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:43.598089Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.598143Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.599275Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.777118Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.778316Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.778317Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779455Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779461Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779594Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.977255Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.981027Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.148363Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.149756Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.150682Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.278772Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.280147Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.280159Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.281530Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.416445Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:44.424441Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:44.516952Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:23:16.599575Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:23:46.690565Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:23:46.791150Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:16.881282Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:16.892841Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:16.981838Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:24:47.068959Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:43.598089Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.598143Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.599275Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.777118Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.778316Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.778317Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779455Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779461Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779594Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.977255Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.981027Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.148363Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.149756Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.150682Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.278772Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.280147Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.280159Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.281530Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.416445Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:44.424441Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:44.516952Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:23:16.599575Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:43.598089Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.598143Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.599275Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.777118Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.778316Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.778317Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779455Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779461Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.779594Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.977255Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:43.981027Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.148363Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.149756Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.150682Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.278772Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.280147Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.280159Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.281530Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:22:44.416445Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:44.424441Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:22:44.516952Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126


```
</details>

