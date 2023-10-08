# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/3s-229ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-43ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-304ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-3ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-206ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-604ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-404ms-blue)
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/2s-622ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-43ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-305ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-5ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-206ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-7ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/2s-922ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-41ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-304ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-4ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-206ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/2s-409ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-41ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-305ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-205ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/0s-98ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-40ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-305ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-08T14:48:12.144871Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-08T14:48:12.146497Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-08T14:48:12.148264Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-08T14:48:12.149793Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-08T14:48:12.151675Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-08T14:48:12.153724Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-08T14:48:12.155407Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-08T14:48:12.157459Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-08T14:48:12.159405Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-08T14:48:12.161419Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-08T14:48:12.163866Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-08T14:48:12.165919Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-08T14:48:12.167869Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-08T14:48:12.169898Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-08T14:48:12.171946Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-08T14:48:12.174286Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-08T14:48:12.176402Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-08T14:48:12.178278Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-08T14:48:12.239244Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-08T14:48:12.240183Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-08T14:48:12.340709Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-08T14:48:12.440975Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-08T14:48:14.549083Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-08T14:48:14.550808Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-08T14:48:14.551709Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-08T14:48:14.652298Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-08T14:48:14.760040Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-08T14:48:14.860715Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-08T14:48:14.961048Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-08T14:48:15.062191Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-08T14:48:15.162640Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-08T14:48:15.262942Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-08T14:48:15.366078Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-08T14:48:15.466496Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-08T14:48:15.567000Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-08T14:48:15.667467Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-08T14:48:12.144871Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-08T14:48:12.146497Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-08T14:48:12.148264Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-08T14:48:12.149793Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-08T14:48:12.151675Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-08T14:48:12.153724Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-08T14:48:12.155407Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-08T14:48:12.157459Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-08T14:48:12.159405Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-08T14:48:12.161419Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-08T14:48:12.163866Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-08T14:48:12.165919Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-08T14:48:12.167869Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-08T14:48:12.169898Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-08T14:48:12.171946Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-08T14:48:12.174286Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-08T14:48:12.176402Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-08T14:48:12.178278Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-08T14:48:12.239244Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-08T14:48:12.240183Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-08T14:48:12.340709Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-08T14:48:12.440975Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-08T14:48:14.549083Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-08T14:48:14.550808Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-08T14:48:14.551709Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-08T14:48:14.652298Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-08T14:48:14.760040Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-08T14:48:14.860715Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-08T14:48:14.961048Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-08T14:48:15.062191Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-08T14:48:15.162640Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-08T14:48:15.262942Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-08T14:48:15.366078Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-08T14:48:15.466496Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-08T14:48:15.567000Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-08T14:48:15.667467Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-08T14:48:12.144871Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-08T14:48:12.146497Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-08T14:48:12.148264Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-08T14:48:12.149793Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-08T14:48:12.151675Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-08T14:48:12.153724Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-08T14:48:12.155407Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-08T14:48:12.157459Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-08T14:48:12.159405Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-08T14:48:12.161419Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-08T14:48:12.163866Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-08T14:48:12.165919Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-08T14:48:12.167869Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-08T14:48:12.169898Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-08T14:48:12.171946Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-08T14:48:12.174286Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-08T14:48:12.176402Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-08T14:48:12.178278Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-08T14:48:12.239244Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-08T14:48:12.240183Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-08T14:48:12.340709Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-08T14:48:12.440975Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-08T14:48:14.549083Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-08T14:48:14.550808Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-08T14:48:14.551709Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-08T14:48:14.652298Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-08T14:48:14.760040Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-08T14:48:14.860715Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-08T14:48:14.961048Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-08T14:48:15.062191Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-08T14:48:15.162640Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-08T14:48:15.262942Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-08T14:48:15.366078Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-08T14:48:15.466496Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-08T14:48:15.567000Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-08T14:48:15.667467Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-08T14:48:12.144871Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-08T14:48:12.146497Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-08T14:48:12.148264Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-08T14:48:12.149793Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-08T14:48:12.151675Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-08T14:48:12.153724Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-08T14:48:12.155407Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-08T14:48:12.157459Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-08T14:48:12.159405Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-08T14:48:12.161419Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-08T14:48:12.163866Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-08T14:48:12.165919Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-08T14:48:12.167869Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-08T14:48:12.169898Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-08T14:48:12.171946Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-08T14:48:12.174286Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-08T14:48:12.176402Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-08T14:48:12.178278Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-08T14:48:12.239244Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-08T14:48:12.240183Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-08T14:48:12.340709Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-08T14:48:12.440975Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-08T14:48:14.549083Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-08T14:48:14.550808Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-08T14:48:14.551709Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-08T14:48:14.652298Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-08T14:48:12.144871Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-08T14:48:12.146497Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-08T14:48:12.148264Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-08T14:48:12.149793Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-08T14:48:12.151675Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-08T14:48:12.153724Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-08T14:48:12.155407Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-08T14:48:12.157459Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-08T14:48:12.159405Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-08T14:48:12.161419Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-08T14:48:12.163866Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-08T14:48:12.165919Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-08T14:48:12.167869Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-08T14:48:12.169898Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-08T14:48:12.171946Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-08T14:48:12.174286Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-08T14:48:12.176402Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-08T14:48:12.178278Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-08T14:48:12.239244Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-08T14:48:12.240183Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-08T14:48:12.340709Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-08T14:48:12.440975Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-08T14:48:14.549083Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-08T14:48:14.550808Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-08T14:48:14.551709Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-08T14:48:14.652298Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136


```
</details>

