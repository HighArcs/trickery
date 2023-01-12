pub fn activity_one<const T: usize>(arr: [f64; T]) -> bool {
    for i in 0..arr.len() {
        if arr[i] < 0.0 {
            return true;
        }
    }

    false
}

pub fn activity_two<const T: usize>(vec: [i32; T]) -> u32 {
    let mut count = 0;
    for i in 0..vec.len() {
        if vec[i] % 3 == 0 {
            count += 1;
        }
    }

    count
}

pub fn activity_three<const T: usize>(num: i32, vec: [i32; T]) -> u32 {
    let mut count = 0;
    for i in 0..vec.len() {
        if vec[i] % num == 0 {
            count += 1;
        }
    }

    count
}