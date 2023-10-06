# Report

## Feature : Dispatch connections ![Failed](https://img.shields.io/badge/Failed-red)

- Reception of connection and disconnection events ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/17s-409ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-93ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/3s-3ms-blue)
  - the peer "P0" receives (line 11) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/9s-313ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P0 is not connected with P1
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-138ms-blue)
</details>



## Feature : Block peers ![Failed](https://img.shields.io/badge/Failed-red)

- A block peer does not receive any messages until he has unblock ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/17s-821ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-88ms-blue)
  - the peer "P1" connects to "P0" (line 9) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/6s-6ms-blue)
  - the peer "P1" receives (line 10) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/6s-725ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P1 is not connected with P0
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-724ms-blue)
</details>


- A block peer can not send any messages until he has unblock ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/17s-95ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 48) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-84ms-blue)
  - the peer "P1" connects to "P0" (line 53) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-7ms-blue)
  - the peer "P1" receives (line 54) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/12s-4ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P1 is not connected with P0
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/1s-447ms-blue)
</details>



## Feature : Exchange messages ![Failed](https://img.shields.io/badge/Failed-red)

- A peer sends a text to all peers ![Passed](https://img.shields.io/badge/4-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/21s-545ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 4) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-80ms-blue)
  - the peer "P1" connects to "P0" (line 10) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/9s-12ms-blue)
  - the peer "P0" receives (line 11) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/4s-35ms-blue)
  - the peer "P2" connects to "P0" (line 14) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-414ms-blue)
  - the peer "P0" receives (line 15) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/3s-1ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P0 is not connected with P2
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-414ms-blue)
</details>


- A peer sends a file to a peer ![Passed](https://img.shields.io/badge/2-Passed-green) ![Skipped](https://img.shields.io/badge/0-Skipped-yellow) ![Failed](https://img.shields.io/badge/1-Failed-red) ![Duration](https://img.shields.io/badge/18s-127ms-blue)

<details>
<summary>Steps</summary>

  - the following peers are started (line 44) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/5s-87ms-blue)
  - the peer "P1" connects to "P0" (line 50) ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/9s-4ms-blue)
  - the peer "P0" receives (line 51) ![Failed](https://img.shields.io/badge/Failed-red) ![Duration](https://img.shields.io/badge/4s-35ms-blue)

```
Matched: tests/steps/mod.rs:93:1
Step panicked. Captured output: Peer P0 is not connected with P1
```
</details>



<details>
<summary>Hook after</summary>

- ![Passed](https://img.shields.io/badge/Passed-green) ![Duration](https://img.shields.io/badge/0s-414ms-blue)
</details>


---


<details>
<summary>Logs</summary>

```
2023-10-06T15:27:16.764794Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-06T15:27:16.926984Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-06T15:27:17.146512Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-06T15:27:17.398754Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-06T15:27:17.533822Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-06T15:27:17.881481Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-06T15:27:18.235522Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-06T15:27:18.543566Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-06T15:27:18.734919Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-06T15:27:19.103125Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-06T15:27:19.450458Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-06T15:27:19.799283Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-06T15:27:20.007419Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-06T15:27:20.306365Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-06T15:27:20.502686Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-06T15:27:20.626833Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-06T15:27:20.939042Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-06T15:27:21.339770Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-06T15:27:33.316921Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-06T15:27:33.420000Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-06T15:27:33.523003Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-06T15:27:33.627393Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-06T15:27:33.729725Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-06T15:27:33.833150Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-06T15:27:33.936181Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-06T15:27:34.040760Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-06T15:27:34.143192Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-06T15:27:34.247047Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-06T15:27:34.353057Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-06T15:27:34.457732Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-06T15:27:34.558004Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-06T15:27:34.660094Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-06T15:27:16.764794Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-06T15:27:16.926984Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-06T15:27:17.146512Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-06T15:27:17.398754Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-06T15:27:17.533822Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-06T15:27:17.881481Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-06T15:27:18.235522Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-06T15:27:18.543566Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-06T15:27:18.734919Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-06T15:27:19.103125Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-06T15:27:19.450458Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-06T15:27:19.799283Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-06T15:27:20.007419Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-06T15:27:20.306365Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-06T15:27:20.502686Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-06T15:27:20.626833Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-06T15:27:20.939042Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-06T15:27:21.339770Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-06T15:27:33.316921Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-06T15:27:33.420000Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-06T15:27:33.523003Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-06T15:27:33.627393Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-06T15:27:33.729725Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-06T15:27:33.833150Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-06T15:27:33.936181Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-06T15:27:34.040760Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-06T15:27:34.143192Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-06T15:27:34.247047Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-06T15:27:34.353057Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-06T15:27:34.457732Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-06T15:27:34.558004Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-06T15:27:34.660094Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-06T15:27:16.764794Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-06T15:27:16.926984Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-06T15:27:17.146512Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-06T15:27:17.398754Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-06T15:27:17.533822Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-06T15:27:17.881481Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-06T15:27:18.235522Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-06T15:27:18.543566Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-06T15:27:18.734919Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-06T15:27:19.103125Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-06T15:27:19.450458Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-06T15:27:19.799283Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-06T15:27:20.007419Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-06T15:27:20.306365Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-06T15:27:20.502686Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-06T15:27:20.626833Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-06T15:27:20.939042Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-06T15:27:21.339770Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-06T15:27:33.316921Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-06T15:27:33.420000Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-06T15:27:33.523003Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-06T15:27:33.627393Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-06T15:27:33.729725Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-06T15:27:33.833150Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-06T15:27:33.936181Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-06T15:27:34.040760Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-06T15:27:34.143192Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-06T15:27:34.247047Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-06T15:27:34.353057Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-06T15:27:34.457732Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-06T15:27:34.558004Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-06T15:27:34.660094Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-06T15:27:37.768862Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-06T15:27:37.871616Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-06T15:27:37.975170Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-06T15:27:38.078582Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-06T15:27:16.764794Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-06T15:27:16.926984Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-06T15:27:17.146512Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-06T15:27:17.398754Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-06T15:27:17.533822Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-06T15:27:17.881481Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-06T15:27:18.235522Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-06T15:27:18.543566Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-06T15:27:18.734919Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-06T15:27:19.103125Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-06T15:27:19.450458Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-06T15:27:19.799283Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-06T15:27:20.007419Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-06T15:27:20.306365Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-06T15:27:20.502686Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-06T15:27:20.626833Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-06T15:27:20.939042Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-06T15:27:21.339770Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-06T15:27:33.316921Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-06T15:27:33.420000Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-06T15:27:33.523003Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-06T15:27:33.627393Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-06T15:27:33.729725Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-06T15:27:33.833150Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-06T15:27:33.936181Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-06T15:27:34.040760Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-06T15:27:34.143192Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-06T15:27:34.247047Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-06T15:27:34.353057Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-06T15:27:34.457732Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-06T15:27:34.558004Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-06T15:27:34.660094Z  INFO rudp2plib::thread: Peer stopped on port 9401.    
2023-10-06T15:27:37.768862Z  INFO rudp2plib::thread: Peer stopped on port 9300.    
2023-10-06T15:27:37.871616Z  INFO rudp2plib::thread: Peer stopped on port 9302.    
2023-10-06T15:27:37.975170Z  INFO rudp2plib::thread: Peer stopped on port 9301.    
2023-10-06T15:27:38.078582Z  INFO rudp2plib::thread: Peer stopped on port 9303.    
2023-10-06T15:27:16.764794Z  INFO rudp2plib::thread: Peer started on port 9000.    
2023-10-06T15:27:16.926984Z  INFO rudp2plib::thread: Peer started on port 9001.    
2023-10-06T15:27:17.146512Z  INFO rudp2plib::thread: Peer started on port 9002.    
2023-10-06T15:27:17.398754Z  INFO rudp2plib::thread: Peer started on port 9003.    
2023-10-06T15:27:17.533822Z  INFO rudp2plib::thread: Peer started on port 9100.    
2023-10-06T15:27:17.881481Z  INFO rudp2plib::thread: Peer started on port 9101.    
2023-10-06T15:27:18.235522Z  INFO rudp2plib::thread: Peer started on port 9102.    
2023-10-06T15:27:18.543566Z  INFO rudp2plib::thread: Peer started on port 9200.    
2023-10-06T15:27:18.734919Z  INFO rudp2plib::thread: Peer started on port 9201.    
2023-10-06T15:27:19.103125Z  INFO rudp2plib::thread: Peer started on port 9202.    
2023-10-06T15:27:19.450458Z  INFO rudp2plib::thread: Peer started on port 9300.    
2023-10-06T15:27:19.799283Z  INFO rudp2plib::thread: Peer started on port 9301.    
2023-10-06T15:27:20.007419Z  INFO rudp2plib::thread: Peer started on port 9302.    
2023-10-06T15:27:20.306365Z  INFO rudp2plib::thread: Peer started on port 9303.    
2023-10-06T15:27:20.502686Z  INFO rudp2plib::thread: Peer started on port 9400.    
2023-10-06T15:27:20.626833Z  INFO rudp2plib::thread: Peer started on port 9401.    
2023-10-06T15:27:20.939042Z  INFO rudp2plib::thread: Peer started on port 9402.    
2023-10-06T15:27:21.339770Z  INFO rudp2plib::thread: Peer started on port 9403.    
2023-10-06T15:27:33.316921Z  INFO rudp2plib::thread: Peer stopped on port 9202.    
2023-10-06T15:27:33.420000Z  INFO rudp2plib::thread: Peer stopped on port 9200.    
2023-10-06T15:27:33.523003Z  INFO rudp2plib::thread: Peer stopped on port 9201.    
2023-10-06T15:27:33.627393Z  INFO rudp2plib::thread: Peer stopped on port 9001.    
2023-10-06T15:27:33.729725Z  INFO rudp2plib::thread: Peer stopped on port 9002.    
2023-10-06T15:27:33.833150Z  INFO rudp2plib::thread: Peer stopped on port 9003.    
2023-10-06T15:27:33.936181Z  INFO rudp2plib::thread: Peer stopped on port 9000.    
2023-10-06T15:27:34.040760Z  INFO rudp2plib::thread: Peer stopped on port 9101.    
2023-10-06T15:27:34.143192Z  INFO rudp2plib::thread: Peer stopped on port 9100.    
2023-10-06T15:27:34.247047Z  INFO rudp2plib::thread: Peer stopped on port 9102.    
2023-10-06T15:27:34.353057Z  INFO rudp2plib::thread: Peer stopped on port 9403.    
2023-10-06T15:27:34.457732Z  INFO rudp2plib::thread: Peer stopped on port 9402.    
2023-10-06T15:27:34.558004Z  INFO rudp2plib::thread: Peer stopped on port 9400.    
2023-10-06T15:27:34.660094Z  INFO rudp2plib::thread: Peer stopped on port 9401.    

```
</details>

