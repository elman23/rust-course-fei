//! You can use this file for experiments.
fn bubble_sort(items: &mut [i64]) {
    let n = items.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if items[j] > items[j + 1] {
                items.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut items = vec![3, 1, 2];
    bubble_sort(&mut items);
    println!("{:?}", items);
}
