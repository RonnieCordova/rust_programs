# Rust Projects

Learning Rust by building real tools that interact with the OS at a low level. The focus is on systems programming, Windows internals, and offensive security tooling — understanding *why* things work, not just making them run.

---

## Projects

### inspector_procesos
A native Windows process inspector built on top of the ToolHelp32 WinAPI. Lists all active processes by talking directly to the kernel — no third-party wrappers.

**Planned features:**
- List all running processes
- Inspect access tokens and privileges of a given PID
- Identify potentially misconfigured privilege levels

**Stack:** Rust + `windows-sys`