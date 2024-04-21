pub fn sort<T: PartialOrd + Clone>(data_ref: &[T]) -> Vec<T> {
    let mut data = data_ref.to_vec();
    let len = data.len();
    let mut min_index;
    for i in 0..len {
        min_index = i;
        for j in i + 1..len {
            if data[min_index] > data[j] {
                min_index = j;
            }
        }
        data.swap(i, min_index);
    }
    data
}
