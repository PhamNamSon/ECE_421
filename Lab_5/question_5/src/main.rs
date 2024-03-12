fn question_2(data: &mut [i64]) -> &mut [i64] {
    if data[1] > 32 {
        data[3] = 10;
    } else {
        data[1] = 10;
    }
    data
}

pub fn main() {
    let mut data = [10, 0, 12, 32];
    question_2(&mut data);
}