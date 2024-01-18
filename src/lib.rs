use std::mem::size_of;

pub fn heapsort<T: Ord, const N: usize>(v: &mut [T]) {
    heapsort_by::<T, N>(v, |a, b| a.lt(b));
}

pub fn heapsort_by<T, const N: usize>(v: &mut [T], is_less: impl FnMut(&T, &T) -> bool) {
    heap_n_sort(v, make_sift_down::<T, N>(is_less));
}

pub fn heap_n_sort<T>(v: &mut [T], mut sift_down: impl FnMut(&mut [T], usize)) {
    if size_of::<T>() == 0 {
        return;
    }
    // Build the heap in linear time.
    for i in (0..v.len() / 2).rev() {
        sift_down(v, i);
    }

    // Pop maximal elements from the heap.
    for i in (1..v.len()).rev() {
        v.swap(0, i);
        sift_down(&mut v[..i], 0);
    }
}

struct AssertHeapChildren<const N: usize>;
impl<const N: usize> AssertHeapChildren<N> {
    const OK: () = assert!(N >= 2);
}

pub fn make_sift_down<T, const N: usize>(
    mut is_less: impl FnMut(&T, &T) -> bool,
) -> impl FnMut(&mut [T], usize) {
    #[allow(clippy::let_unit_value)]
    let _ = AssertHeapChildren::<N>::OK;

    move |v, mut node| loop {
        // Children of `node`.
        let mut child = N * node + 1;
        if child >= v.len() {
            break;
        }

        // Choose the greater child.
        {
            let mut max_idx = 0;
            let mut max = &v[child];
            for (idx, elm) in v.iter().skip(child).take(N).enumerate().skip(1) {
                if is_less(max, elm) {
                    max = elm;
                    max_idx = idx;
                }
            }
            child += max_idx;
        }

        // Stop if the invariant holds at `node`.
        if !is_less(&v[node], &v[child]) {
            break;
        }

        // Swap `node` with the greater child, move one step down, and continue sifting.
        v.swap(node, child);
        node = child;
    }
}

pub fn heap_3_sort<T, F>(v: &mut [T], mut is_less: F)
where
    F: FnMut(&T, &T) -> bool,
{
    // This "3-ary heap" respects the invariant `parent >= child`.
    let mut sift_down = |v: &mut [T], mut node| {
        loop {
            // Children of `node`.
            let mut child = 3 * node + 1;
            if child >= v.len() {
                break;
            }

            // Choose the greater child.
            if child + 2 < v.len() && is_less(&v[child + 1], &v[child + 2]) {
                // 1st is not grater
                // Comp 0th and 2nd, add 0 or 2
                child += is_less(&v[child], &v[child + 2]) as usize * 2;
            } else if child + 1 < v.len() {
                // Comp 0th and 1st, add 0 or 1
                child += is_less(&v[child], &v[child + 1]) as usize;
            }

            // Stop if the invariant holds at `node`.
            if !is_less(&v[node], &v[child]) {
                break;
            }

            // Swap `node` with the greater child, move one step down, and continue sifting.
            v.swap(node, child);
            node = child;
        }
    };

    // Build the heap in linear time.
    for i in (0..v.len() / 2).rev() {
        sift_down(v, i);
    }

    // Pop maximal elements from the heap.
    for i in (1..v.len()).rev() {
        v.swap(0, i);
        sift_down(&mut v[..i], 0);
    }
}

pub fn std_heapsort<T, F>(v: &mut [T], mut is_less: F)
where
    F: FnMut(&T, &T) -> bool,
{
    // This binary heap respects the invariant `parent >= child`.
    let mut sift_down = |v: &mut [T], mut node| {
        loop {
            // Children of `node`.
            let mut child = 2 * node + 1;
            if child >= v.len() {
                break;
            }

            // Choose the greater child.
            if child + 1 < v.len() {
                // We need a branch to be sure not to out-of-bounds index,
                // but it's highly predictable.  The comparison, however,
                // is better done branchless, especially for primitives.
                child += is_less(&v[child], &v[child + 1]) as usize;
            }

            // Stop if the invariant holds at `node`.
            if !is_less(&v[node], &v[child]) {
                break;
            }

            // Swap `node` with the greater child, move one step down, and continue sifting.
            v.swap(node, child);
            node = child;
        }
    };

    // Build the heap in linear time.
    for i in (0..v.len() / 2).rev() {
        sift_down(v, i);
    }

    // Pop maximal elements from the heap.
    for i in (1..v.len()).rev() {
        v.swap(0, i);
        sift_down(&mut v[..i], 0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    #[test]
    fn test_sort() {
        fn test_sort_n<const N: usize>() -> bool {
            let mut arr = (0..10u32.pow(3))
                .map(|_| random::<u32>())
                .collect::<Vec<_>>();
            heapsort::<_, N>(&mut arr);
            arr.windows(2).all(|v| v[0] <= v[1])
        }
        macro_rules! test_sorts {
            ($($x:expr),*) => {
                $(assert!(test_sort_n::<$x>());)*
            };
        }
        test_sorts! {
            2,3,4,5,6,7,8
        }
    }
}
