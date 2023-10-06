# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/8s-324ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-131ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-25ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-106ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-3ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-403ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-304ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/7s-717ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-127ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-25ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-105ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-405ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-603ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/8s-18ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-128ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-17ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-107ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-3ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-404ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-304ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-704ms-blue)
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/7s-295ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-127ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-25ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-105ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/5s-186ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-125ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-17ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-105ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-06T18:27:35.696453Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-06T18:27:35.973978Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-06T18:27:36.212006Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-06T18:27:36.466468Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-06T18:27:36.782224Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-06T18:27:37.090246Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-06T18:27:37.411030Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-06T18:27:37.931565Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-06T18:27:38.226367Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-06T18:27:38.356686Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-06T18:27:38.523686Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-06T18:27:38.819120Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-06T18:27:39.120934Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-06T18:27:39.245029Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-06T18:27:39.332671Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-06T18:27:39.398062Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-06T18:27:39.711855Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-06T18:27:40.545431Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-06T18:27:40.608267Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-06T18:27:40.609480Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-06T18:27:40.610759Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-06T18:27:40.611285Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-06T18:27:42.716926Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-06T18:27:42.817363Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-06T18:27:42.917512Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-06T18:27:43.017806Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-06T18:27:43.136657Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-06T18:27:43.237253Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-06T18:27:43.337428Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-06T18:27:43.438763Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-06T18:27:43.539242Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-06T18:27:43.639403Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-06T18:27:43.741083Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-06T18:27:43.841613Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-06T18:27:43.941963Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-06T18:27:44.042170Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-06T18:27:35.696453Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-06T18:27:35.973978Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-06T18:27:36.212006Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-06T18:27:36.466468Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-06T18:27:36.782224Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-06T18:27:37.090246Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-06T18:27:37.411030Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-06T18:27:37.931565Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-06T18:27:38.226367Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-06T18:27:38.356686Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-06T18:27:38.523686Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-06T18:27:38.819120Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-06T18:27:39.120934Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-06T18:27:39.245029Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-06T18:27:39.332671Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-06T18:27:39.398062Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-06T18:27:39.711855Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-06T18:27:40.545431Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-06T18:27:40.608267Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-06T18:27:40.609480Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-06T18:27:40.610759Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-06T18:27:40.611285Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-06T18:27:42.716926Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-06T18:27:42.817363Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-06T18:27:42.917512Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-06T18:27:43.017806Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-06T18:27:43.136657Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-06T18:27:43.237253Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-06T18:27:43.337428Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-06T18:27:43.438763Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-06T18:27:43.539242Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-06T18:27:43.639403Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-06T18:27:43.741083Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-06T18:27:43.841613Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-06T18:27:43.941963Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-06T18:27:44.042170Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-06T18:27:35.696453Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-06T18:27:35.973978Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-06T18:27:36.212006Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-06T18:27:36.466468Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-06T18:27:36.782224Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-06T18:27:37.090246Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-06T18:27:37.411030Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-06T18:27:37.931565Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-06T18:27:38.226367Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-06T18:27:38.356686Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-06T18:27:38.523686Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-06T18:27:38.819120Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-06T18:27:39.120934Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-06T18:27:39.245029Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-06T18:27:39.332671Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-06T18:27:39.398062Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-06T18:27:39.711855Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-06T18:27:40.545431Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-06T18:27:40.608267Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-06T18:27:40.609480Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-06T18:27:40.610759Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-06T18:27:40.611285Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-06T18:27:42.716926Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-06T18:27:42.817363Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-06T18:27:42.917512Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-06T18:27:43.017806Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-06T18:27:43.136657Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-06T18:27:43.237253Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-06T18:27:43.337428Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-06T18:27:43.438763Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-06T18:27:43.539242Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-06T18:27:43.639403Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-06T18:27:43.741083Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-06T18:27:43.841613Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-06T18:27:43.941963Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-06T18:27:44.042170Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-06T18:27:35.696453Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-06T18:27:35.973978Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-06T18:27:36.212006Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-06T18:27:36.466468Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-06T18:27:36.782224Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-06T18:27:37.090246Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-06T18:27:37.411030Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-06T18:27:37.931565Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-06T18:27:38.226367Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-06T18:27:38.356686Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-06T18:27:38.523686Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-06T18:27:38.819120Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-06T18:27:39.120934Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-06T18:27:39.245029Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-06T18:27:39.332671Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-06T18:27:39.398062Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-06T18:27:39.711855Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-06T18:27:40.545431Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-06T18:27:40.608267Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-06T18:27:40.609480Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-06T18:27:40.610759Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-06T18:27:40.611285Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-06T18:27:42.716926Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-06T18:27:42.817363Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-06T18:27:42.917512Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-06T18:27:43.017806Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-06T18:27:35.696453Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-06T18:27:35.973978Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-06T18:27:36.212006Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-06T18:27:36.466468Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-06T18:27:36.782224Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-06T18:27:37.090246Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-06T18:27:37.411030Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-06T18:27:37.931565Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-06T18:27:38.226367Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-06T18:27:38.356686Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-06T18:27:38.523686Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-06T18:27:38.819120Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-06T18:27:39.120934Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-06T18:27:39.245029Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-06T18:27:39.332671Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-06T18:27:39.398062Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-06T18:27:39.711855Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-06T18:27:40.545431Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-06T18:27:40.608267Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-06T18:27:40.609480Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-06T18:27:40.610759Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-06T18:27:40.611285Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-06T18:27:42.716926Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-06T18:27:42.817363Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-06T18:27:42.917512Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-06T18:27:43.017806Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136


```
</details>

