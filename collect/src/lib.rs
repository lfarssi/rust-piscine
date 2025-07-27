pub fn bubble_sort(arr : &mut [i32]){
    for i in 0..arr.len(){
        for j in 0..arr.len()-1-i{
            if arr[j]>arr[j+1]{
                arr.swap(j,j+1);
            }
        }
    }
    // arr.sort()
}