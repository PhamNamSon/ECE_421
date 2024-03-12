pub fn question_7(data: &mut [i64]) -> &mut [i64] {
    let length: usize = data.len();
    for i in 0..length {
        // use min_by_key to find the index of the minimum value from i to length
        // swap the value at i with the value at min_index
        if let Some(min_index) = (i..length).min_by_key(|&j| &data[j]) {
            data.swap(i, min_index);
        }
    }
    data
}

fn main() {
    let mut l: Vec<i64> = vec![5, 3, 2, 1, 4];
    question_7(&mut l);
    println!("{:?}", l);
}
