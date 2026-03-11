use std::io::{Read, stdin};

const MONTHLY_DAYS: [usize; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

fn main() {
    let mut buf = String::new();
    stdin().read_to_string(&mut buf).unwrap();
    let mut iter = buf.trim().split(&['-', ' ']).map(|x| x.parse().unwrap());

    let year: usize = iter.next().unwrap();
    let month = iter.next().unwrap();
    let day = iter.next().unwrap();
    let nth = iter.next().unwrap();

    let n = days_to_date(date_to_days(year, month, day) + nth - 1);
    println!("{:04}-{:02}-{:02}", n.0, n.1, n.2);
}

/*fn test_code() {
    let cele = [
        1, 10, 20, 30, 33, 40, 44, 50, 55, 60, 66, 70, 77, 80, 88, 90, 99, 100, 111, 123, 200, 222,
        300, 333, 400, 444, 500, 555, 600, 666, 700, 777, 800, 888, 900, 999, 1000, 1111, 1234,
        2000,
    ];

    for i in 1..=10000 {
        let n = days_to_date(date_to_days(1900, 1, 1) + i - 1);
        assert!(n.1 > 0 && n.1 < 13);
        assert!(n.2 > 0 && n.2 < 32);
    }

    println!(
        "1900-01-01 10000: {:?}",
        days_to_date(date_to_days(1900, 1, 1) + 10000 - 1)
    );

    for i in 1..=10000 {
        let n = days_to_date(date_to_days(2014, 12, 31) + i - 1);
        assert!(n.1 > 0 && n.1 < 13);
        assert!(n.2 > 0 && n.2 < 32);
    }

    println!(
        "2014-12-31 10000: {:?}",
        days_to_date(date_to_days(2014, 12, 31) + 10000 - 1)
    );
}*/

/// 윤년인지 확인
fn is_leaf_year(year: usize) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

/// 1900-01-01로부터의 날짜 차이 계산
fn date_to_days(year: usize, month: usize, day: usize) -> usize {
    let _year = year - 1900;
    let _month = month - 1;
    let _day = day - 1;

    let mut sum = 0;

    for y in 1900..year {
        if is_leaf_year(y) {
            sum += 366;
        } else {
            sum += 365;
        }
    }

    for (index, m) in MONTHLY_DAYS.iter().enumerate().take(month - 1) {
        if index == 1 && is_leaf_year(year) {
            sum += 1;
        }

        sum += *m;
    }

    sum += day - 1;

    sum
}

fn days_to_date(days: usize) -> (usize, usize, usize) {
    let mut _days = days;
    let days_in_year = |x| if is_leaf_year(x) { 366 } else { 365 };

    let mut year = 1900;
    while _days > days_in_year(year) {
        _days -= days_in_year(year);
        year += 1;
    }

    let mut month = 0;
    for (index, m) in MONTHLY_DAYS.iter().enumerate() {
        let mut monthly_days = *m;
        if index == 1 && is_leaf_year(year) {
            monthly_days += 1;
        }

        if _days >= monthly_days {
            _days -= monthly_days;
        } else {
            month = index;
            break;
        }
    }

    (year, month + 1, _days + 1)
}
