export MFA_AGENT_IS_PROXY=true
export MFA_AGENT_IS_DEV=true
export MFA_AGENT_LOG_LEVEL=info

if [[ -f "./target/debug/mfa-agent" ]]; then
    ./target/debug/mfa-agent
else
    mfa-agent
fi
