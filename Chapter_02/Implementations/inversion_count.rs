fn merge_sort_and_count_inversions(arr: &mut Vec<i32>) -> u64 {
    let n = arr.len();
    if n <= 1 {
        return 0;
    }

    let mid = n / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();

    let mut inversions = merge_sort_and_count_inversions(&mut left)
        + merge_sort_and_count_inversions(&mut right);

    inversions += merge_and_count_split_inversions(arr, &left, &right);

    inversions
}

fn merge_and_count_split_inversions(
    arr: &mut Vec<i32>,
    left: &Vec<i32>,
    right: &Vec<i32>,
) -> u64 {
    let mut inversions = 0;
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            inversions += left.len() as u64 - i as u64;
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        arr[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        arr[k] = right[j];
        j += 1;
        k += 1;
    }

    inversions
}