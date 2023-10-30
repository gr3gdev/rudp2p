# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/97s-801ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-187ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-37ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-33ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-90ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-1ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/31s-293ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-84ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/60s-268ms-blue)

```
Unable to read errors
```
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/127s-884ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-187ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-22ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-42ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-90ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-2ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/31s-292ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/60s-167ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-185ms-blue)

```
Unable to read errors
```
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/67s-715ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-187ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-42ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-90ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-2ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-292ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/90s-352ms-blue)

```
Unable to read errors
```
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/37s-418ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-186ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-42ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-90ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-7ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-287ms-blue)

```
Unable to read errors
```
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/5s-319ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-186ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-35ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-33ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-90ms-blue)

```
Unable to read errors
```
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-30T14:25:28.863717Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:28.942143Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:29.576569Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.004371Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.582675Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.886231Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.052437Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.198408Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.446935Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.860876Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.112148Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.582419Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.632560Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.003905Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.346725Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.450465Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.702328Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.823897Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.961730Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:25:33.969226Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.057848Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.159319Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.163340Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.259686Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:36.355244Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:36.358237Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:27:06.438955Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:27:36.522538Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:27:36.622965Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:25:28.863717Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:28.942143Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:29.576569Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.004371Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.582675Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.886231Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.052437Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.198408Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.446935Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.860876Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.112148Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.582419Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.632560Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.003905Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.346725Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.450465Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.702328Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.823897Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.961730Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:25:33.969226Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.057848Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.159319Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.163340Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.259686Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:36.355244Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:36.358237Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:27:06.438955Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:27:36.522538Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:27:36.622965Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:25:28.863717Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:28.942143Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:29.576569Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.004371Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.582675Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.886231Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.052437Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.198408Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.446935Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.860876Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.112148Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.582419Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.632560Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.003905Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.346725Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.450465Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.702328Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.823897Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.961730Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:25:33.969226Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.057848Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.159319Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.163340Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.259686Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:36.355244Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:36.358237Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:27:06.438955Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:27:36.522538Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:27:36.622965Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:25:28.863717Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:28.942143Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:29.576569Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.004371Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.582675Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.886231Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.052437Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.198408Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.446935Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.860876Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.112148Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.582419Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.632560Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.003905Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.346725Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.450465Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.702328Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.823897Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.961730Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:25:33.969226Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.057848Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.159319Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.163340Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:26:06.259686Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:25:28.863717Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:28.942143Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:29.576569Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.004371Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.582675Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:30.886231Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.052437Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.198408Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.446935Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:31.860876Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.112148Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.582419Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:32.632560Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.003905Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.346725Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.450465Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.702328Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.823897Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:25:33.961730Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:25:33.969226Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126


```
</details>

