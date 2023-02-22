pub fn activity_one<const N: usize>(mut arr: [String; N]) {
    let n = arr.len();
    for i in 0..(n - 1) { 
        let mut min_idx = i;
        for j in (i + 1)..n {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        
        (arr[min_idx], arr[i]) = (arr[i].clone(), arr[min_idx].clone());
    }

    for i in arr {
        write!(f, "{i} ");
    }
}

pub fn activity_two(mut vec: Vec<i32>) {
    for i in 0..(vec.len() - 1) {
        let mut min_idx = i;
        for j in (i + 1)..(vec.len()) {
            if vec[j] > vec[min_idx] {
                min_idx = j;
            }
        }
        
        (vec[min_idx], vec[i]) = (vec[i], vec[min_idx]);
    }

    writeln!(f, "{vec:?}");
}
