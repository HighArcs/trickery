pub fn activity_one(mut f: impl std::io::Write, n: i32) {
    if n - 10 > 0 {
        activity_one(n - 10)
    }

    write!(f, "{} ", n)
}