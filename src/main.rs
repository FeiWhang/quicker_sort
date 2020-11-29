use rand::Rng;
use rand::rngs::ThreadRng;
use rayon::join;

fn main() {
    // let test_1: Vec<u64> = vec![9,18,1,6,7,8,4,12,10,6,1];
    // println!("{:?}", normal_quick_sort(test_1));

    let mut test_2: Vec<u64> = vec![8,4,19,1,14,1,3,3];
    quicker_sort(&mut test_2);
    println!("{:?}", test_2);
}

fn normal_quick_sort(xs: Vec<u64>) -> Vec<u64> {
    if xs.len() <= 1 { xs }
    else {
        let mut rng: ThreadRng = rand::thread_rng();
        let random_i: usize = rng.gen_range(0, xs.len());
        let pivot: u64 = *xs.get(random_i).unwrap();

        let less_than: Vec<u64> = xs.iter().filter(|&x| x < &pivot).map(|x| *x).collect();
        let equal: Vec<u64> = xs.iter().filter(|&x| x == &pivot).map(|x| *x).collect();
        let more_than: Vec<u64> = xs.iter().filter(|&x| x > &pivot).map(|x| *x).collect();

        let recurse_less_than: Vec<u64> = normal_quick_sort(less_than);
        let recurse_more_than: Vec<u64> = normal_quick_sort(more_than);

        [recurse_less_than, equal, recurse_more_than].concat()
    }
}

// use swap instead
fn quicker_sort(xs: &mut [u64]) {
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
        let pivot_value: u64 = *xs.get(pivot).unwrap();

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

        join(|| quicker_sort(less_than_or_eq),
            || quicker_sort(more_than_or_eq),);

    }

}
