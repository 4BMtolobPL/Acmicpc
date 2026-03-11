# [Silver IV] 팰린드롬 애너그램 - 30458 

[문제 링크](https://www.acmicpc.net/problem/30458) 

### 성능 요약

메모리: 13464 KB, 시간: 0 ms

### 분류

구현, 문자열, 애드 혹

### 제출 일자

2026년 03월 11일 14:35:12

### 문제 설명

<p>팰린드롬이란 앞으로 읽어도, 뒤로 읽어도 같은 문자열을 의미한다. 예를 들어, <code>radar</code>는 팰린드롬이지만, <code>konkuk</code>은 팰린드롬이 아니다.</p>

<p>알파벳 소문자로만 이루어진 길이 $N$의 문자열 $S$가 주어진다. 문자열에 아래 연산을 $0$회 이상 수행해서 팰린드롬으로 만들 수 있을까?</p>

<ul>
	<li>문자열의 왼쪽 $\left\lfloor \frac{N}{2} \right\rfloor$개 문자 중 하나와 오른쪽 $\left\lfloor \frac{N}{2} \right\rfloor$개 문자 중 하나를 골라 서로 위치를 바꾼다.</li>
</ul>

### 입력 

 <p>첫째 줄에 문자열의 길이를 나타내는 정수 $N$이 주어진다. $\left( 3\leq N\leq 200\, 000 \right)$</p>

<p>둘째 줄에 길이 $N$의 문자열 $S$가 주어진다. $S$는 알파벳 소문자로만 이루어져 있다.</p>

### 출력 

 <p>주어진 연산을 $0$회 이상 수행해서 팰린드롬으로 만들 수 있다면 <span style="color:#e74c3c;"><code>Yes</code></span>를, 만들 수 없다면 <span style="color:#e74c3c;"><code>No</code></span>를 출력한다.</p>

