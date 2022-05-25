## Debugging
The `MFA_AGENT_IS_DEV` and `MFA_AGENT_LOG_LEVEL` environment variables can be used during
development:

```
MFA_AGENT_IS_PROXY=true MFA_AGENT_IS_DEV=true MFA_AGENT_LOG_LEVEL=debug ./target/debug/mfa-agent
```
