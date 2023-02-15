pub fn activity_one<const T: usize, const U: usize>(arr: [[i32; U]; T]) -> i32 {
    let mut sum = 0;

    for i in 0..arr.len() {
        for j in 0..arr[i].len() {
            if i == j {
                sum += arr[i][j];
            }
        }
    }

    sum
}

pub fn activity_two<const T: usize, const U: usize>() -> [[usize; U]; T] {
    let mut buf = [[0; U]; T];

    for i in 0..buf.len() {
        for j in 0..buf[i].len() {
            buf[i][j] = i * j;
        }
    }

    buf
}