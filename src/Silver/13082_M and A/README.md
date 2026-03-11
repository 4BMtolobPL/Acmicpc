# [Silver II] M and A - 13082 

[문제 링크](https://www.acmicpc.net/problem/13082) 

### 성능 요약

메모리: 13200 KB, 시간: 0 ms

### 분류

문자열, 두 포인터

### 제출 일자

2026년 03월 11일 14:35:12

### 문제 설명

<p>The CEO of the company named SS is planning M&A with another company named TT. M&A is an abbreviation for "Mergers and Acquisitions". The CEO wants to keep the original company name SS through the M&A on the plea that both company names are mixed into a new one.</p>

<p>The CEO insists that the mixed company name after the M&A is produced as follows.</p>

<p>Let ss be an arbitrary subsequence of S, and t be an arbitrary subsequence of T. The new company name must be a string of the same length to S obtained by alternatively lining up the characters in s and t. More formally, s<sub>0</sub>+t<sub>0</sub>+s<sub>1</sub>+t<sub>1</sub>+⋯ or t<sub>0</sub>+s<sub>0</sub>+t<sub>1</sub>+s<sub>1</sub>+⋯ can be used as the company name after M&A. Here, sksk denotes the kk-th (0-based) character of string s. Please note that the lengths of s and t will be different if the length of S is odd. In this case, the company name after M&A is obtained by s<sub>0</sub>+t<sub>0</sub>+⋯+t<sub>|S|/2</sub>+s<sub>|S|/2+1</sub> or t<sub>0</sub>+s<sub>0</sub>+⋯+s<sub>|S|/2</sub>+t<sub>|S|/2+1</sub> (|S| denotes the length of S and "/" denotes integer division).</p>

<p>A subsequence of a string is a string which is obtained by erasing zero or more characters from the original string. For example, the strings "abe", "abcde" and "" (the empty string) are all subsequences of the string "abcde".</p>

<p>You are a programmer employed by the acquiring company. You are assigned a task to write a program that determines whether it is possible to make S, which is the original company name, by mixing the two company names.</p>

### 입력 

 <p>The input consists of a single test case. The test case consists of two lines.</p>

<p>The first line contains a string S which denotes the name of the company that you belong to. The second line contains a string T which denotes the name of the target company of the planned M&A. The two names S and T are non-empty and of the same length no longer than 1,0001,000 characters, and all the characters used in the names are lowercase English letters.</p>

<p> </p>

### 출력 

 <p>Print "Yes" in a line if it is possible to make original company name by combining S and T. Otherwise, print "No" in a line</p>

<p> </p>

