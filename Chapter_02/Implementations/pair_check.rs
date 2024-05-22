fn pair_check(arr: &mut Vec<i32>, x: i32) -> bool {
    
    // use accompanying merge sort implementation
    merge_sort(arr);

    let mut left = 0;
    let mut right = arr.len() - 1;

    while left < right {
        let sum = arr[left] + arr[right];

        if sum == x {
            return true;
        } else if sum < x {
            left += 1;
        } else {
            right -= 1;
        }
    }

    false
}