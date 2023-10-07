# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/13s-513ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-28ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-403ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-3ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-314ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
</details>



## Feature : Block peers ![Failed](https://img.shields.io/badge/Failed-red)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/13s-209ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-24ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-314ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-703ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/16-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/12s-906ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-22ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-403ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-405ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P2" receives (line 87) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/5s-1ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P2 has not received the message from P1
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-6ms-blue)
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/7s-486ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-25ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-402ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-2ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/5s-77ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-19ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-404ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-07T18:02:46.400307Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-07T18:02:46.641679Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-07T18:02:46.771118Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-07T18:02:47.197998Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-07T18:02:47.648863Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-07T18:02:47.769276Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-07T18:02:48.002263Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-07T18:02:48.309199Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-07T18:02:49.278911Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-07T18:02:49.416530Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-07T18:02:50.091665Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-07T18:02:50.174106Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-07T18:02:50.386260Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-07T18:02:50.520636Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-07T18:02:50.721924Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-07T18:02:50.972275Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-07T18:02:51.108858Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-07T18:02:51.246007Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-07T18:02:51.305072Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-07T18:02:51.405613Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-07T18:02:51.505749Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-07T18:02:51.605897Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-07T18:02:53.712565Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-07T18:02:53.813397Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-07T18:02:53.913209Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-07T18:02:54.013442Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-07T18:02:59.132495Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-07T18:02:59.232660Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-07T18:02:59.332941Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-07T18:02:59.434002Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-07T18:02:59.534412Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-07T18:02:59.634547Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-07T18:02:59.735797Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-07T18:02:59.836252Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-07T18:02:59.936289Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-07T18:03:00.036695Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-07T18:02:46.400307Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-07T18:02:46.641679Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-07T18:02:46.771118Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-07T18:02:47.197998Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-07T18:02:47.648863Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-07T18:02:47.769276Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-07T18:02:48.002263Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-07T18:02:48.309199Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-07T18:02:49.278911Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-07T18:02:49.416530Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-07T18:02:50.091665Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-07T18:02:50.174106Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-07T18:02:50.386260Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-07T18:02:50.520636Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-07T18:02:50.721924Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-07T18:02:50.972275Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-07T18:02:51.108858Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-07T18:02:51.246007Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-07T18:02:51.305072Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-07T18:02:51.405613Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-07T18:02:51.505749Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-07T18:02:51.605897Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-07T18:02:53.712565Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-07T18:02:53.813397Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-07T18:02:53.913209Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-07T18:02:54.013442Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-07T18:02:59.132495Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-07T18:02:59.232660Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-07T18:02:59.332941Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-07T18:02:59.434002Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-07T18:02:59.534412Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-07T18:02:59.634547Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-07T18:02:59.735797Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-07T18:02:59.836252Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-07T18:02:59.936289Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-07T18:03:00.036695Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-07T18:02:46.400307Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-07T18:02:46.641679Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-07T18:02:46.771118Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-07T18:02:47.197998Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-07T18:02:47.648863Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-07T18:02:47.769276Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-07T18:02:48.002263Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-07T18:02:48.309199Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-07T18:02:49.278911Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-07T18:02:49.416530Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-07T18:02:50.091665Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-07T18:02:50.174106Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-07T18:02:50.386260Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-07T18:02:50.520636Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-07T18:02:50.721924Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-07T18:02:50.972275Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-07T18:02:51.108858Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-07T18:02:51.246007Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-07T18:02:51.305072Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-07T18:02:51.405613Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-07T18:02:51.505749Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-07T18:02:51.605897Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-07T18:02:53.712565Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-07T18:02:53.813397Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-07T18:02:53.913209Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-07T18:02:54.013442Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-07T18:02:59.132495Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-07T18:02:59.232660Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-07T18:02:59.332941Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-07T18:02:59.434002Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-07T18:02:59.534412Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-07T18:02:59.634547Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-07T18:02:59.735797Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-07T18:02:59.836252Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-07T18:02:59.936289Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-07T18:03:00.036695Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-07T18:02:46.400307Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-07T18:02:46.641679Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-07T18:02:46.771118Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-07T18:02:47.197998Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-07T18:02:47.648863Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-07T18:02:47.769276Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-07T18:02:48.002263Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-07T18:02:48.309199Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-07T18:02:49.278911Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-07T18:02:49.416530Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-07T18:02:50.091665Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-07T18:02:50.174106Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-07T18:02:50.386260Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-07T18:02:50.520636Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-07T18:02:50.721924Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-07T18:02:50.972275Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-07T18:02:51.108858Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-07T18:02:51.246007Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-07T18:02:51.305072Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-07T18:02:51.405613Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-07T18:02:51.505749Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-07T18:02:51.605897Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-07T18:02:53.712565Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-07T18:02:53.813397Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-07T18:02:53.913209Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-07T18:02:54.013442Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-07T18:02:46.400307Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-07T18:02:46.641679Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-07T18:02:46.771118Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-07T18:02:47.197998Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-07T18:02:47.648863Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-07T18:02:47.769276Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-07T18:02:48.002263Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-07T18:02:48.309199Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-07T18:02:49.278911Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-07T18:02:49.416530Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-07T18:02:50.091665Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-07T18:02:50.174106Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-07T18:02:50.386260Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-07T18:02:50.520636Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-07T18:02:50.721924Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-07T18:02:50.972275Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-07T18:02:51.108858Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-07T18:02:51.246007Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-07T18:02:51.305072Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-07T18:02:51.405613Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-07T18:02:51.505749Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-07T18:02:51.605897Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-07T18:02:53.712565Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-07T18:02:53.813397Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-07T18:02:53.913209Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-07T18:02:54.013442Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136


```
</details>

