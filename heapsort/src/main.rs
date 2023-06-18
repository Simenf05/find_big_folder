

fn heapify(arr: &mut Vec<u64>, n: i32, i: i32) {

    let mut largest = if i < 0 { arr.len() as i32 + i } else { i };
    let l = if 2 * i + 1 < 0 { arr.len() as i32 + (2 * i + 1) } else { 2 * i + 1 };
    let r = if 2 * i + 2 < 0 { arr.len() as i32 + (2 * i + 2) } else { 2 * i + 2 };
    let i = if i < 0 { arr.len() as i32 + i } else { i };

    
    if l < n && arr[largest as usize] < arr[l as usize] {
        largest = l;
    }

    if r < n && arr[largest as usize] < arr[r as usize] {
        largest = r;
    }

    if largest != i {
        arr.swap(i as usize, largest as usize);

        heapify(arr, n, largest);
    }
}


fn heap_sort(arr: &mut Vec<u64>) {
    
    let n = arr.len() as i32;
    
    for i in (-1..n / 2 - 1).rev() {
        
        heapify(arr, n, i);
    }

    for i in (0..n-1).rev() {
        arr.swap(i as usize, 0);

        heapify(arr, i, 0);

    }

}


fn main() {
    

    let mut arr = vec![7, 3, 2, 4, 8, 6, 1, 5, 9, 10];
    

    heap_sort(&mut arr);

    
    println!("{arr:?}");

}

