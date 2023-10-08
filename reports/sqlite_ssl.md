# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-575ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-583ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-13ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-404ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-4ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-206ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-705ms-blue)
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-975ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-582ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-16ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-404ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-4ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-207ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-705ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-304ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-269ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-580ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-13ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-404ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-4ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-206ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-6ms-blue)
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-53ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-581ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-13ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-404ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-4ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-205ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/3s-642ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-576ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-404ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-08T14:49:49.464746Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-08T14:49:49.655302Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-08T14:49:49.794091Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-08T14:49:49.856606Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-08T14:49:50.185734Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-08T14:49:50.261059Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-08T14:49:50.495818Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-08T14:49:50.598716Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-08T14:49:50.861799Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-08T14:49:51.122275Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-08T14:49:51.257505Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-08T14:49:51.502946Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-08T14:49:51.687683Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-08T14:49:51.855357Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-08T14:49:52.009152Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-08T14:49:52.264191Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-08T14:49:52.406314Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-08T14:49:52.741077Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-08T14:49:52.808775Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-08T14:49:52.909004Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-08T14:49:53.009553Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-08T14:49:53.109715Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-08T14:49:55.218211Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-08T14:49:55.219218Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-08T14:49:55.220308Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-08T14:49:55.321155Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-08T14:49:55.433192Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-08T14:49:55.533880Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-08T14:49:55.634090Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-08T14:49:55.735524Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-08T14:49:55.836153Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-08T14:49:55.936626Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-08T14:49:56.036890Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-08T14:49:56.138159Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-08T14:49:56.238593Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-08T14:49:56.338821Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-08T14:49:49.464746Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-08T14:49:49.655302Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-08T14:49:49.794091Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-08T14:49:49.856606Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-08T14:49:50.185734Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-08T14:49:50.261059Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-08T14:49:50.495818Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-08T14:49:50.598716Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-08T14:49:50.861799Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-08T14:49:51.122275Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-08T14:49:51.257505Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-08T14:49:51.502946Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-08T14:49:51.687683Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-08T14:49:51.855357Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-08T14:49:52.009152Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-08T14:49:52.264191Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-08T14:49:52.406314Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-08T14:49:52.741077Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-08T14:49:52.808775Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-08T14:49:52.909004Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-08T14:49:53.009553Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-08T14:49:53.109715Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-08T14:49:55.218211Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-08T14:49:55.219218Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-08T14:49:55.220308Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-08T14:49:55.321155Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-08T14:49:55.433192Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-08T14:49:55.533880Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-08T14:49:55.634090Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-08T14:49:55.735524Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-08T14:49:55.836153Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-08T14:49:55.936626Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-08T14:49:56.036890Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-08T14:49:56.138159Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-08T14:49:56.238593Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-08T14:49:56.338821Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-08T14:49:49.464746Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-08T14:49:49.655302Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-08T14:49:49.794091Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-08T14:49:49.856606Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-08T14:49:50.185734Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-08T14:49:50.261059Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-08T14:49:50.495818Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-08T14:49:50.598716Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-08T14:49:50.861799Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-08T14:49:51.122275Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-08T14:49:51.257505Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-08T14:49:51.502946Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-08T14:49:51.687683Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-08T14:49:51.855357Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-08T14:49:52.009152Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-08T14:49:52.264191Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-08T14:49:52.406314Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-08T14:49:52.741077Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-08T14:49:52.808775Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-08T14:49:52.909004Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-08T14:49:53.009553Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-08T14:49:53.109715Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-08T14:49:55.218211Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-08T14:49:55.219218Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-08T14:49:55.220308Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-08T14:49:55.321155Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-08T14:49:55.433192Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-08T14:49:55.533880Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-08T14:49:55.634090Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-08T14:49:55.735524Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-08T14:49:55.836153Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-08T14:49:55.936626Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-08T14:49:56.036890Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-08T14:49:56.138159Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-08T14:49:56.238593Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-08T14:49:56.338821Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-08T14:49:49.464746Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-08T14:49:49.655302Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-08T14:49:49.794091Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-08T14:49:49.856606Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-08T14:49:50.185734Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-08T14:49:50.261059Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-08T14:49:50.495818Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-08T14:49:50.598716Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-08T14:49:50.861799Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-08T14:49:51.122275Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-08T14:49:51.257505Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-08T14:49:51.502946Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-08T14:49:51.687683Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-08T14:49:51.855357Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-08T14:49:52.009152Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-08T14:49:52.264191Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-08T14:49:52.406314Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-08T14:49:52.741077Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-08T14:49:52.808775Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-08T14:49:52.909004Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-08T14:49:53.009553Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-08T14:49:53.109715Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-08T14:49:55.218211Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-08T14:49:55.219218Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-08T14:49:55.220308Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-08T14:49:55.321155Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-08T14:49:49.464746Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-08T14:49:49.655302Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-08T14:49:49.794091Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-08T14:49:49.856606Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-08T14:49:50.185734Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-08T14:49:50.261059Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-08T14:49:50.495818Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-08T14:49:50.598716Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-08T14:49:50.861799Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-08T14:49:51.122275Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-08T14:49:51.257505Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-08T14:49:51.502946Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-08T14:49:51.687683Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-08T14:49:51.855357Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-08T14:49:52.009152Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-08T14:49:52.264191Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-08T14:49:52.406314Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-08T14:49:52.741077Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-08T14:49:52.808775Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-08T14:49:52.909004Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-08T14:49:53.009553Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-08T14:49:53.109715Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136


```
</details>

