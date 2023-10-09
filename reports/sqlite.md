# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/3s-509ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-37ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-406ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-7ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-405ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-305ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-402ms-blue)
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/3s-199ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-405ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-8ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-4ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-405ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-303ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-305ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/2s-895ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-35ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-405ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-5ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-405ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-605ms-blue)
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/2s-487ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-35ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-405ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-8ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/0s-68ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-30ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-407ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-09T08:17:07.696555Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T08:17:07.697658Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T08:17:07.698762Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T08:17:07.699775Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T08:17:07.702751Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T08:17:07.704151Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T08:17:07.706066Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T08:17:07.708887Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T08:17:07.710674Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T08:17:07.711888Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T08:17:07.713599Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T08:17:07.714682Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T08:17:07.715713Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T08:17:07.716751Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T08:17:07.719282Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T08:17:07.721926Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T08:17:07.723465Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T08:17:07.724507Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T08:17:07.763845Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T08:17:07.864436Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T08:17:07.965105Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T08:17:08.065134Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T08:17:10.181712Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T08:17:10.281666Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T08:17:10.382115Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T08:17:10.482499Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T08:17:10.588189Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:137

  2023-10-09T08:17:10.688443Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:137

  2023-10-09T08:17:10.788945Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:137

  2023-10-09T08:17:10.892061Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:137

  2023-10-09T08:17:10.992658Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:137

  2023-10-09T08:17:11.092778Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:137

  2023-10-09T08:17:11.200417Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:137

  2023-10-09T08:17:11.300611Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:137

  2023-10-09T08:17:11.400891Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:137

  2023-10-09T08:17:11.501375Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:137

  2023-10-09T08:17:07.696555Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T08:17:07.697658Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T08:17:07.698762Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T08:17:07.699775Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T08:17:07.702751Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T08:17:07.704151Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T08:17:07.706066Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T08:17:07.708887Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T08:17:07.710674Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T08:17:07.711888Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T08:17:07.713599Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T08:17:07.714682Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T08:17:07.715713Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T08:17:07.716751Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T08:17:07.719282Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T08:17:07.721926Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T08:17:07.723465Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T08:17:07.724507Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T08:17:07.763845Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T08:17:07.864436Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T08:17:07.965105Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T08:17:08.065134Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T08:17:10.181712Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T08:17:10.281666Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T08:17:10.382115Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T08:17:10.482499Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T08:17:10.588189Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:137

  2023-10-09T08:17:10.688443Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:137

  2023-10-09T08:17:10.788945Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:137

  2023-10-09T08:17:10.892061Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:137

  2023-10-09T08:17:10.992658Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:137

  2023-10-09T08:17:11.092778Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:137

  2023-10-09T08:17:11.200417Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:137

  2023-10-09T08:17:11.300611Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:137

  2023-10-09T08:17:11.400891Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:137

  2023-10-09T08:17:11.501375Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:137

  2023-10-09T08:17:07.696555Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T08:17:07.697658Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T08:17:07.698762Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T08:17:07.699775Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T08:17:07.702751Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T08:17:07.704151Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T08:17:07.706066Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T08:17:07.708887Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T08:17:07.710674Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T08:17:07.711888Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T08:17:07.713599Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T08:17:07.714682Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T08:17:07.715713Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T08:17:07.716751Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T08:17:07.719282Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T08:17:07.721926Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T08:17:07.723465Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T08:17:07.724507Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T08:17:07.763845Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T08:17:07.864436Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T08:17:07.965105Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T08:17:08.065134Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T08:17:10.181712Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T08:17:10.281666Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T08:17:10.382115Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T08:17:10.482499Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T08:17:10.588189Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:137

  2023-10-09T08:17:10.688443Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:137

  2023-10-09T08:17:10.788945Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:137

  2023-10-09T08:17:10.892061Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:137

  2023-10-09T08:17:10.992658Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:137

  2023-10-09T08:17:11.092778Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:137

  2023-10-09T08:17:07.696555Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T08:17:07.697658Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T08:17:07.698762Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T08:17:07.699775Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T08:17:07.702751Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T08:17:07.704151Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T08:17:07.706066Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T08:17:07.708887Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T08:17:07.710674Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T08:17:07.711888Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T08:17:07.713599Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T08:17:07.714682Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T08:17:07.715713Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T08:17:07.716751Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T08:17:07.719282Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T08:17:07.721926Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T08:17:07.723465Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T08:17:07.724507Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T08:17:07.763845Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T08:17:07.864436Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T08:17:07.965105Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T08:17:08.065134Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T08:17:10.181712Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T08:17:10.281666Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T08:17:10.382115Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T08:17:10.482499Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T08:17:07.696555Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T08:17:07.697658Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T08:17:07.698762Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T08:17:07.699775Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T08:17:07.702751Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T08:17:07.704151Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T08:17:07.706066Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T08:17:07.708887Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T08:17:07.710674Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T08:17:07.711888Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T08:17:07.713599Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T08:17:07.714682Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T08:17:07.715713Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T08:17:07.716751Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T08:17:07.719282Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T08:17:07.721926Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T08:17:07.723465Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T08:17:07.724507Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T08:17:07.763845Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T08:17:07.864436Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T08:17:07.965105Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T08:17:08.065134Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137


```
</details>

