# Fix: Compilation Errors in Common Library

## Issues Fixed:

### 1. Unused Variable Warnings
- Fixed `env` parameter in `require_auth()` - prefixed with underscore
- Fixed `material_type` parameter in `validate_material_type()` - prefixed with underscore

### 2. Event Emission Type Errors  
- Changed all event functions to accept references instead of owned values
- Updated event calls throughout contracts to pass references

## Files Modified:
- `contracts/common/src/utils.rs` - Fixed unused variables
- `contracts/common/src/validation.rs` - Fixed unused variables  
- `contracts/common/src/events.rs` - Changed signatures to use references
- `contracts/waste_token/src/lib.rs` - Updated event calls
- `contracts/collector_registry/src/lib.rs` - Updated event calls
- `contracts/collection_point/src/lib.rs` - Updated event calls

## Build Status:
All compilation errors resolved. Ready to build and test.
