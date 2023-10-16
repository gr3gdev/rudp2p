# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/128s-124ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-292ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-22ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-20ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/32s-97ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-97ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/60s-473ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-191ms-blue)

```
Unable to read errors
```
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/67s-646ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-291ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-23ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/31s-96ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-7ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-97ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/60s-474ms-blue)

```
Unable to read errors
```
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/97s-930ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-290ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-24ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-23ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-95ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-4ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-102ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-287ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-189ms-blue)

```
Unable to read errors
```
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/37s-531ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-289ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-13ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-23ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-13ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/32s-96ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-95ms-blue)

```
Unable to read errors
```
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/5s-424ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-287ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-35ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-20ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/32s-97ms-blue)

```
Unable to read errors
```
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-16T12:14:31.076089Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:92

  2023-10-16T12:14:31.300957Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:92

  2023-10-16T12:14:31.488643Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:92

  2023-10-16T12:14:31.664596Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:92

  2023-10-16T12:14:31.830495Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:92

  2023-10-16T12:14:32.089382Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:92

  2023-10-16T12:14:32.176492Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:92

  2023-10-16T12:14:32.294607Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:92

  2023-10-16T12:14:32.607753Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:92

  2023-10-16T12:14:33.078736Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:92

  2023-10-16T12:14:33.473182Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:92

  2023-10-16T12:14:34.141181Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:92

  2023-10-16T12:14:34.419763Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:92

  2023-10-16T12:14:34.566015Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:92

  2023-10-16T12:14:35.005523Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:92

  2023-10-16T12:14:35.193612Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:92

  2023-10-16T12:14:35.943124Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:92

  2023-10-16T12:14:36.096004Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:92

  2023-10-16T12:14:36.236385Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:125

  2023-10-16T12:15:08.342914Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:125

  2023-10-16T12:15:08.343549Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:125

  2023-10-16T12:15:38.454127Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:125

  2023-10-16T12:15:38.554469Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:125

  2023-10-16T12:15:38.654771Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:125

  2023-10-16T12:16:08.739308Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:125

  2023-10-16T12:16:08.839508Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:125

  2023-10-16T12:16:38.929562Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:125

  2023-10-16T12:16:39.029906Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:125

  2023-10-16T12:14:31.076089Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:92

  2023-10-16T12:14:31.300957Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:92

  2023-10-16T12:14:31.488643Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:92

  2023-10-16T12:14:31.664596Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:92

  2023-10-16T12:14:31.830495Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:92

  2023-10-16T12:14:32.089382Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:92

  2023-10-16T12:14:32.176492Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:92

  2023-10-16T12:14:32.294607Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:92

  2023-10-16T12:14:32.607753Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:92

  2023-10-16T12:14:33.078736Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:92

  2023-10-16T12:14:33.473182Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:92

  2023-10-16T12:14:34.141181Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:92

  2023-10-16T12:14:34.419763Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:92

  2023-10-16T12:14:34.566015Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:92

  2023-10-16T12:14:35.005523Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:92

  2023-10-16T12:14:35.193612Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:92

  2023-10-16T12:14:35.943124Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:92

  2023-10-16T12:14:36.096004Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:92

  2023-10-16T12:14:36.236385Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:125

  2023-10-16T12:15:08.342914Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:125

  2023-10-16T12:15:08.343549Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:125

  2023-10-16T12:15:38.454127Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:125

  2023-10-16T12:15:38.554469Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:125

  2023-10-16T12:15:38.654771Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:125

  2023-10-16T12:16:08.739308Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:125

  2023-10-16T12:16:08.839508Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:125

  2023-10-16T12:16:38.929562Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:125

  2023-10-16T12:16:39.029906Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:125

  2023-10-16T12:14:31.076089Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:92

  2023-10-16T12:14:31.300957Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:92

  2023-10-16T12:14:31.488643Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:92

  2023-10-16T12:14:31.664596Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:92

  2023-10-16T12:14:31.830495Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:92

  2023-10-16T12:14:32.089382Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:92

  2023-10-16T12:14:32.176492Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:92

  2023-10-16T12:14:32.294607Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:92

  2023-10-16T12:14:32.607753Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:92

  2023-10-16T12:14:33.078736Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:92

  2023-10-16T12:14:33.473182Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:92

  2023-10-16T12:14:34.141181Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:92

  2023-10-16T12:14:34.419763Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:92

  2023-10-16T12:14:34.566015Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:92

  2023-10-16T12:14:35.005523Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:92

  2023-10-16T12:14:35.193612Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:92

  2023-10-16T12:14:35.943124Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:92

  2023-10-16T12:14:36.096004Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:92

  2023-10-16T12:14:36.236385Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:125

  2023-10-16T12:15:08.342914Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:125

  2023-10-16T12:15:08.343549Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:125

  2023-10-16T12:15:38.454127Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:125

  2023-10-16T12:15:38.554469Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:125

  2023-10-16T12:15:38.654771Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:125

  2023-10-16T12:16:08.739308Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:125

  2023-10-16T12:16:08.839508Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:125

  2023-10-16T12:16:38.929562Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:125

  2023-10-16T12:16:39.029906Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:125

  2023-10-16T12:14:31.076089Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:92

  2023-10-16T12:14:31.300957Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:92

  2023-10-16T12:14:31.488643Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:92

  2023-10-16T12:14:31.664596Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:92

  2023-10-16T12:14:31.830495Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:92

  2023-10-16T12:14:32.089382Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:92

  2023-10-16T12:14:32.176492Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:92

  2023-10-16T12:14:32.294607Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:92

  2023-10-16T12:14:32.607753Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:92

  2023-10-16T12:14:33.078736Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:92

  2023-10-16T12:14:33.473182Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:92

  2023-10-16T12:14:34.141181Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:92

  2023-10-16T12:14:34.419763Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:92

  2023-10-16T12:14:34.566015Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:92

  2023-10-16T12:14:35.005523Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:92

  2023-10-16T12:14:35.193612Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:92

  2023-10-16T12:14:35.943124Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:92

  2023-10-16T12:14:36.096004Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:92

  2023-10-16T12:14:36.236385Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:125

  2023-10-16T12:15:08.342914Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:125

  2023-10-16T12:15:08.343549Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:125

  2023-10-16T12:14:31.076089Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:92

  2023-10-16T12:14:31.300957Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:92

  2023-10-16T12:14:31.488643Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:92

  2023-10-16T12:14:31.664596Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:92

  2023-10-16T12:14:31.830495Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:92

  2023-10-16T12:14:32.089382Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:92

  2023-10-16T12:14:32.176492Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:92

  2023-10-16T12:14:32.294607Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:92

  2023-10-16T12:14:32.607753Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:92

  2023-10-16T12:14:33.078736Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:92

  2023-10-16T12:14:33.473182Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:92

  2023-10-16T12:14:34.141181Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:92

  2023-10-16T12:14:34.419763Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:92

  2023-10-16T12:14:34.566015Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:92

  2023-10-16T12:14:35.005523Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:92

  2023-10-16T12:14:35.193612Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:92

  2023-10-16T12:14:35.943124Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:92

  2023-10-16T12:14:36.096004Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:92

  2023-10-16T12:14:36.236385Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:125

  2023-10-16T12:15:08.342914Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:125

  2023-10-16T12:15:08.343549Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:125


```
</details>

