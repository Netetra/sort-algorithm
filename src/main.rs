mod bubble;
mod selection;

fn main() {
    let data = vec![8, 2, 6, 4, 9, 3, 7, 1, 5];
    let bubble = bubble::sort::<i32>(&data);
    let selection = selection::sort::<i32>(&data);
    println!("data: {:?}", data);
    println!("bubble: {:?}", bubble);
    println!("bubble: {:?}", selection);
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_test() {
        let mut data = vec![8, 2, 6, 4, 9, 3, 7, 1, 5];
        let bubble = bubble::sort(&data);
        data.sort();
        assert_eq!(data, bubble);
    }

    #[test]
    fn selection_test() {
        let mut data = vec![8, 2, 6, 4, 9, 3, 7, 1, 5];
        let selection = selection::sort(&data);
        data.sort();
        assert_eq!(data, selection);
    }
}
