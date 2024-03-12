pub fn question_2(data: &mut [i64]) -> &mut [i64] {
    data.iter_mut()
        .enumerate()
        .for_each(|(index, value)| {
            if index == 1 && *value > 32 {
                data[3] = 10;
            } else if index == 1 {
                *value = 10;
            }
        });
    data
}
