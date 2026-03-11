# [Gold III] Bubu - 8757 

[문제 링크](https://www.acmicpc.net/problem/8757) 

### 성능 요약

메모리: 34948 KB, 시간: 152 ms

### 분류

데이크스트라, 그래프 이론, 최단 경로

### 제출 일자

2026년 03월 11일 14:35:12

### 문제 설명

<p>Bubu wyszło z jaskini szukać pikników, jednak zauważyli je strażnicy parku Jellystone. Strażnicy próbują zagrodzić drogę Bubu, które ucieka do jaskini. Strażnik złapie Bubu, jeśli znajdzie się w tym samym czasie na polanie, na której jest Bubu. Strażnik może odpoczywać na polanach.</p>

### 입력 

 <p>W pierwszej linii znajdują się trzy liczby całkowite <em>n</em>, <em>m</em>, <em>s</em>, (1 ≤ <em>n</em> ≤ 10<sup>5</sup>, 0 ≤ <em>m</em> ≤ 2 · 10<sup>5</sup>, 0 ≤ <em>s</em> ≤ 3 · 10<sup>4</sup>) oznaczające odpowiednio liczbę polan w lesie, liczbę przesiek między tymi polanami oraz liczbę strażników. W następnych <em>m</em> liniach znajdują się opisy (dwukierunkowych) przesiek w postaci trzech liczb całkowitych <em>a</em>, <em>b</em>, <em>w</em>, (1 ≤ <em>a</em>, <em>b</em> ≤ <em>n</em>, 1 ≤ <em>w</em> ≤ 10<sup>9</sup>), gdzie <em>a</em>, <em>b</em> oznaczają numery polan, zaś <em>w</em> oznacza czas w sekundach, jaki zajmuje strażnikowi lub Bubu przebycie tej przesieki. W następnych <em>s</em> liniach podane są numery polan, na których stoją kolejni strażnicy. W ostatniej linii podany jest numer polany, na której znajduje się Bubu. Jaskinia Bubu jest na polanie 1.</p>

### 출력 

 <p>Jeżeli Bubu może dotrzeć do jaskini niezłapany przez strażników, należy wypisać minimalny czas dotarcia Bubu do jaskini, w innym przypadku należy wypisać -1. Zakładamy, że istnieje ścieżka między jaskinią Bubu a polaną, gdzie się ono znajduje.</p>

