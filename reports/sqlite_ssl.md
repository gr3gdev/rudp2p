# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-687ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-189ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-403ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-404ms-blue)
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-682ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-184ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-404ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-405ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-682ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-183ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-8ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-403ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-405ms-blue)
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-250ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-181ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-2ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/4s-236ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-182ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-09T09:13:30.333382Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T09:13:30.593120Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T09:13:30.752760Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T09:13:31.051095Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T09:13:31.409708Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T09:13:31.792961Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T09:13:31.952893Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T09:13:32.085464Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T09:13:32.174901Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T09:13:32.256612Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T09:13:32.533268Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T09:13:32.730251Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T09:13:32.848791Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T09:13:33.202727Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T09:13:33.636056Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T09:13:33.902945Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T09:13:34.232770Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T09:13:34.357726Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T09:13:34.415743Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T09:13:34.416533Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T09:13:34.417144Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T09:13:34.417589Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T09:13:36.428694Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T09:13:36.528982Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T09:13:36.629230Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T09:13:36.729397Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T09:13:36.857840Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:137

  2023-10-09T09:13:36.858469Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:137

  2023-10-09T09:13:36.858968Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:137

  2023-10-09T09:13:36.859575Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:137

  2023-10-09T09:13:36.860578Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:137

  2023-10-09T09:13:36.860997Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:137

  2023-10-09T09:13:36.862003Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:137

  2023-10-09T09:13:36.962598Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:137

  2023-10-09T09:13:37.062778Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:137

  2023-10-09T09:13:37.163059Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:137

  2023-10-09T09:13:30.333382Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T09:13:30.593120Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T09:13:30.752760Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T09:13:31.051095Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T09:13:31.409708Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T09:13:31.792961Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T09:13:31.952893Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T09:13:32.085464Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T09:13:32.174901Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T09:13:32.256612Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T09:13:32.533268Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T09:13:32.730251Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T09:13:32.848791Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T09:13:33.202727Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T09:13:33.636056Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T09:13:33.902945Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T09:13:34.232770Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T09:13:34.357726Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T09:13:34.415743Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T09:13:34.416533Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T09:13:34.417144Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T09:13:34.417589Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T09:13:36.428694Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T09:13:36.528982Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T09:13:36.629230Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T09:13:36.729397Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T09:13:36.857840Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:137

  2023-10-09T09:13:36.858469Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:137

  2023-10-09T09:13:36.858968Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:137

  2023-10-09T09:13:36.859575Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:137

  2023-10-09T09:13:36.860578Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:137

  2023-10-09T09:13:36.860997Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:137

  2023-10-09T09:13:36.862003Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:137

  2023-10-09T09:13:36.962598Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:137

  2023-10-09T09:13:37.062778Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:137

  2023-10-09T09:13:37.163059Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:137

  2023-10-09T09:13:30.333382Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T09:13:30.593120Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T09:13:30.752760Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T09:13:31.051095Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T09:13:31.409708Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T09:13:31.792961Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T09:13:31.952893Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T09:13:32.085464Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T09:13:32.174901Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T09:13:32.256612Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T09:13:32.533268Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T09:13:32.730251Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T09:13:32.848791Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T09:13:33.202727Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T09:13:33.636056Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T09:13:33.902945Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T09:13:34.232770Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T09:13:34.357726Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T09:13:34.415743Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T09:13:34.416533Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T09:13:34.417144Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T09:13:34.417589Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T09:13:36.428694Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T09:13:36.528982Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T09:13:36.629230Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T09:13:36.729397Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T09:13:36.857840Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:137

  2023-10-09T09:13:36.858469Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:137

  2023-10-09T09:13:36.858968Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:137

  2023-10-09T09:13:36.859575Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:137

  2023-10-09T09:13:36.860578Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:137

  2023-10-09T09:13:36.860997Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:137

  2023-10-09T09:13:36.862003Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:137

  2023-10-09T09:13:36.962598Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:137

  2023-10-09T09:13:37.062778Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:137

  2023-10-09T09:13:37.163059Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:137

  2023-10-09T09:13:30.333382Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T09:13:30.593120Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T09:13:30.752760Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T09:13:31.051095Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T09:13:31.409708Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T09:13:31.792961Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T09:13:31.952893Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T09:13:32.085464Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T09:13:32.174901Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T09:13:32.256612Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T09:13:32.533268Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T09:13:32.730251Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T09:13:32.848791Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T09:13:33.202727Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T09:13:33.636056Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T09:13:33.902945Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T09:13:34.232770Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T09:13:34.357726Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T09:13:34.415743Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T09:13:34.416533Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T09:13:34.417144Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T09:13:34.417589Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T09:13:36.428694Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T09:13:36.528982Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T09:13:36.629230Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T09:13:36.729397Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T09:13:30.333382Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T09:13:30.593120Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T09:13:30.752760Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T09:13:31.051095Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T09:13:31.409708Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T09:13:31.792961Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T09:13:31.952893Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T09:13:32.085464Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T09:13:32.174901Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T09:13:32.256612Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T09:13:32.533268Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T09:13:32.730251Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T09:13:32.848791Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T09:13:33.202727Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T09:13:33.636056Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T09:13:33.902945Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T09:13:34.232770Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T09:13:34.357726Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T09:13:34.415743Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T09:13:34.416533Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T09:13:34.417144Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T09:13:34.417589Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137


```
</details>

