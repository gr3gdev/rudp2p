# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/3s-518ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-39ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-602ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-402ms-blue)
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/2s-911ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-37ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-403ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-0ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-603ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/3s-211ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-36ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-402ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-2ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-403ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-301ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/2s-497ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-34ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-402ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/0s-90ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-34ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-402ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-09T09:12:58.775168Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T09:12:58.776818Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T09:12:58.778766Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T09:12:58.780374Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T09:12:58.782247Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T09:12:58.784007Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T09:12:58.785735Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T09:12:58.787238Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T09:12:58.788953Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T09:12:58.790243Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T09:12:58.792781Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T09:12:58.794318Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T09:12:58.795676Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T09:12:58.797455Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T09:12:58.799418Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T09:12:58.800915Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T09:12:58.802128Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T09:12:58.803909Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T09:12:58.862450Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T09:12:58.962925Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T09:12:59.063199Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T09:12:59.163259Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T09:13:01.268885Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T09:13:01.369247Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T09:13:01.469443Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T09:13:01.569602Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T09:13:01.680311Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:137

  2023-10-09T09:13:01.780698Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:137

  2023-10-09T09:13:01.880881Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:137

  2023-10-09T09:13:01.981685Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:137

  2023-10-09T09:13:02.082066Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:137

  2023-10-09T09:13:02.182284Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:137

  2023-10-09T09:13:02.285505Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:137

  2023-10-09T09:13:02.385947Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:137

  2023-10-09T09:13:02.486160Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:137

  2023-10-09T09:13:02.586290Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:137

  2023-10-09T09:12:58.775168Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T09:12:58.776818Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T09:12:58.778766Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T09:12:58.780374Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T09:12:58.782247Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T09:12:58.784007Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T09:12:58.785735Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T09:12:58.787238Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T09:12:58.788953Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T09:12:58.790243Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T09:12:58.792781Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T09:12:58.794318Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T09:12:58.795676Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T09:12:58.797455Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T09:12:58.799418Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T09:12:58.800915Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T09:12:58.802128Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T09:12:58.803909Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T09:12:58.862450Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T09:12:58.962925Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T09:12:59.063199Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T09:12:59.163259Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T09:13:01.268885Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T09:13:01.369247Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T09:13:01.469443Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T09:13:01.569602Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T09:13:01.680311Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:137

  2023-10-09T09:13:01.780698Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:137

  2023-10-09T09:13:01.880881Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:137

  2023-10-09T09:13:01.981685Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:137

  2023-10-09T09:13:02.082066Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:137

  2023-10-09T09:13:02.182284Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:137

  2023-10-09T09:13:02.285505Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:137

  2023-10-09T09:13:02.385947Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:137

  2023-10-09T09:13:02.486160Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:137

  2023-10-09T09:13:02.586290Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:137

  2023-10-09T09:12:58.775168Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T09:12:58.776818Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T09:12:58.778766Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T09:12:58.780374Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T09:12:58.782247Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T09:12:58.784007Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T09:12:58.785735Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T09:12:58.787238Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T09:12:58.788953Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T09:12:58.790243Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T09:12:58.792781Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T09:12:58.794318Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T09:12:58.795676Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T09:12:58.797455Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T09:12:58.799418Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T09:12:58.800915Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T09:12:58.802128Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T09:12:58.803909Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T09:12:58.862450Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T09:12:58.962925Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T09:12:59.063199Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T09:12:59.163259Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T09:13:01.268885Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T09:13:01.369247Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T09:13:01.469443Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T09:13:01.569602Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T09:13:01.680311Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:137

  2023-10-09T09:13:01.780698Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:137

  2023-10-09T09:13:01.880881Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:137

  2023-10-09T09:13:01.981685Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:137

  2023-10-09T09:13:02.082066Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:137

  2023-10-09T09:13:02.182284Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:137

  2023-10-09T09:13:02.285505Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:137

  2023-10-09T09:13:02.385947Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:137

  2023-10-09T09:13:02.486160Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:137

  2023-10-09T09:13:02.586290Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:137

  2023-10-09T09:12:58.775168Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T09:12:58.776818Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T09:12:58.778766Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T09:12:58.780374Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T09:12:58.782247Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T09:12:58.784007Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T09:12:58.785735Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T09:12:58.787238Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T09:12:58.788953Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T09:12:58.790243Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T09:12:58.792781Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T09:12:58.794318Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T09:12:58.795676Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T09:12:58.797455Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T09:12:58.799418Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T09:12:58.800915Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T09:12:58.802128Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T09:12:58.803909Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T09:12:58.862450Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T09:12:58.962925Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T09:12:59.063199Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T09:12:59.163259Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137

  2023-10-09T09:13:01.268885Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:137

  2023-10-09T09:13:01.369247Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:137

  2023-10-09T09:13:01.469443Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:137

  2023-10-09T09:13:01.569602Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:137

  2023-10-09T09:12:58.775168Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:102

  2023-10-09T09:12:58.776818Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:102

  2023-10-09T09:12:58.778766Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:102

  2023-10-09T09:12:58.780374Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:102

  2023-10-09T09:12:58.782247Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:102

  2023-10-09T09:12:58.784007Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:102

  2023-10-09T09:12:58.785735Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:102

  2023-10-09T09:12:58.787238Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:102

  2023-10-09T09:12:58.788953Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:102

  2023-10-09T09:12:58.790243Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:102

  2023-10-09T09:12:58.792781Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:102

  2023-10-09T09:12:58.794318Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:102

  2023-10-09T09:12:58.795676Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:102

  2023-10-09T09:12:58.797455Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:102

  2023-10-09T09:12:58.799418Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:102

  2023-10-09T09:12:58.800915Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:102

  2023-10-09T09:12:58.802128Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:102

  2023-10-09T09:12:58.803909Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:102

  2023-10-09T09:12:58.862450Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:137

  2023-10-09T09:12:58.962925Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:137

  2023-10-09T09:12:59.063199Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:137

  2023-10-09T09:12:59.163259Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:137


```
</details>

