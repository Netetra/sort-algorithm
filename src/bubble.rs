pub fn sort<T: PartialOrd + Clone>(data_ref: &[T]) -> Vec<T> {
    let mut data = data_ref.to_vec();
    let len = data.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if data[j] > data[j + 1] {
                data.swap(j, j + 1);
            }
        }
    }
    data
}
