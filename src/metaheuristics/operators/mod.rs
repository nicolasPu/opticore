use rand::Rng;

pub fn two_opt_swap(mut state: Vec<usize>) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let vec_len = state.len();

    let index1 = rng.gen_range(0..vec_len);
    let mut index2 = rng.gen_range(0..vec_len);

    while index1 == index2 {
        index2 = rng.gen_range(0..vec_len);
    }

    state.swap(index1, index2);

    state
}
