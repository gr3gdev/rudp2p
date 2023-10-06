# Report

## Feature : Dispatch connections ![Passed](https://img.shields.io/badge/Passed-green)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/18-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-923ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-474ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P0" receives (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P1" receives (line 21) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 24) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P3" connects to "P0" (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 29) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-404ms-blue)
  - the peer "P2" receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-2ms-blue)
  - the peer "P3" receives (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
  - the peer "P2" disconnects (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 47) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" receives (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-602ms-blue)
  - the peer "P2" receives (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-402ms-blue)
</details>



## Feature : Block peers ![Passed](https://img.shields.io/badge/Passed-green)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-619ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-472ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P1" receives (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 13) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 16) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" receives (line 17) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P0" receives (line 20) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 23) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" blocks the peer "P2" (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 28) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 31) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
  - the peer "P0" receives (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-2ms-blue)
  - the peer "P2" does not receives (line 35) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-404ms-blue)
  - the peer "P1" unblocks the peer "P2" (line 38) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 42) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" receives (line 43) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-302ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/17-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/6s-317ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-472ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 57) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 60) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" receives (line 61) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 64) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" blocks the peer "P1" (line 71) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "I am a peer" to "all" (line 75) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-404ms-blue)
  - the peer "P0" receives (line 76) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" does not receives (line 79) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/2s-2ms-blue)
  - the peer "P2" unblocks the peer "P1" (line 82) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
  - the peer "P1" receives (line 83) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" sends "Hello" to "all" (line 86) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" receives (line 87) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-602ms-blue)
</details>



## Feature : Exchange messages ![Passed](https://img.shields.io/badge/Passed-green)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/13-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/5s-907ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-471ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 15) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-6ms-blue)
  - the peer "P3" connects to "P0" (line 18) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 19) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-4ms-blue)
  - the peer "P2" receives (line 22) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P3" receives (line 27) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P1" sends "Hello all" to "all" (line 32) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 33) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-404ms-blue)
  - the peer "P2" receives (line 36) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-2ms-blue)
  - the peer "P3" receives (line 39) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-2ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/11-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/0-Failed-red) ![Duration](https://img.shields.io/badge/3s-497ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-467ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P2" receives (line 62) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P3" receives (line 67) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P2" sends "file:/tests/test.txt" to "P1" (line 72) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P1" receives (line 73) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-403ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
  2023-10-06T13:59:07.343366Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-06T13:59:07.439361Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-06T13:59:07.557597Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-06T13:59:07.705378Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-06T13:59:07.859034Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-06T13:59:08.099012Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-06T13:59:08.492075Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-06T13:59:08.602965Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-06T13:59:08.842996Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-06T13:59:08.910720Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-06T13:59:08.993226Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-06T13:59:09.138897Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-06T13:59:09.372928Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-06T13:59:09.583511Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-06T13:59:09.688442Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-06T13:59:09.851190Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-06T13:59:10.292914Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-06T13:59:10.707010Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-06T13:59:10.737259Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-06T13:59:10.837888Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-06T13:59:10.938532Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-06T13:59:11.038616Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-06T13:59:13.145176Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-06T13:59:13.245616Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-06T13:59:13.346002Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-06T13:59:13.446361Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-06T13:59:13.553837Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-06T13:59:13.654146Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-06T13:59:13.754625Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-06T13:59:13.855614Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-06T13:59:13.955711Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-06T13:59:14.056251Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-06T13:59:14.158410Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-06T13:59:14.258610Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-06T13:59:14.359179Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-06T13:59:14.459243Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-06T13:59:07.343366Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-06T13:59:07.439361Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-06T13:59:07.557597Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-06T13:59:07.705378Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-06T13:59:07.859034Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-06T13:59:08.099012Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-06T13:59:08.492075Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-06T13:59:08.602965Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-06T13:59:08.842996Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-06T13:59:08.910720Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-06T13:59:08.993226Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-06T13:59:09.138897Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-06T13:59:09.372928Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-06T13:59:09.583511Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-06T13:59:09.688442Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-06T13:59:09.851190Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-06T13:59:10.292914Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-06T13:59:10.707010Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-06T13:59:10.737259Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-06T13:59:10.837888Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-06T13:59:10.938532Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-06T13:59:11.038616Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-06T13:59:13.145176Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-06T13:59:13.245616Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-06T13:59:13.346002Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-06T13:59:13.446361Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-06T13:59:13.553837Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-06T13:59:13.654146Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-06T13:59:13.754625Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-06T13:59:13.855614Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-06T13:59:13.955711Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-06T13:59:14.056251Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-06T13:59:14.158410Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-06T13:59:14.258610Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-06T13:59:14.359179Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-06T13:59:14.459243Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-06T13:59:07.343366Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-06T13:59:07.439361Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-06T13:59:07.557597Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-06T13:59:07.705378Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-06T13:59:07.859034Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-06T13:59:08.099012Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-06T13:59:08.492075Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-06T13:59:08.602965Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-06T13:59:08.842996Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-06T13:59:08.910720Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-06T13:59:08.993226Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-06T13:59:09.138897Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-06T13:59:09.372928Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-06T13:59:09.583511Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-06T13:59:09.688442Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-06T13:59:09.851190Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-06T13:59:10.292914Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-06T13:59:10.707010Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-06T13:59:10.737259Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-06T13:59:10.837888Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-06T13:59:10.938532Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-06T13:59:11.038616Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-06T13:59:13.145176Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-06T13:59:13.245616Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-06T13:59:13.346002Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-06T13:59:13.446361Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-06T13:59:13.553837Z  INFO rudp2plib::thread: Peer stopped on port 9200.
    at src/thread.rs:136

  2023-10-06T13:59:13.654146Z  INFO rudp2plib::thread: Peer stopped on port 9201.
    at src/thread.rs:136

  2023-10-06T13:59:13.754625Z  INFO rudp2plib::thread: Peer stopped on port 9202.
    at src/thread.rs:136

  2023-10-06T13:59:13.855614Z  INFO rudp2plib::thread: Peer stopped on port 9100.
    at src/thread.rs:136

  2023-10-06T13:59:13.955711Z  INFO rudp2plib::thread: Peer stopped on port 9102.
    at src/thread.rs:136

  2023-10-06T13:59:14.056251Z  INFO rudp2plib::thread: Peer stopped on port 9101.
    at src/thread.rs:136

  2023-10-06T13:59:14.158410Z  INFO rudp2plib::thread: Peer stopped on port 9003.
    at src/thread.rs:136

  2023-10-06T13:59:14.258610Z  INFO rudp2plib::thread: Peer stopped on port 9002.
    at src/thread.rs:136

  2023-10-06T13:59:14.359179Z  INFO rudp2plib::thread: Peer stopped on port 9000.
    at src/thread.rs:136

  2023-10-06T13:59:14.459243Z  INFO rudp2plib::thread: Peer stopped on port 9001.
    at src/thread.rs:136

  2023-10-06T13:59:07.343366Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-06T13:59:07.439361Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-06T13:59:07.557597Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-06T13:59:07.705378Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-06T13:59:07.859034Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-06T13:59:08.099012Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-06T13:59:08.492075Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-06T13:59:08.602965Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-06T13:59:08.842996Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-06T13:59:08.910720Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-06T13:59:08.993226Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-06T13:59:09.138897Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-06T13:59:09.372928Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-06T13:59:09.583511Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-06T13:59:09.688442Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-06T13:59:09.851190Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-06T13:59:10.292914Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-06T13:59:10.707010Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-06T13:59:10.737259Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-06T13:59:10.837888Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-06T13:59:10.938532Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-06T13:59:11.038616Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136

  2023-10-06T13:59:13.145176Z  INFO rudp2plib::thread: Peer stopped on port 9301.
    at src/thread.rs:136

  2023-10-06T13:59:13.245616Z  INFO rudp2plib::thread: Peer stopped on port 9302.
    at src/thread.rs:136

  2023-10-06T13:59:13.346002Z  INFO rudp2plib::thread: Peer stopped on port 9303.
    at src/thread.rs:136

  2023-10-06T13:59:13.446361Z  INFO rudp2plib::thread: Peer stopped on port 9300.
    at src/thread.rs:136

  2023-10-06T13:59:07.343366Z  INFO rudp2plib::thread: Peer started on port 9000.
    at src/thread.rs:101

  2023-10-06T13:59:07.439361Z  INFO rudp2plib::thread: Peer started on port 9001.
    at src/thread.rs:101

  2023-10-06T13:59:07.557597Z  INFO rudp2plib::thread: Peer started on port 9002.
    at src/thread.rs:101

  2023-10-06T13:59:07.705378Z  INFO rudp2plib::thread: Peer started on port 9003.
    at src/thread.rs:101

  2023-10-06T13:59:07.859034Z  INFO rudp2plib::thread: Peer started on port 9100.
    at src/thread.rs:101

  2023-10-06T13:59:08.099012Z  INFO rudp2plib::thread: Peer started on port 9101.
    at src/thread.rs:101

  2023-10-06T13:59:08.492075Z  INFO rudp2plib::thread: Peer started on port 9102.
    at src/thread.rs:101

  2023-10-06T13:59:08.602965Z  INFO rudp2plib::thread: Peer started on port 9200.
    at src/thread.rs:101

  2023-10-06T13:59:08.842996Z  INFO rudp2plib::thread: Peer started on port 9201.
    at src/thread.rs:101

  2023-10-06T13:59:08.910720Z  INFO rudp2plib::thread: Peer started on port 9202.
    at src/thread.rs:101

  2023-10-06T13:59:08.993226Z  INFO rudp2plib::thread: Peer started on port 9300.
    at src/thread.rs:101

  2023-10-06T13:59:09.138897Z  INFO rudp2plib::thread: Peer started on port 9301.
    at src/thread.rs:101

  2023-10-06T13:59:09.372928Z  INFO rudp2plib::thread: Peer started on port 9302.
    at src/thread.rs:101

  2023-10-06T13:59:09.583511Z  INFO rudp2plib::thread: Peer started on port 9303.
    at src/thread.rs:101

  2023-10-06T13:59:09.688442Z  INFO rudp2plib::thread: Peer started on port 9400.
    at src/thread.rs:101

  2023-10-06T13:59:09.851190Z  INFO rudp2plib::thread: Peer started on port 9401.
    at src/thread.rs:101

  2023-10-06T13:59:10.292914Z  INFO rudp2plib::thread: Peer started on port 9402.
    at src/thread.rs:101

  2023-10-06T13:59:10.707010Z  INFO rudp2plib::thread: Peer started on port 9403.
    at src/thread.rs:101

  2023-10-06T13:59:10.737259Z  INFO rudp2plib::thread: Peer stopped on port 9403.
    at src/thread.rs:136

  2023-10-06T13:59:10.837888Z  INFO rudp2plib::thread: Peer stopped on port 9401.
    at src/thread.rs:136

  2023-10-06T13:59:10.938532Z  INFO rudp2plib::thread: Peer stopped on port 9402.
    at src/thread.rs:136

  2023-10-06T13:59:11.038616Z  INFO rudp2plib::thread: Peer stopped on port 9400.
    at src/thread.rs:136


```
</details>

