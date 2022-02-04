/// takes slice of chars (as u8) and returns the parsed int
/// so [b'1', b'2', b'3'] -> 123
pub fn str_to_int(string: &[u8]) -> u32 {
    let mut out = 0;
    let mut factor = 1;
    for i in string.iter().rev() {
        let digit = (*i - b'0') as u32;
        out += digit * factor;
        factor *= 10;
    }
    out
}

fn restore_heap<T: PartialOrd>(arr: &mut [T], size: usize, mut i: usize) {
    let mut largest = i;
    while {
        arr.swap(i, largest);
        i = largest;

        if 2 * i + 1 < size && arr[2 * i + 1] > arr[largest] {
            largest = 2 * i + 1;
        }
        if 2 * i + 2 < size && arr[2 * i + 2] > arr[largest] {
            largest = 2 * i + 2;
        }

        largest != i
    } { /**/ }
}

pub fn heapsort<T: PartialOrd>(arr: &mut [T]) {
    for i in (0..arr.len() / 2).rev() {
        restore_heap(arr, arr.len(), i);
    }

    for i in (0..arr.len()).rev() {
        arr.swap(i, 0);
        restore_heap(arr, i, 0);
    }
}
