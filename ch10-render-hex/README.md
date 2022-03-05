Generate some input from the base 16 alphabet in PowerShell:

```powershell
@("Rust in Action" | % {([System.Security.Cryptography.SHA1CryptoServiceProvider]::new()).ComputeHash([Char[]]$_)} | % { "{0:x2}" -f $_}) -join $null
```

> Note: the book's use of `echo 'Rust in Action' | sha1sum` includes a new line. In PowerShell, pass the string with an
> ``"Rust in Action`n"`` to match sha1. Alternatively, compare with `echo -n`.

Generate an output `.svg` file:

```powershell
@("Rust in Action`n" | % {([System.Security.Cryptography.SHA1CryptoServiceProvider]::new()).ComputeHash([int[]][Char[]]$_)} | % { "{0:x2}" -f $_}) -join $null | % {cargo run -- $_}
```
