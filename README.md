# minigrep

Classic command line search tool `grep` (`g`lobal search `r`egular `e`xpression and `p`rint).

In the simplest use case, `grep` searches a specified file for a specified string.

Reference: Chapter 12 of The Book: [chap12](https://doc.rust-lang.org/book/ch12-00-an-io-project.html)

### Build
```
$ git clone https://github.com/jiisanda/minigrep.git
$ cargo build
```

### Running Tests
```
$ cargo tests
```

### Searching
```
$ cargo run -- <string_to_search> <file_path>
```
This will get the output in the terminal. If the output is expected in output.txt file.
```
$ cargo run -- <string_to_search> <file_path> > ../output.txt
```

### For Case Sensitive Searching
For case sensitive searching, environment variables are used...
```
$ IGNORE_CASE=1 cargo run -- <string_to_search> <file_path>
```

In case of Powershell:
```
PS> $Env:IGNORE_CASE=1; cargo run -- <string_to_search> <file_path>
```
This will make `IGNORE_CASE` persist for the remainder of the shell session. It can be unset with `Remove-Item` cmdlet:
```
PS> Remove-Item Env:IGNORE_CASE
```

<br>
🙆‍♂️🙆‍♂️🙆‍♂️