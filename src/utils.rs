use rand::{Rng, SeedableRng};
use rand::distributions::Standard;
use rand_pcg::Pcg64Mcg;

pub fn new_u32_vec(n: usize) -> Vec<u32> {
    // init RNG
    let mut rng = Pcg64Mcg::from_seed([0; 16]);

    rng.sample_iter(&Standard).take(n).collect()
}

// listが昇順かの確認
pub fn is_sorted_ascending<T: Ord>(x: &[T]) -> bool {

    // windows(2) はリストを１要素刻みで２要素づつ返す
    // [ 1, 2 ,3 ,4 ] なら [1, 2] [2, 3] [3, 4] を返す
    x.windows(2).all(|pair| pair[0] <= pair[1])
}

pub fn is_sorted_decending<T: Ord>(x: &[T]) -> bool {
    x.windows(2).all(|pair| pair[0] >= pair[1])
}

mod tests {
    use crate::utils::{new_u32_vec,is_sorted_ascending,is_sorted_decending};
    use crate::third::sort;
    use crate::SortOrder::*;

    #[test]
    fn sort_32_large() {
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x,&Ascending),Ok(()));
            assert!(is_sorted_ascending(&x))
        }
        {
            let mut x = new_u32_vec(65536);
            assert_eq!(sort(&mut x,&Decending),Ok(()));
            assert!(is_sorted_decending(&x))
        }

    }
}