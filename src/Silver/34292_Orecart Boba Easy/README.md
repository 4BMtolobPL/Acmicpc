# [Silver IV] Orecart Boba Easy - 34292 

[문제 링크](https://www.acmicpc.net/problem/34292) 

### 성능 요약

메모리: 16284 KB, 시간: 8 ms

### 분류

수학, 구현, 그리디 알고리즘, 시뮬레이션, 물리학

### 제출 일자

2026년 03월 11일 14:35:12

### 문제 설명

<p><strong>The only difference between this version and the hard version is that in this version, the maximum speed $v$ is provided as input.</strong></p>

<p>Eugin and Kelly are celebrating E-Days at the Colorado School of Mines by participating in the annual Orecart Pull. This is an event where everyone walks the Orecart down Colfax Avenue all the way to downtown Denver. Eugin and Kelly can go faster than the group, so they have decided to break off and grab boba from every stop along the route!</p>

<p>The walk is $l$ meters long, and the Orecart moves at a constant speed of $1$ meter per second. Eugin and Kelly can run at a speed of at most $v$ meters per second, but they are not allowed to move ahead of the Orecart at any time.</p>

<p>There are $n$ boba stops along the route, the $i$-th of which is located at distance $d_i$ along the route. Additionally, at the $i$-th boba stop, they must wait at least $w_i$ seconds to receive their boba before continuing.</p>

<p>Eugin and Kelly need to visit all $n$ boba stops while ensuring that they reach the end at exactly the same time as the Orecart. Can they accomplish this?</p>

### 입력 

 <p>The input consists of multiple lines:</p>

<ul>
	<li>The first line contains two integers $n$ and $l$ $(1 \leq n \leq 10^5, 2 \leq l \leq 10^8)$---the number of boba stops and the length of the route.</li>
	<li>The second line contains $n$ integers $d_1, d_2, \dots, d_n$ $(0 < d_1 < d_2 < \dots < d_n < l)$---the distances at which each of the boba stops are located.</li>
	<li>The third line contains $n$ integers $w_1, w_2, \dots, w_n$ $(0 \leq w_i \leq 10^8)$---the waiting times at each boba stop.</li>
	<li>The fourth line contains a single integer $v$ $(1 \leq v \leq 10^8)$---the maximum speed at which Eugin and Kelly can run.</li>
</ul>

### 출력 

 <p>Print <code>YES</code> if it is possible for Eugin and Kelly to visit every boba stop and reach the end at exactly the same time as the Orecart. Otherwise, print <code>NO</code>.</p>

