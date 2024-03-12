pub fn question_2(data: &mut [i64]) -> &mut [i64] {
    if data.iter().enumerate().any(|(index, &value)| index == 1 && value > 32) {
        if data.len() > 3 {
            data[3] = 10;
        }
    } else {
        if data.len() > 1 {
            data[1] = 10;
        }
    }

    data
}
