# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/122s-951ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-307ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-22ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-46ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-102ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-192ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/60s-181ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-94ms-blue)

```
Unable to read errors
```
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/62s-761ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-300ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-42ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-17ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-102ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-3ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-4ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-196ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/90s-275ms-blue)

```
Unable to read errors
```
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/92s-850ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-300ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-20ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-28ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-20ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-13ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-18ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-102ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-6ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-195ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-94ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/60s-184ms-blue)

```
Unable to read errors
```
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/32s-558ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-299ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-20ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-28ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-20ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-98ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-5ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-192ms-blue)

```
Unable to read errors
```
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/0s-447ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-296ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-20ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-29ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-22ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-97ms-blue)

```
Unable to read errors
```
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-31T10:27:26.436398Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.436800Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.439298Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.513408Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.514655Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.516712Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.516990Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.519443Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.519539Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.576455Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.578241Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.650254Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.651759Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.653956Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.725489Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.727116Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.728134Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.729952Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.885279Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.887008Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.887713Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:58.992091Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:59.092447Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:28:29.192514Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:28:59.283420Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:29:29.375512Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:29:29.376158Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:29:29.378231Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.436398Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.436800Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.439298Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.513408Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.514655Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.516712Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.516990Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.519443Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.519539Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.576455Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.578241Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.650254Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.651759Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.653956Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.725489Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.727116Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.728134Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.729952Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.885279Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.887008Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.887713Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:58.992091Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:59.092447Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:28:29.192514Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:28:59.283420Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:29:29.375512Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:29:29.376158Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:29:29.378231Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.436398Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.436800Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.439298Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.513408Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.514655Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.516712Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.516990Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.519443Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.519539Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.576455Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.578241Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.650254Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.651759Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.653956Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.725489Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.727116Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.728134Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.729952Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.885279Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.887008Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.887713Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:58.992091Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:59.092447Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:28:29.192514Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:28:59.283420Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:29:29.375512Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:29:29.376158Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:29:29.378231Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.436398Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.436800Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.439298Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.513408Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.514655Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.516712Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.516990Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.519443Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.519539Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.576455Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.578241Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.650254Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.651759Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.653956Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.725489Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.727116Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.728134Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.729952Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.885279Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.887008Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.887713Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:58.992091Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:59.092447Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.436398Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.436800Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.439298Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.513408Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.514655Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.516712Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.516990Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.519443Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.519539Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.576455Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.578241Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.650254Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.651759Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.653956Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.725489Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.727116Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.728134Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.729952Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-31T10:27:26.885279Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.887008Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at rudp2p/src/thread.rs:126

  2023-10-31T10:27:26.887713Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126


```
</details>

