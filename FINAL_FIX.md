# Complete Fix for CI Build Failures

## Issues Fixed:

### 1. Storage Key Type Incompatibility
**Error**: `trait bound 'soroban_sdk::Val: TryFromVal<Env, u8>' is not satisfied`
**Fix**: Changed `u8` to `u32` in StorageKey enum
- `MaterialPrice(u8)` → `MaterialPrice(u32)`
- `PassportsByMaterial(u8, u64)` → `PassportsByMaterial(u32, u64)`

### 2. Event Emission Type Errors
**Error**: Event parameters not implementing `IntoVal` trait
**Fix**: Changed all event functions to use owned values
- All `Address` parameters now cloned before emission
- All `String` parameters now cloned before emission
- Enum values passed directly (Copy trait)

### 3. Dependency Version Conflict
**Error**: `ChaCha20Rng: ed25519_dalek::rand_core::CryptoRng` not satisfied
**Fix**: Updated Soroban SDK version
- `soroban-sdk = "21.7.0"` → `soroban-sdk = "21.7.7"`

## Files Modified:
1. `contracts/common/src/storage.rs` - Fixed u8 → u32
2. `contracts/common/src/events.rs` - Fixed event signatures
3. `contracts/waste_token/src/lib.rs` - Fixed event calls
4. `contracts/collector_registry/src/lib.rs` - Fixed event calls
5. `contracts/collection_point/src/lib.rs` - Fixed event calls
6. `Cargo.toml` - Updated SDK version

## Verification:
After this fix, all three CI jobs should pass:
- ✅ Check (formatting, clippy)
- ✅ Test (unit tests)
- ✅ Build WASM (contract compilation)
