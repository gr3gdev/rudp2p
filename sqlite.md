# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/8s-384ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-984ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-304ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-406ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-603ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-402ms-blue)
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/7s-777ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-982ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-303ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-405ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-603ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/8s-78ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-979ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-13ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-304ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-3ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-403ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/7s-358ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-979ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-21ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-303ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/5s-51ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-976ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-13ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-303ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-05T14:22:29.552949Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:131

  2023-10-05T14:22:29.915913Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:131

  2023-10-05T14:22:30.139898Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:131

  2023-10-05T14:22:30.851896Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:131

  2023-10-05T14:22:31.122401Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:131

  2023-10-05T14:22:31.384127Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:131

  2023-10-05T14:22:31.567483Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:131

  2023-10-05T14:22:31.786620Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:131

  2023-10-05T14:22:31.929317Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:131

  2023-10-05T14:22:32.463339Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:131

  2023-10-05T14:22:32.601733Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:131

  2023-10-05T14:22:32.738089Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:131

  2023-10-05T14:22:32.930907Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:131

  2023-10-05T14:22:33.087723Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:131

  2023-10-05T14:22:33.642561Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:131

  2023-10-05T14:22:33.931117Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:131

  2023-10-05T14:22:34.075253Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:131

  2023-10-05T14:22:34.307158Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:131

  2023-10-05T14:22:34.383360Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:166

  2023-10-05T14:22:34.384135Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:166

  2023-10-05T14:22:34.484044Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:166

  2023-10-05T14:22:34.584230Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:166

  2023-10-05T14:22:36.690116Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:166

  2023-10-05T14:22:36.790436Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:166

  2023-10-05T14:22:36.891026Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:166

  2023-10-05T14:22:36.991151Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:166

  2023-10-05T14:22:37.106904Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:166

  2023-10-05T14:22:37.207455Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:166

  2023-10-05T14:22:37.307660Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:166

  2023-10-05T14:22:37.408484Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:166

  2023-10-05T14:22:37.508691Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:166

  2023-10-05T14:22:37.608888Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:166

  2023-10-05T14:22:37.712422Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:166

  2023-10-05T14:22:37.812856Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:166

  2023-10-05T14:22:37.912983Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:166

  2023-10-05T14:22:38.013300Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:166

  2023-10-05T14:22:29.552949Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:131

  2023-10-05T14:22:29.915913Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:131

  2023-10-05T14:22:30.139898Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:131

  2023-10-05T14:22:30.851896Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:131

  2023-10-05T14:22:31.122401Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:131

  2023-10-05T14:22:31.384127Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:131

  2023-10-05T14:22:31.567483Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:131

  2023-10-05T14:22:31.786620Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:131

  2023-10-05T14:22:31.929317Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:131

  2023-10-05T14:22:32.463339Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:131

  2023-10-05T14:22:32.601733Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:131

  2023-10-05T14:22:32.738089Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:131

  2023-10-05T14:22:32.930907Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:131

  2023-10-05T14:22:33.087723Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:131

  2023-10-05T14:22:33.642561Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:131

  2023-10-05T14:22:33.931117Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:131

  2023-10-05T14:22:34.075253Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:131

  2023-10-05T14:22:34.307158Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:131

  2023-10-05T14:22:34.383360Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:166

  2023-10-05T14:22:34.384135Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:166

  2023-10-05T14:22:34.484044Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:166

  2023-10-05T14:22:34.584230Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:166

  2023-10-05T14:22:36.690116Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:166

  2023-10-05T14:22:36.790436Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:166

  2023-10-05T14:22:36.891026Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:166

  2023-10-05T14:22:36.991151Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:166

  2023-10-05T14:22:37.106904Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:166

  2023-10-05T14:22:37.207455Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:166

  2023-10-05T14:22:37.307660Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:166

  2023-10-05T14:22:37.408484Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:166

  2023-10-05T14:22:37.508691Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:166

  2023-10-05T14:22:37.608888Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:166

  2023-10-05T14:22:37.712422Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:166

  2023-10-05T14:22:37.812856Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:166

  2023-10-05T14:22:37.912983Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:166

  2023-10-05T14:22:38.013300Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:166

  2023-10-05T14:22:29.552949Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:131

  2023-10-05T14:22:29.915913Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:131

  2023-10-05T14:22:30.139898Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:131

  2023-10-05T14:22:30.851896Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:131

  2023-10-05T14:22:31.122401Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:131

  2023-10-05T14:22:31.384127Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:131

  2023-10-05T14:22:31.567483Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:131

  2023-10-05T14:22:31.786620Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:131

  2023-10-05T14:22:31.929317Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:131

  2023-10-05T14:22:32.463339Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:131

  2023-10-05T14:22:32.601733Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:131

  2023-10-05T14:22:32.738089Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:131

  2023-10-05T14:22:32.930907Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:131

  2023-10-05T14:22:33.087723Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:131

  2023-10-05T14:22:33.642561Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:131

  2023-10-05T14:22:33.931117Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:131

  2023-10-05T14:22:34.075253Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:131

  2023-10-05T14:22:34.307158Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:131

  2023-10-05T14:22:34.383360Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:166

  2023-10-05T14:22:34.384135Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:166

  2023-10-05T14:22:34.484044Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:166

  2023-10-05T14:22:34.584230Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:166

  2023-10-05T14:22:36.690116Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:166

  2023-10-05T14:22:36.790436Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:166

  2023-10-05T14:22:36.891026Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:166

  2023-10-05T14:22:36.991151Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:166

  2023-10-05T14:22:37.106904Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:166

  2023-10-05T14:22:37.207455Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:166

  2023-10-05T14:22:37.307660Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:166

  2023-10-05T14:22:37.408484Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:166

  2023-10-05T14:22:37.508691Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:166

  2023-10-05T14:22:37.608888Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:166

  2023-10-05T14:22:37.712422Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:166

  2023-10-05T14:22:37.812856Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:166

  2023-10-05T14:22:37.912983Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:166

  2023-10-05T14:22:38.013300Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:166

  2023-10-05T14:22:29.552949Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:131

  2023-10-05T14:22:29.915913Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:131

  2023-10-05T14:22:30.139898Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:131

  2023-10-05T14:22:30.851896Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:131

  2023-10-05T14:22:31.122401Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:131

  2023-10-05T14:22:31.384127Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:131

  2023-10-05T14:22:31.567483Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:131

  2023-10-05T14:22:31.786620Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:131

  2023-10-05T14:22:31.929317Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:131

  2023-10-05T14:22:32.463339Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:131

  2023-10-05T14:22:32.601733Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:131

  2023-10-05T14:22:32.738089Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:131

  2023-10-05T14:22:32.930907Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:131

  2023-10-05T14:22:33.087723Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:131

  2023-10-05T14:22:33.642561Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:131

  2023-10-05T14:22:33.931117Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:131

  2023-10-05T14:22:34.075253Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:131

  2023-10-05T14:22:34.307158Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:131

  2023-10-05T14:22:34.383360Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:166

  2023-10-05T14:22:34.384135Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:166

  2023-10-05T14:22:34.484044Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:166

  2023-10-05T14:22:34.584230Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:166

  2023-10-05T14:22:36.690116Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:166

  2023-10-05T14:22:36.790436Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:166

  2023-10-05T14:22:36.891026Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:166

  2023-10-05T14:22:36.991151Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:166

  2023-10-05T14:22:29.552949Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:131

  2023-10-05T14:22:29.915913Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:131

  2023-10-05T14:22:30.139898Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:131

  2023-10-05T14:22:30.851896Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:131

  2023-10-05T14:22:31.122401Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:131

  2023-10-05T14:22:31.384127Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:131

  2023-10-05T14:22:31.567483Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:131

  2023-10-05T14:22:31.786620Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:131

  2023-10-05T14:22:31.929317Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:131

  2023-10-05T14:22:32.463339Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:131

  2023-10-05T14:22:32.601733Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:131

  2023-10-05T14:22:32.738089Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:131

  2023-10-05T14:22:32.930907Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:131

  2023-10-05T14:22:33.087723Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:131

  2023-10-05T14:22:33.642561Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:131

  2023-10-05T14:22:33.931117Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:131

  2023-10-05T14:22:34.075253Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:131

  2023-10-05T14:22:34.307158Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:131

  2023-10-05T14:22:34.383360Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:166

  2023-10-05T14:22:34.384135Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:166

  2023-10-05T14:22:34.484044Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:166

  2023-10-05T14:22:34.584230Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:166

  2023-10-05T14:22:36.690116Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:166

  2023-10-05T14:22:36.790436Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:166

  2023-10-05T14:22:36.891026Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:166

  2023-10-05T14:22:36.991151Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:166


```
</details>

