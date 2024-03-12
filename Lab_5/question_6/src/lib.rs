pub fn question_2(data: &mut [i64]) -> &mut [i64] {
    if data[1] > 32 {
        data[3] = 10;
    } else {
        data[1] = 10;
    }
    data
}
