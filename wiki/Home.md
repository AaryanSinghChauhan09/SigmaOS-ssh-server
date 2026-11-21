# SigmaOS-ssh-server

SigmaOS integration component for **OpenSSH server**.

## Commands

```bash
sigmaos-ssh-server install   # install package + deploy defaults
sigmaos-ssh-server deploy    # copy config templates to $HOME
sigmaos-ssh-server status    # binary/package/config report
sigmaos-ssh-server reset     # remove deployed config
sigmaos-ssh-server open      # launch the application
```

## Design

- Shell-based component manager (Omarchy-style per-app repos)
- Secure defaults; no secrets stored
- Deterministic `test.sh` with isolated `$HOME`
