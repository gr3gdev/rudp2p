# Report

## Feature : Dispatch connections ![Failed](https://img.shields.io/badge/Failed-red)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/3-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/17s-81ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-37ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-11ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-721ms-blue)
  - the peer "P1" receives (line 14) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/3s-310ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P1 is not connected with P0
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-413ms-blue)
</details>



## Feature : Block peers ![Failed](https://img.shields.io/badge/Failed-red)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/13s-458ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-31ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-12ms-blue)
  - the peer "P1" receives (line 10) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/9s-414ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P1 is not connected with P0
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-619ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/16s-767ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-34ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-11ms-blue)
  - the peer "P1" receives (line 54) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/6s-722ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P1 is not connected with P0
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-722ms-blue)
</details>



## Feature : Exchange messages ![Failed](https://img.shields.io/badge/Failed-red)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/13s-45ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-35ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-7ms-blue)
  - the peer "P0" receives (line 11) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/6s-2ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P0 is not connected with P1
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-30ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/7-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/20s-504ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-35ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/9s-8ms-blue)
  - the peer "P0" receives (line 51) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-29ms-blue)
  - the peer "P2" connects to "P0" (line 54) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-413ms-blue)
  - the peer "P0" receives (line 55) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P3" connects to "P0" (line 58) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-1ms-blue)
  - the peer "P0" receives (line 59) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-11ms-blue)
  - the peer "P2" receives (line 62) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/3s-1ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P2 is not connected with P0
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-412ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
2023-10-07T18:03:12.901364Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-07T18:03:13.206729Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-07T18:03:13.286126Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-07T18:03:13.706201Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-07T18:03:13.852464Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-07T18:03:14.005965Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-07T18:03:14.502535Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-07T18:03:14.612294Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-07T18:03:15.108497Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-07T18:03:15.420343Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-07T18:03:15.584137Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-07T18:03:15.840371Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-07T18:03:15.935875Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-07T18:03:16.220526Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-07T18:03:16.384263Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-07T18:03:16.492371Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-07T18:03:16.672254Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-07T18:03:16.820966Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-07T18:03:25.841879Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-07T18:03:25.943652Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-07T18:03:26.046796Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-07T18:03:26.149443Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-07T18:03:26.252679Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-07T18:03:26.354145Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-07T18:03:26.456553Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-07T18:03:29.564525Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-07T18:03:29.665188Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-07T18:03:29.767302Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-07T18:03:29.872214Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-07T18:03:29.977009Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-07T18:03:30.079446Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-07T18:03:30.181398Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-07T18:03:12.901364Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-07T18:03:13.206729Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-07T18:03:13.286126Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-07T18:03:13.706201Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-07T18:03:13.852464Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-07T18:03:14.005965Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-07T18:03:14.502535Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-07T18:03:14.612294Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-07T18:03:15.108497Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-07T18:03:15.420343Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-07T18:03:15.584137Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-07T18:03:15.840371Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-07T18:03:15.935875Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-07T18:03:16.220526Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-07T18:03:16.384263Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-07T18:03:16.492371Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-07T18:03:16.672254Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-07T18:03:16.820966Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-07T18:03:25.841879Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-07T18:03:25.943652Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-07T18:03:26.046796Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-07T18:03:26.149443Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-07T18:03:26.252679Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-07T18:03:26.354145Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-07T18:03:26.456553Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-07T18:03:29.564525Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-07T18:03:29.665188Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-07T18:03:29.767302Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-07T18:03:29.872214Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-07T18:03:29.977009Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-07T18:03:30.079446Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-07T18:03:30.181398Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-07T18:03:12.901364Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-07T18:03:13.206729Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-07T18:03:13.286126Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-07T18:03:13.706201Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-07T18:03:13.852464Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-07T18:03:14.005965Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-07T18:03:14.502535Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-07T18:03:14.612294Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-07T18:03:15.108497Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-07T18:03:15.420343Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-07T18:03:15.584137Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-07T18:03:15.840371Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-07T18:03:15.935875Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-07T18:03:16.220526Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-07T18:03:16.384263Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-07T18:03:16.492371Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-07T18:03:16.672254Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-07T18:03:16.820966Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-07T18:03:25.841879Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-07T18:03:25.943652Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-07T18:03:26.046796Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-07T18:03:26.149443Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-07T18:03:26.252679Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-07T18:03:26.354145Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-07T18:03:26.456553Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-07T18:03:29.564525Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-07T18:03:29.665188Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-07T18:03:29.767302Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-07T18:03:29.872214Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-07T18:03:29.977009Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-07T18:03:30.079446Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-07T18:03:30.181398Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-07T18:03:12.901364Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-07T18:03:13.206729Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-07T18:03:13.286126Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-07T18:03:13.706201Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-07T18:03:13.852464Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-07T18:03:14.005965Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-07T18:03:14.502535Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-07T18:03:14.612294Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-07T18:03:15.108497Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-07T18:03:15.420343Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-07T18:03:15.584137Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-07T18:03:15.840371Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-07T18:03:15.935875Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-07T18:03:16.220526Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-07T18:03:16.384263Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-07T18:03:16.492371Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-07T18:03:16.672254Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-07T18:03:16.820966Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-07T18:03:25.841879Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-07T18:03:25.943652Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-07T18:03:26.046796Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-07T18:03:26.149443Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-07T18:03:26.252679Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-07T18:03:26.354145Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-07T18:03:26.456553Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-07T18:03:29.564525Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-07T18:03:29.665188Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-07T18:03:29.767302Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-07T18:03:29.872214Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-07T18:03:29.977009Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-07T18:03:30.079446Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-07T18:03:30.181398Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-07T18:03:12.901364Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-07T18:03:13.206729Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-07T18:03:13.286126Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-07T18:03:13.706201Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-07T18:03:13.852464Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-07T18:03:14.005965Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-07T18:03:14.502535Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-07T18:03:14.612294Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-07T18:03:15.108497Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-07T18:03:15.420343Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-07T18:03:15.584137Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-07T18:03:15.840371Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-07T18:03:15.935875Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-07T18:03:16.220526Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-07T18:03:16.384263Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-07T18:03:16.492371Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-07T18:03:16.672254Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-07T18:03:16.820966Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-07T18:03:25.841879Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-07T18:03:25.943652Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-07T18:03:26.046796Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-07T18:03:26.149443Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-07T18:03:26.252679Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-07T18:03:26.354145Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-07T18:03:26.456553Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-07T18:03:29.564525Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-07T18:03:29.665188Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-07T18:03:29.767302Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-07T18:03:29.872214Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-07T18:03:29.977009Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-07T18:03:30.079446Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-07T18:03:30.181398Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-07T18:03:33.309265Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-07T18:03:33.406182Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-07T18:03:33.508287Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-07T18:03:33.609426Z  INFO rudp2plib::thread: Peer stopped on port 9401.    

```
</details>

