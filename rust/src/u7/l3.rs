pub fn activity_one(mut vec: &mut Vec<String>) {
    vec.add(vec.len() - 1, vec.remove(0));
}

pub fn activity_two(vec: Vec<String>) {
    let mut sum = 0;
    let mut mode = vec.get(0).unwrap();
    let mut has_mode = false;
    let mut max_count = 0;

    for i in 0..vec.len() {
        sum += vec.get(i).unwrap();
        let mut count = 1;
        for j in (i + 1)..vec.len() {
            if vec.get(i).unwrap() == vec.get(j).unwrap() {
                count++;
            }
        }

        if count > max_count {
            mode = vec.get(i).unwrap();
            has_mode = true;
            max_count = count;
        } else if count == max_count {
            has_mode = false;
        }
    }

    println!("Sum: {sum}");
    println!("Average: {}", (sum as f64) / (vec.len() as f64));
    println!("Mode: {}", if has_mode { mode.to_string() } else { "no single mode".to_string() })
}