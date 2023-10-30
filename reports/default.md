# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/123s-164ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-230ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-15ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-41ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-19ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-299ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-207ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/60s-281ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-293ms-blue)

```
Unable to read errors
```
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/62s-874ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-224ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-39ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-23ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-299ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-3ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-207ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/90s-573ms-blue)

```
Unable to read errors
```
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/92s-963ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-224ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-38ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-27ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-299ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-2ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/31s-204ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-96ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/60s-484ms-blue)

```
Unable to read errors
```
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/32s-652ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-220ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-14ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-39ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-26ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-10ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-8ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-298ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-2ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-3ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-206ms-blue)

```
Unable to read errors
```
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/0s-347ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-219ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-43ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-5ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-20ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-13ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-299ms-blue)

```
Unable to read errors
```
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-30T14:49:39.145619Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.146381Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.147944Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.202554Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.203077Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.205141Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.206062Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.207258Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.207363Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.249194Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.250413Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.311671Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.313314Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.314455Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.357212Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.358072Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.360291Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.360385Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.494119Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.495355Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.595682Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.696021Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.795380Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.804159Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.805354Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.904673Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:42.015134Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:12.106261Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:12.109023Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:12.206593Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.296528Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.396821Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.498679Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.502050Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.145619Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.146381Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.147944Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.202554Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.203077Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.205141Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.206062Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.207258Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.207363Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.249194Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.250413Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.311671Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.313314Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.314455Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.357212Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.358072Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.360291Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.360385Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.494119Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.495355Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.595682Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.696021Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.795380Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.804159Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.805354Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.904673Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:42.015134Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:12.106261Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:12.109023Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:12.206593Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.296528Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.396821Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.498679Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.502050Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.145619Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.146381Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.147944Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.202554Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.203077Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.205141Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.206062Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.207258Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.207363Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.249194Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.250413Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.311671Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.313314Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.314455Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.357212Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.358072Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.360291Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.360385Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.494119Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.495355Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.595682Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.696021Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.795380Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.804159Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.805354Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.904673Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:42.015134Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:12.106261Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:12.109023Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:12.206593Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.296528Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.396821Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.498679Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:51:42.502050Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.145619Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.146381Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.147944Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.202554Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.203077Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.205141Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.206062Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.207258Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.207363Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.249194Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.250413Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.311671Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.313314Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.314455Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.357212Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.358072Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.360291Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.360385Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.494119Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.495355Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.595682Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.696021Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.795380Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.804159Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.805354Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:50:11.904673Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.145619Z  INFO rudp2plib::thread: Peer started on port 9000.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.146381Z  INFO rudp2plib::thread: Peer started on port 9001.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.147944Z  INFO rudp2plib::thread: Peer started on port 9002.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.202554Z  INFO rudp2plib::thread: Peer started on port 9100.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.203077Z  INFO rudp2plib::thread: Peer started on port 9003.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.205141Z  INFO rudp2plib::thread: Peer started on port 9101.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.206062Z  INFO rudp2plib::thread: Peer started on port 9102.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.207258Z  INFO rudp2plib::thread: Peer started on port 9200.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.207363Z  INFO rudp2plib::thread: Peer started on port 9201.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.249194Z  INFO rudp2plib::thread: Peer started on port 9300.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.250413Z  INFO rudp2plib::thread: Peer started on port 9202.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.311671Z  INFO rudp2plib::thread: Peer started on port 9302.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.313314Z  INFO rudp2plib::thread: Peer started on port 9301.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.314455Z  INFO rudp2plib::thread: Peer started on port 9303.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.357212Z  INFO rudp2plib::thread: Peer started on port 9401.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.358072Z  INFO rudp2plib::thread: Peer started on port 9400.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.360291Z  INFO rudp2plib::thread: Peer started on port 9402.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.360385Z  INFO rudp2plib::thread: Peer started on port 9403.
    at rudp2p/src/thread.rs:93

  2023-10-30T14:49:39.494119Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.495355Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.595682Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at rudp2p/src/thread.rs:126

  2023-10-30T14:49:39.696021Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at rudp2p/src/thread.rs:126


```
</details>

