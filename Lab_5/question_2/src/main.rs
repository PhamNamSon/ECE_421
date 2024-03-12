pub fn question_2(data: &mut [i64]) -> &mut [i64] {
    let length = data.len();
    for i in 0..length {
        let mut min = i;
        for j in (i+1)..length {
            if data[j] < data[min] {
                min = j;
            }
        }
        if min != i {
            data.swap(i, min);
        }
    }
    data
}

pub fn main() {
    let mut l: Vec<i64> = vec![5, 3, 2, 1, 4];
    question_2(&mut l);
    println!("{:?}", l);
}