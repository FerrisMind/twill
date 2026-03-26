# Demos And Verification

This repository currently focuses on the library crate itself. There are no checked-in example binaries under `examples/`.

Use the verification commands from the root crate instead:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" fmt --all --check
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build
& "$env:USERPROFILE\.cargo\bin\cargo.exe" clippy -- -D warnings
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test
```

If you add local demo applications, keep them layered on top of `Style` and backend adapters rather than introducing UI components into Twill itself.
