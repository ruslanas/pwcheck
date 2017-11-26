Pwned password checker
======================

Tool to check passwords against a local copy of passwords
[exposed](https://www.troyhunt.com/introducing-306-million-freely-downloadable-pwned-passwords/) in data breaches.

Build
-----

```shell
cargo build --release
```

Install
-------

Create symlink to `target/release/pwcheck` in any directory on executable search PATH.

Usage example
-------------

```shell
pwcheck pwned-passwords-1.0.txt p@55word
```

Output
------

```
306259512 password hashes in file.
SHA1 3558288C5E578A05536EC28C4C1613AE865FE75C
Found at line: 63563447
```

[Download Pwned Passwords](https://haveibeenpwned.com/Passwords)