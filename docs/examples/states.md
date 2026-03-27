# States Example

This example isolates state and attribute layers such as `hover`, `focus_visible`, `disabled`, `data_state`, and `aria_state`.

- File: `examples/03_states.rs`
- Run:

```powershell
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --example states
```

- Expected output: a terminal summary of the nested state styles stored on the composed `Style`.

Why this exists:
stateful styling is one of the biggest differences between raw tokens and a useful UI style system, so it deserves its own single-purpose example.
