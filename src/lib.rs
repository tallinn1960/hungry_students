use std::collections::VecDeque;

pub fn count_students(students: &[i32], sandwiches: &[i32]) -> i32 {
    // this is interestingly faster than the naive filter-based count
    let (mut s, mut c) =
        students.iter().fold((0, 0), |(mut s, mut c), &student| {
            if student == 1 {
                s += 1;
            } else {
                c += 1;
            };
            (s, c)
        });
    for top in sandwiches {
        match top {
            1 if s > 0 => s -= 1,
            0 if c > 0 => c -= 1,
            _ => break,
        }
    }
    (s + c) as i32
}

pub fn count_students_leetcode(
    students: Vec<i32>,
    sandwiches: Vec<i32>,
) -> i32 {
    let mut students = VecDeque::from(students);
    let mut sandwiches = VecDeque::from(sandwiches);

    let mut serves = 0;
    while !students.is_empty() && serves < students.len() {
        if students[0] == sandwiches[0] {
            students.remove(0);
            sandwiches.remove(0);
            serves = 0;
        } else {
            let first_item = students.pop_front().unwrap();
            students.push_back(first_item);
            serves += 1;
        }
    }
    return students.len() as i32;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(count_students(&vec![1, 1, 0, 0], &vec![0, 1, 0, 1]), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(
            count_students(&vec![1, 1, 1, 0, 0, 1], &vec![1, 0, 0, 0, 1, 1]),
            3
        );
    }
}
