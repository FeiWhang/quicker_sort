// SAMPLE INPUT FROM: https://github.com/ashwinravishankar/Sorting-Techniques
use rand::Rng;
use rand::rngs::ThreadRng;
use std::fs::File;
use std::io::prelude::Read;
use rayon::join;

fn main() {

    let mut f = File::open("resources/input_100000.txt").unwrap();
    let mut whole_file = String::new();
    let _ = f.read_to_string(&mut whole_file);

    let mut test_input: Vec<i64> = whole_file
        .split(" ")
        .map(|s| s.parse::<i64>().unwrap()).collect();

    quicker_sort(&mut test_input);

    let sorted = test_input.clone();
    test_input.sort_unstable();
    assert_eq!(sorted, test_input);

}

// 3-way Quick-Sort Algo from: http://www.cs.princeton.edu/~rs/talks/QuicksortIsOptimal.pdf
fn quicker_sort(xs: &mut Vec<i64>) {
    fn sort_helper(xs: &mut Vec<i64>, left: i64, right: i64) {
        if right <= left { return }

        let mut i: i64 = left - 1;
        let mut j: i64 = right;
        let mut p: i64 = left - 1;
        let mut q: i64 = right;
        let v: i64 = *xs.get(right as usize).unwrap();

        loop {
            i += 1;
            while *xs.get(i as usize).unwrap() < v { i += 1; }

            j -= 1;
            while v < *xs.get(j as usize).unwrap() {
                if j == left { break }
                j -= 1;
            }

            if i >= j { break }

            xs.swap(i as usize, j as usize);

            if *xs.get(i as usize).unwrap() == v {
                p += 1;
                xs.swap(p as usize, i as usize);
            }

            if v == *xs.get(j as usize).unwrap() {
                q -= 1;
                xs.swap(j as usize, q as usize);
            }
        }

        xs.swap(i as usize, right as usize);
        j = i - 1;
        i += 1;

        for z in left..p-1{
            xs.swap(z as usize, j as usize);
            j -= 1;
        }

        for z in (right-1)..q-1 {
            xs.swap(i as usize, z as usize);
            i += 1;
        }

        sort_helper(xs, left, j);
        sort_helper(xs, i, right);

    }

    let length = xs.len();
    if length > 1 {
        sort_helper(xs, 0, (length - 1) as i64);
    }
}

fn _normal_quick_sort(xs: Vec<u64>) -> Vec<u64> {
    if xs.len() <= 1 { xs }
    else {
        let mut rng: ThreadRng = rand::thread_rng();
        let random_i: usize = rng.gen_range(0, xs.len());
        let pivot: u64 = *xs.get(random_i).unwrap();

        let less_than: Vec<u64> = xs.iter().filter(|&x| x < &pivot).map(|x| *x).collect();
        let equal: Vec<u64> = xs.iter().filter(|&x| x == &pivot).map(|x| *x).collect();
        let more_than: Vec<u64> = xs.iter().filter(|&x| x > &pivot).map(|x| *x).collect();

        let recurse_less_than: Vec<u64> = _normal_quick_sort(less_than);
        let recurse_more_than: Vec<u64> = _normal_quick_sort(more_than);

        [recurse_less_than, equal, recurse_more_than].concat()
    }
}

// STACK_OVERFLOWED when input is longer than 400
// ANY CHANGE TO IMPROVE THIS VERSION...?
fn _parallel_quicker_sort(xs: &mut [i64]) {
    let length: usize = xs.len();

    if length == 2 {
        if xs.first().unwrap() > xs.last().unwrap() {
            xs.swap(0, 1);
        }
    }

    if length > 2 {
        // randomize pivot
        let mut rng: ThreadRng = rand::thread_rng();
        let mut pivot: usize = rng.gen_range(1, length);
        let pivot_value: i64 = *xs.get(pivot).unwrap();

        // move pivot to the front
        xs.swap(0, pivot);
        pivot = 0;

        for i in 1..length {
            // if xs[i] less than pivot value
            // swap xs[i] with xs[pivot+1]
            if xs.get(i).unwrap() < &pivot_value {
                pivot += 1;
                xs.swap(i, pivot)
            }
        }

        // swap pivot value back to place
        xs.swap(0, pivot);
        // after swapping done
        // every n less than or eq to pivot value
        // will be on the left of pivot

        let (less_than_or_eq, more_than_or_eq) = xs.split_at_mut(pivot);

        join(|| _parallel_quicker_sort(less_than_or_eq),
             || _parallel_quicker_sort(more_than_or_eq), );
    }
}