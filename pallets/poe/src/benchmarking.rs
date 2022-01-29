#[allow(unused)]
use crate::*;
use frame_benchmarking::{benchmarks, account, impl_benchmark_test_suite, whitelisted_caller};
use frame_system::RawOrigin;
use sp_std::prelude::*;

benchmarks!{
  create_claim {
    let b in 1 .. 255;
    let caller = account("caller", 0, 0);
    let caller_clone = account("caller",0,0);
  }: _ (RawOrigin::Signed(caller), vec![b as u8;2])
  verify {
    let claim = vec![b as u8;2];
    let poe = Proofs::<T>::get(&claim);
    assert_eq!(poe, Some((caller_clone, frame_system::Pallet::<T>::block_number())));
  }
}

impl_benchmark_test_suite!(
  Poe,
  crate::mock::new_test_ext(),
  crate::mock::Test,
);

// #[cfg(test)]
// mod tests {
	// use super::*;
	// use crate::mock::{new_test_ext, Test};
	// use frame_support::assert_ok;
//
	// #[test]
	// fn test_benchmarks() {
		// new_test_ext().execute_with(|| {
			// assert_ok!(test_benchmark_create_claim::<Test>());
		// });
	// }
// }
