pub fn activity_one<const N: usize>(mut arr: [String; N]) {
    for i in 1..arr.len() {
        let to_insert = arr[i].clone();
        let mut j = (i - 1) as isize;

        while j >= 0 {
            if to_insert >= arr[j as usize] {
                arr[j as usize + 1] = arr[j as usize].clone();
            } else {
                break;
            }

            j -= 1;
        }

        arr[j as usize + 1] = to_insert.clone();
        println!("{arr:?}");
    }
}

pub fn activity_two(mut arr: Vec<i32>) -> usize {
    let mut count = 0;
    for i in 1..arr.len() {
        let val = arr[i];
        let mut j = (i - 1) as isize;

        while j >= 0 {
            count += 1;
            if arr[j as usize] > val {
                arr[j as usize + 1] = arr[j as usize];
            } else {
                break;
            }

            j -= 1;
        }
    }

    count
}
