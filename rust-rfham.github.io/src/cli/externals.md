# External Commands

The RF-Ham CLI supports extension in a similar manner to Git and Cargo, which is to assume any
command it does not recognize *may* represent an external command to run as a sub-process.
Specifically an unknown command `qso` would have the CLI try and execute a process `rfham-qso`
on the current path (`PATH` environment variable).

```bash
❯ cargo run qso  
🛑 Error: no executable found for external command
   ├── 🔎 Command `qso`
   └── ℹ️  Help check that the executable `rfham-qso` exists and is in $PATH
```
