# Report

## Feature : Dispatch connections ![Failed](https://img.shields.io/badge/Failed-red)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/56s-885ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-44ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/20s-12ms-blue)
  - the peer "P0" receives (line 11) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/30s-828ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P0 is not connected with P1
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-29ms-blue)
</details>



## Feature : Block peers ![Failed](https://img.shields.io/badge/Failed-red)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/57s-603ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-46ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/40s-11ms-blue)
  - the peer "P1" receives (line 10) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/11s-545ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P1 is not connected with P0
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-309ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/57s-293ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-39ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/10s-14ms-blue)
  - the peer "P1" receives (line 54) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/41s-240ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P1 is not connected with P0
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-617ms-blue)
</details>



## Feature : Exchange messages ![Failed](https://img.shields.io/badge/Failed-red)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/56s-468ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-40ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-9ms-blue)
  - the peer "P0" receives (line 11) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/50s-418ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P0 is not connected with P1
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-440ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/56s-55ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-41ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/30s-11ms-blue)
  - the peer "P0" receives (line 51) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/20s-2ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P0 is not connected with P1
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-852ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
2023-10-08T14:50:09.758448Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-08T14:50:09.940984Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-08T14:50:10.495886Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-08T14:50:10.621142Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-08T14:50:10.778770Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-08T14:50:11.158131Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-08T14:50:11.303738Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-08T14:50:11.540087Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-08T14:50:12.078453Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-08T14:50:12.279296Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-08T14:50:12.376631Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-08T14:50:12.540630Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-08T14:50:12.779052Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-08T14:50:13.414609Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-08T14:50:13.720879Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-08T14:50:13.799659Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-08T14:50:15.005611Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-08T14:50:15.566956Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-08T14:51:05.584804Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-08T14:51:05.688255Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-08T14:51:05.791009Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-08T14:51:05.893427Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-08T14:51:05.997073Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-08T14:51:06.101635Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-08T14:51:06.201989Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-08T14:51:06.305279Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-08T14:51:06.409088Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-08T14:51:06.511779Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-08T14:51:06.614842Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-08T14:51:06.720410Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-08T14:51:06.820459Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-08T14:51:06.922342Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-08T14:51:07.025224Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-08T14:51:07.128349Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-08T14:51:07.230771Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-08T14:51:07.333504Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-08T14:50:09.758448Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-08T14:50:09.940984Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-08T14:50:10.495886Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-08T14:50:10.621142Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-08T14:50:10.778770Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-08T14:50:11.158131Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-08T14:50:11.303738Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-08T14:50:11.540087Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-08T14:50:12.078453Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-08T14:50:12.279296Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-08T14:50:12.376631Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-08T14:50:12.540630Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-08T14:50:12.779052Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-08T14:50:13.414609Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-08T14:50:13.720879Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-08T14:50:13.799659Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-08T14:50:15.005611Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-08T14:50:15.566956Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-08T14:51:05.584804Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-08T14:51:05.688255Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-08T14:51:05.791009Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-08T14:51:05.893427Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-08T14:51:05.997073Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-08T14:51:06.101635Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-08T14:51:06.201989Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-08T14:51:06.305279Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-08T14:51:06.409088Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-08T14:51:06.511779Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-08T14:51:06.614842Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-08T14:51:06.720410Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-08T14:51:06.820459Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-08T14:51:06.922342Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-08T14:51:07.025224Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-08T14:51:07.128349Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-08T14:51:07.230771Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-08T14:51:07.333504Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-08T14:50:09.758448Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-08T14:50:09.940984Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-08T14:50:10.495886Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-08T14:50:10.621142Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-08T14:50:10.778770Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-08T14:50:11.158131Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-08T14:50:11.303738Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-08T14:50:11.540087Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-08T14:50:12.078453Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-08T14:50:12.279296Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-08T14:50:12.376631Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-08T14:50:12.540630Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-08T14:50:12.779052Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-08T14:50:13.414609Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-08T14:50:13.720879Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-08T14:50:13.799659Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-08T14:50:15.005611Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-08T14:50:15.566956Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-08T14:51:05.584804Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-08T14:51:05.688255Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-08T14:51:05.791009Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-08T14:51:05.893427Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-08T14:51:05.997073Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-08T14:51:06.101635Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-08T14:51:06.201989Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-08T14:51:06.305279Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-08T14:51:06.409088Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-08T14:51:06.511779Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-08T14:51:06.614842Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-08T14:51:06.720410Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-08T14:51:06.820459Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-08T14:51:06.922342Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-08T14:51:07.025224Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-08T14:51:07.128349Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-08T14:51:07.230771Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-08T14:51:07.333504Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-08T14:50:09.758448Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-08T14:50:09.940984Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-08T14:50:10.495886Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-08T14:50:10.621142Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-08T14:50:10.778770Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-08T14:50:11.158131Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-08T14:50:11.303738Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-08T14:50:11.540087Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-08T14:50:12.078453Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-08T14:50:12.279296Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-08T14:50:12.376631Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-08T14:50:12.540630Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-08T14:50:12.779052Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-08T14:50:13.414609Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-08T14:50:13.720879Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-08T14:50:13.799659Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-08T14:50:15.005611Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-08T14:50:15.566956Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-08T14:51:05.584804Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-08T14:51:05.688255Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-08T14:51:05.791009Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-08T14:51:05.893427Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-08T14:51:05.997073Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-08T14:51:06.101635Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-08T14:51:06.201989Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-08T14:51:06.305279Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-08T14:51:06.409088Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-08T14:51:06.511779Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-08T14:51:06.614842Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-08T14:51:06.720410Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-08T14:51:06.820459Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-08T14:51:06.922342Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-08T14:51:07.025224Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-08T14:51:07.128349Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-08T14:51:07.230771Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-08T14:51:07.333504Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-08T14:50:09.758448Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-08T14:50:09.940984Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-08T14:50:10.495886Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-08T14:50:10.621142Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-08T14:50:10.778770Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-08T14:50:11.158131Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-08T14:50:11.303738Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-08T14:50:11.540087Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-08T14:50:12.078453Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-08T14:50:12.279296Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-08T14:50:12.376631Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-08T14:50:12.540630Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-08T14:50:12.779052Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-08T14:50:13.414609Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-08T14:50:13.720879Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-08T14:50:13.799659Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-08T14:50:15.005611Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-08T14:50:15.566956Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-08T14:51:05.584804Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-08T14:51:05.688255Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-08T14:51:05.791009Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-08T14:51:05.893427Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-08T14:51:05.997073Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-08T14:51:06.101635Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-08T14:51:06.201989Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-08T14:51:06.305279Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-08T14:51:06.409088Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-08T14:51:06.511779Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-08T14:51:06.614842Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-08T14:51:06.720410Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-08T14:51:06.820459Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-08T14:51:06.922342Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-08T14:51:07.025224Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-08T14:51:07.128349Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-08T14:51:07.230771Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-08T14:51:07.333504Z  INFO rudp2plib::thread: Peer stopped on port 9100.    

```
</details>

