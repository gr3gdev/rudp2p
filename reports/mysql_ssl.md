# Report

## Feature : Dispatch connections ![Failed](https://img.shields.io/badge/Failed-red)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/18s-227ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-174ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-9ms-blue)
  - the peer "P0" receives (line 11) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/7s-43ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P0 is not connected with P1
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-417ms-blue)
</details>



## Feature : Block peers ![Failed](https://img.shields.io/badge/Failed-red)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/17s-598ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-176ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-3ms-blue)
  - the peer "P1" receives (line 10) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/9s-418ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P1 is not connected with P0
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-42ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/17s-910ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-173ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/9s-7ms-blue)
  - the peer "P1" receives (line 54) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/3s-729ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P1 is not connected with P0
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-729ms-blue)
</details>



## Feature : Exchange messages ![Failed](https://img.shields.io/badge/Failed-red)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/4-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/21s-641ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-173ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/12s-6ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-456ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-2ms-blue)
  - the peer "P0" receives (line 15) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/3s-2ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P0 is not connected with P2
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-414ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/17s-178ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-170ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-3ms-blue)
  - the peer "P0" receives (line 51) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/12s-5ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P0 is not connected with P1
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-456ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
2023-10-07T18:19:26.640836Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-07T18:19:26.929412Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-07T18:19:27.128950Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-07T18:19:27.505523Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-07T18:19:27.732823Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-07T18:19:27.915825Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-07T18:19:28.207530Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-07T18:19:28.376876Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-07T18:19:28.707344Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-07T18:19:29.133186Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-07T18:19:29.346899Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-07T18:19:29.472044Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-07T18:19:29.770350Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-07T18:19:30.340636Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-07T18:19:30.716870Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-07T18:19:30.999127Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-07T18:19:31.149584Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-07T18:19:31.498898Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-07T18:19:43.522084Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-07T18:19:43.618324Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-07T18:19:43.722172Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-07T18:19:43.833454Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-07T18:19:43.929753Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-07T18:19:44.034810Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-07T18:19:44.137985Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-07T18:19:44.244406Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-07T18:19:44.348672Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-07T18:19:44.452896Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-07T18:19:44.556057Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-07T18:19:44.658747Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-07T18:19:44.763512Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-07T18:19:44.870211Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-07T18:19:47.974506Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-07T18:19:48.080759Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-07T18:19:48.183168Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-07T18:19:48.285679Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-07T18:19:26.640836Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-07T18:19:26.929412Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-07T18:19:27.128950Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-07T18:19:27.505523Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-07T18:19:27.732823Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-07T18:19:27.915825Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-07T18:19:28.207530Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-07T18:19:28.376876Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-07T18:19:28.707344Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-07T18:19:29.133186Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-07T18:19:29.346899Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-07T18:19:29.472044Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-07T18:19:29.770350Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-07T18:19:30.340636Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-07T18:19:30.716870Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-07T18:19:30.999127Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-07T18:19:31.149584Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-07T18:19:31.498898Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-07T18:19:43.522084Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-07T18:19:43.618324Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-07T18:19:43.722172Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-07T18:19:43.833454Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-07T18:19:43.929753Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-07T18:19:44.034810Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-07T18:19:44.137985Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-07T18:19:44.244406Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-07T18:19:44.348672Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-07T18:19:44.452896Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-07T18:19:44.556057Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-07T18:19:44.658747Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-07T18:19:44.763512Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-07T18:19:44.870211Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-07T18:19:26.640836Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-07T18:19:26.929412Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-07T18:19:27.128950Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-07T18:19:27.505523Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-07T18:19:27.732823Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-07T18:19:27.915825Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-07T18:19:28.207530Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-07T18:19:28.376876Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-07T18:19:28.707344Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-07T18:19:29.133186Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-07T18:19:29.346899Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-07T18:19:29.472044Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-07T18:19:29.770350Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-07T18:19:30.340636Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-07T18:19:30.716870Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-07T18:19:30.999127Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-07T18:19:31.149584Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-07T18:19:31.498898Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-07T18:19:43.522084Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-07T18:19:43.618324Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-07T18:19:43.722172Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-07T18:19:43.833454Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-07T18:19:43.929753Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-07T18:19:44.034810Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-07T18:19:44.137985Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-07T18:19:44.244406Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-07T18:19:44.348672Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-07T18:19:44.452896Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-07T18:19:44.556057Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-07T18:19:44.658747Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-07T18:19:44.763512Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-07T18:19:44.870211Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-07T18:19:26.640836Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-07T18:19:26.929412Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-07T18:19:27.128950Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-07T18:19:27.505523Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-07T18:19:27.732823Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-07T18:19:27.915825Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-07T18:19:28.207530Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-07T18:19:28.376876Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-07T18:19:28.707344Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-07T18:19:29.133186Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-07T18:19:29.346899Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-07T18:19:29.472044Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-07T18:19:29.770350Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-07T18:19:30.340636Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-07T18:19:30.716870Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-07T18:19:30.999127Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-07T18:19:31.149584Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-07T18:19:31.498898Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-07T18:19:43.522084Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-07T18:19:43.618324Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-07T18:19:43.722172Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-07T18:19:43.833454Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-07T18:19:43.929753Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-07T18:19:44.034810Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-07T18:19:44.137985Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-07T18:19:44.244406Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-07T18:19:44.348672Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-07T18:19:44.452896Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-07T18:19:44.556057Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-07T18:19:44.658747Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-07T18:19:44.763512Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-07T18:19:44.870211Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-07T18:19:47.974506Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-07T18:19:48.080759Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-07T18:19:48.183168Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-07T18:19:48.285679Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-07T18:19:26.640836Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-07T18:19:26.929412Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-07T18:19:27.128950Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-07T18:19:27.505523Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-07T18:19:27.732823Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-07T18:19:27.915825Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-07T18:19:28.207530Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-07T18:19:28.376876Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-07T18:19:28.707344Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-07T18:19:29.133186Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-07T18:19:29.346899Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-07T18:19:29.472044Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-07T18:19:29.770350Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-07T18:19:30.340636Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-07T18:19:30.716870Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-07T18:19:30.999127Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-07T18:19:31.149584Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-07T18:19:31.498898Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-07T18:19:43.522084Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-07T18:19:43.618324Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-07T18:19:43.722172Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-07T18:19:43.833454Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-07T18:19:43.929753Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-07T18:19:44.034810Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-07T18:19:44.137985Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-07T18:19:44.244406Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-07T18:19:44.348672Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-07T18:19:44.452896Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-07T18:19:44.556057Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-07T18:19:44.658747Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-07T18:19:44.763512Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-07T18:19:44.870211Z  INFO rudp2plib::thread: Peer stopped on port 9000.    

```
</details>

