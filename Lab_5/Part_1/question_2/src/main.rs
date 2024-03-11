fn main() {
    let mut data = [10, 0, 12, 32];
    let condition = data[2] > 32;
    if condition {
        data[1] = 10;
    } else {
        data[0] = 10;
    }
}
