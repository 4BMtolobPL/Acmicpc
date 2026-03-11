# [Gold III] Teaching Assistant (Small) - 14348 

[문제 링크](https://www.acmicpc.net/problem/14348) 

### 성능 요약

메모리: 13200 KB, 시간: 0 ms

### 분류

자료 구조, 다이나믹 프로그래밍, 그리디 알고리즘, 스택

### 제출 일자

2026년 03월 11일 14:35:12

### 문제 설명

<p>You are taking a programming course which is graded using problem sets of different types. The course goes for a positive even number of days. You start the course with no problem sets. On each day of the course, you must do exactly one of the following:</p>

<ul>
	<li>Request a "Coding" problem set.</li>
	<li>Request a "Jamming" problem set.</li>
	<li>Submit a problem set for grading. You must have at least one problem set to choose this option. If you have multiple problem sets, you must submit the one among those that was requested most recently, regardless of its type.</li>
</ul>

<p>All problem sets are different. There is no requirement on how many sets of each type must be submitted. Once you submit a set, you no longer have that set. Any problem sets that you have not submitted before the end of the course get you no points.</p>

<p>The problem sets are requested from and submitted to an artificially-intelligent teaching assistant. Strangely, the assistant has different moods — on each day it is in the mood for either "Coding" or "Jamming".</p>

<ul>
	<li>When you request a problem set:
	<ul>
		<li>If the requested topic matches the assistant's mood, it assigns a problem set worth a maximum of 10 points.</li>
		<li>If the requested topic does not match its mood, it assigns a problem set worth a maximum of 5 points.</li>
	</ul>
	</li>
	<li>When you submit a problem set:
	<ul>
		<li>If the topic of the submitted set matches the assistant's mood that day, it gives you the maximum number of points for that set.</li>
		<li>If the topic of the submitted set does not match its mood that day, it gives you 5 points fewer than the maximum number of points for that set.</li>
	</ul>
	</li>
</ul>

<p>For example:</p>

<ul>
	<li>If you request a "Coding" problem set on a day in which the assistant is in a "Coding" mood, and submit it on a day in which it is in a "Jamming" mood, you will earn 5 points: the problem set is worth a maximum of 10, but the assistant gives 5 points fewer than that.</li>
	<li>If you request a "Jamming" problem set on a day in which the assistant is in a "Coding" mood, and submit it on a day in which it is in a "Jamming" mood, you will earn 5 points: the set is worth a maximum of 5, and the assistant gives you the maximum number of points.</li>
</ul>

<p>Thanks to some help from a senior colleague who understands the assistant very well, you know what sort of mood the assistant will be in on each day of the course. What is the maximum total score that you will be able to obtain?</p>

### 입력 

 <p>The first line of the input gives the number of test cases, T; T test cases follow. Each test case consists of one line with a string S of <code>C</code> and/or <code>J</code> characters. The i-th character of Sdenotes the assistant's mood on the i-th day of the course. If it is <code>C</code>, it is in the mood for "Coding"; if it is <code>J</code>, it is in the mood for "Jamming".</p>

<p>Limits</p>

<ul>
	<li>1 ≤ T ≤ 100.</li>
	<li>The length of S is even.</li>
	<li>2 ≤ the length of S ≤ 50.</li>
</ul>

### 출력 

 <p>For each test case, output one line containing <code>Case #x: y</code>, where <code>x</code> is the test case number (starting from 1) and <code>y</code> is the maximum number of points you can obtain for that case.</p>

