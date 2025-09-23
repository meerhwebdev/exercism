pub fn find<C, T>(array: C, key: T) -> Option<usize>
where 
    C : AsRef<[T]>,
    T:PartialEq + PartialOrd + Copy
{
    let slice = array.as_ref();
    if slice.is_empty() { return None}
    let mid = (slice.len() - 1)/ 2;
    binary_search(slice, key, mid)
}

fn binary_search<T:PartialEq + PartialOrd + Copy>(array: &[T], key:T, mut index:usize) -> Option<usize> {
    let current_mid_index= (array.len() - 1) / 2;
    let middle_value = array[current_mid_index];
    let left_list  = &array[..current_mid_index];
    let right_list = &array[current_mid_index+1..];
    let left_list_len = left_list.len();
    let right_list_len = right_list.len();
    if middle_value == key {
        return Some(index)
    } else if key < middle_value && left_list_len > 0 {
        index  = (left_list_len - 1) / 2;
        return binary_search(left_list, key, index)
    } else if key > middle_value && right_list_len > 0 {
        index  = index + ((right_list_len - 1) / 2) + 1;
        return binary_search(right_list, key, index)
    } 
    else {
       None
    }
}