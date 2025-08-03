pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for i in (0..array.len()).rev(){
        if array[i]==key{
            return Some(i);
        }
    }
    None
}