use std::ops::RangeInclusive;

fn check_passwords(range: RangeInclusive<i64>) -> usize {
    range
        .filter_map(check_password)
        .count()
}

fn check_passwords2(range: RangeInclusive<i64>) -> usize {
    range
        .filter_map(check_password_2)
        .count()
}

fn check_password(password:i64) -> Option<i64> {
    let pstr = password.to_string();
    let mut last_char = '-';
    let mut double = false;
    let mut decrease = false;
    for char in pstr.chars() {
        if char == last_char {
            double = true
        }
        if char < last_char {
            decrease = true
        }
        last_char = char
    }
    if double && !decrease {
        Some(password)
    } else {
        None
    }
}

fn check_password_2(password:i64) -> Option<i64> {
    let pstr = password.to_string();
    let mut last_char = '-';
    let mut groups:Vec<i64> = Vec::new();
    let mut group_count = 0;
    let mut decrease = false;
    for char in pstr.chars() {
        if char == last_char {
            group_count+=1;
        } else {
            if group_count > 0 {
                groups.push(group_count+1);
                group_count = 0;
            }
        }
        if char < last_char {
            decrease = true
        }
        last_char = char
    }
    if group_count > 0 { groups.push(group_count+1) };
    if groups.contains(&2) && !decrease {
        Some(password)
    } else {
        None
    }
}


fn main() {
    let passwords = check_passwords(137683..=596253);
    let passwords2 = check_passwords2(137683..=596253);
    println!("{}, {}", passwords, passwords2);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_range1() {
        let n = check_passwords(111111..=111111);
        assert_eq!(n,1);
    }

    #[test]
    fn test_range2() {
        let n = check_passwords2(123444..=123444);
        assert_eq!(n,0);
    }

    #[test]
    fn test_range3() {
        let n = check_passwords2(111122..=111122);
        assert_eq!(n,1);
    }

}