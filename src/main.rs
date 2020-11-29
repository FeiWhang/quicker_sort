use rand::Rng;

fn main() {
    let test_1: Vec<usize> = vec![9,18,1,6,7,8,4,12,10,6,1];
    println!("{:?}", _recursive_quick_sort(test_1));
}

fn _recursive_quick_sort(xs: Vec<usize>) -> Vec<usize> {
    if xs.len() <= 1 {
        xs
    } else {
        let mut rng = rand::thread_rng();
        let random_i: usize = rng.gen_range(0, xs.len());
        let pivot: &usize = xs.get(random_i).unwrap();
        let less_than: Vec<usize> = xs.iter().filter(|&x| x < pivot).map(|x| *x).collect();
        let equal: Vec<usize> = xs.iter().filter(|&x| x == pivot).map(|x| *x).collect();
        let more_than: Vec<usize> = xs.iter().filter(|&x| x > pivot).map(|x| *x).collect();

        let recurse_less_than: Vec<usize> = _recursive_quick_sort(less_than);
        let recurse_more_than: Vec<usize> = _recursive_quick_sort(more_than);

        [recurse_less_than, equal, recurse_more_than].concat()
    }
}


// fn quicker_sort() {
//
// }