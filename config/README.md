# WasteFi Configuration Files

This directory contains deployment configuration files for different networks.

## Files

### testnet.json
Complete configuration for Stellar testnet deployment.

**Usage**:
```bash
# Unix/Linux/macOS
./scripts/deploy.sh testnet config/testnet.json

# Windows
.\scripts\deploy.ps1 -Network testnet -ConfigFile config\testnet.json
```

### mainnet.template.json
Template for mainnet deployment. **MUST be customized before use**.

**Steps Before Mainnet Deployment**:
1. Copy template: `cp mainnet.template.json mainnet.json`
2. Replace all `REPLACE_WITH_*` placeholders
3. Update admin and operator addresses
4. Configure multi-sig (strongly recommended)
5. Set up monitoring and alerting
6. Complete deployment checklist
7. **DO NOT commit mainnet.json with real keys to version control**

## Configuration Schema

```json
{
  "network": "testnet|mainnet",
  "rpc_url": "Stellar RPC endpoint",
  "network_passphrase": "Network passphrase",
  "admin": "Admin Stellar address",
  "operator": "Operator Stellar address",
  "contracts": {
    "<contract_name>": {
      "wasm": "Path to WASM file",
      "init_params": {}
    }
  },
  "rate_limits": {},
  "fraud_detection": {},
  "price_bounds": {},
  "emergency": {},
  "monitoring": {}
}
```

## Security Notes

- **NEVER** commit mainnet admin private keys
- Use environment variables for sensitive data
- Enable multi-sig for mainnet admin
- Regularly rotate operator keys
- Keep testnet and mainnet configs separate

## Validation

Validate configuration before deployment:
```bash
# Check JSON syntax
cat config/testnet.json | jq .

# Verify required fields present
jq -r '.admin, .operator, .network' config/testnet.json
```

---

For questions: security@wastefi.io
