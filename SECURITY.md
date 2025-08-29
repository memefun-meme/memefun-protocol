# 🔒 Security Guidelines - Solana Memes

This document outlines security best practices and guidelines for the Solana Memes project.

## 🛡️ Protected Information

The following files and information are **NEVER** committed to the repository:

### ✅ Environment Variables
- `.env` files (all variants)
- API keys and secrets
- Database credentials
- Private keys and wallet seeds
- External service tokens

### ✅ Build Artifacts
- `node_modules/` directories
- `target/` directories (Rust builds)
- `dist/` directories (compiled assets)
- `.anchor/` directories (Anchor builds)

### ✅ Logs and Temporary Files
- Log files (`*.log`)
- Temporary directories (`tmp/`, `temp/`)
- Cache directories
- IDE-specific files

## 🔐 Security Checklist

### Before Committing
- [ ] No `.env` files in the repository
- [ ] No private keys or wallet seeds
- [ ] No API keys or secrets
- [ ] No database credentials
- [ ] No build artifacts
- [ ] No log files

### Before Pushing to GitHub
- [ ] Run `git status` to verify no sensitive files
- [ ] Check that `.gitignore` is working properly
- [ ] Verify no secrets in commit history
- [ ] Ensure all sensitive data is in `.env` files

## 🚨 Security Best Practices

### Environment Variables
```bash
# ✅ Good - Use .env files
cp env.example .env
# Edit .env with your actual values

# ❌ Bad - Never commit actual values
echo "API_KEY=your_actual_key" >> .env
git add .env  # NEVER do this!
```

### Private Keys
```bash
# ✅ Good - Use environment variables
export WALLET_PRIVATE_KEY="your_key_here"

# ❌ Bad - Never hardcode keys
const privateKey = "your_actual_private_key";
```

### Database Credentials
```bash
# ✅ Good - Use environment variables
DATABASE_URL=postgresql://user:pass@localhost:5432/db

# ❌ Bad - Never commit credentials
DATABASE_URL=postgresql://admin:password123@localhost:5432/production
```

## 🔍 Security Verification

### Check for Sensitive Files
```bash
# Check if any .env files are tracked
git ls-files | grep -E "\.env"

# Check for potential secrets in code
grep -r "password\|secret\|key\|token" src/ --exclude-dir=node_modules

# Check for private keys
grep -r "private.*key\|secret.*key" src/ --exclude-dir=node_modules
```

### Verify .gitignore is Working
```bash
# Test .gitignore
echo "test" > .env
git status  # Should NOT show .env
rm .env
```

## 🛠️ Security Tools

### Pre-commit Hooks
Consider installing pre-commit hooks to automatically check for secrets:

```bash
# Install pre-commit
pip install pre-commit

# Install detect-secrets
pip install detect-secrets

# Create .pre-commit-config.yaml
pre-commit install
```

### Secret Scanning
Use tools like:
- [GitGuardian](https://www.gitguardian.com/)
- [TruffleHog](https://github.com/trufflesecurity/truffleHog)
- [Detect-secrets](https://github.com/Yelp/detect-secrets)

## 🚨 Emergency Procedures

### If Secrets Are Accidentally Committed

1. **Immediate Action**
   ```bash
   # Remove from tracking (but keep locally)
   git rm --cached .env
   
   # Commit the removal
   git commit -m "Remove sensitive files"
   ```

2. **Rotate Secrets**
   - Change all API keys
   - Generate new private keys
   - Update database passwords
   - Update environment variables

3. **Force Push (if necessary)**
   ```bash
   # Only if secrets were pushed to remote
   git push --force-with-lease origin main
   ```

## 📋 Repository Security Settings

### GitHub Repository Settings
- [ ] Enable branch protection rules
- [ ] Require pull request reviews
- [ ] Enable secret scanning
- [ ] Enable dependency scanning
- [ ] Set up security alerts

### Access Control
- [ ] Limit repository access
- [ ] Use personal access tokens with minimal permissions
- [ ] Enable two-factor authentication
- [ ] Regular access reviews

## 🔐 Production Security

### Deployment Security
- [ ] Use environment-specific configurations
- [ ] Encrypt secrets at rest
- [ ] Use secure key management (AWS KMS, HashiCorp Vault)
- [ ] Implement proper access controls
- [ ] Regular security audits

### Monitoring
- [ ] Set up security monitoring
- [ ] Monitor for suspicious activities
- [ ] Regular log analysis
- [ ] Automated security scanning

## 📞 Security Contacts

- **Security Issues**: security@solana-memes.com
- **Emergency**: Create a security advisory on GitHub
- **Responsible Disclosure**: Please report vulnerabilities privately

---

**Remember: Security is everyone's responsibility! 🔒**
