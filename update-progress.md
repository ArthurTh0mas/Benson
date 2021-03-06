## Update plan
- update crates to 4.0.0-dev
- fix warnings
- use this substrate commit to match frontier: https://github.com/paritytech/frontier/commit/bcae5695242cc346aa26814b1160fe7e041854b3#diff-13ee4b2252c9e516a0547f2891aa2105c3ca71c6d7a1e682c69be97998dfc87e
- switch deps to benson/substrate fork (ensure ethy bridge keys are derived correctly)

## Crate update progress
---
crml
- [x] attestation (removed*)
- [x] generic-asset
    - [x] tests
- [x] governance
    - [x] tests
- [x] nft
    - [x] tests
- [x] transaction-payment
    - [x] tests
- [x] eth-bridge
    - [x] tests
- [x] eth-wallet
    - [x] tests
- [x] erc20-peg
    - [x] tests
- [x] support
- [x] sylo (removed*)
- [x] cennzx
    - [x] tests
- [x] staking
    - [x] tests
- [x] ethy-gadget
    - [x] tests
- [x] cli
    - [x] tests
- [x] runtime
    - [x] tests

*modules unused, removed to speed up update process

## Migration Notes

- runtime deps updated to 4.0.0-dev
- cli deps updated to 0.10.0-dev
- scale-info import required on most crml crates
- scale-info derive `TypeInfo` needed on most types
- warning: <frame_system::Pallet<T>> -> <frame_system::Pallet<T>>
- new prelude imports for pallets: `frame_system::pallet_prelude::*` && `frame_support:pallet_prelude::*`
- structs using T and deriving `TypeInfo` require `#[scale_info(skip_type_params(T))]`
- For test runtimes, replace `Module` with `Pallet`
- For test runtimes, `type BaseCallFilter = frame_support::traits::Everything;` works
- For test runtimes, add `type OnSetCode = ();`
