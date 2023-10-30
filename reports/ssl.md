# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/128s-806ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-124ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-30ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-25ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-17ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-13ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-111ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-7ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-203ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-92ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-94ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-295ms-blue)

```
Unable to read errors
```
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/98s-710ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-115ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-23ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-37ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-28ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-26ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-17ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/31s-97ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-8ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-201ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-92ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/60s-388ms-blue)

```
Unable to read errors
```
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/68s-616ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-121ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-17ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-17ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-25ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-111ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-206ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/90s-480ms-blue)

```
Unable to read errors
```
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/38s-404ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-117ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-22ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-34ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-26ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-25ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-17ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/31s-98ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-11ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-199ms-blue)

```
Unable to read errors
```
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-296ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-113ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-25ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-38ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-30ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
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
  2023-10-30T14:52:31.121125Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.346929Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.521762Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.735182Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.014808Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.392429Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.608297Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.865219Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.232292Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.561591Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.920278Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:34.111558Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:34.576292Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:35.080408Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:35.275588Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.200294Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.680517Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.801524Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.978889Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.091561Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.093111Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.193738Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:39.297856Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:09.389599Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:09.390228Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:09.390715Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:39.483538Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:39.583682Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:39.684057Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:52:31.121125Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.346929Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.521762Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.735182Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.014808Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.392429Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.608297Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.865219Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.232292Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.561591Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.920278Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:34.111558Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:34.576292Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:35.080408Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:35.275588Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.200294Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.680517Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.801524Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.978889Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.091561Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.093111Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.193738Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:39.297856Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:09.389599Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:09.390228Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:09.390715Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:39.483538Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:39.583682Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:39.684057Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:52:31.121125Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.346929Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.521762Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.735182Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.014808Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.392429Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.608297Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.865219Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.232292Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.561591Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.920278Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:34.111558Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:34.576292Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:35.080408Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:35.275588Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.200294Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.680517Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.801524Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.978889Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.091561Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.093111Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.193738Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:39.297856Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:09.389599Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:09.390228Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:09.390715Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:39.483538Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:39.583682Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:54:39.684057Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:52:31.121125Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.346929Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.521762Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.735182Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.014808Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.392429Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.608297Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.865219Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.232292Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.561591Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.920278Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:34.111558Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:34.576292Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:35.080408Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:35.275588Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.200294Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.680517Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.801524Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.978889Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.091561Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.093111Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.193738Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:52:31.121125Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.346929Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.521762Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:31.735182Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.014808Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.392429Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.608297Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:32.865219Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.232292Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.561591Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:33.920278Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:34.111558Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:34.576292Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:35.080408Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:35.275588Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.200294Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.680517Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.801524Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:52:36.978889Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.091561Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.093111Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:53:09.193738Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126


```
</details>

