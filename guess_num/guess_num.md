# some problems

## Blocking waiting for file lock on package cache

in China add these in your cargo config, Gengerally, C://users/[your name]/.cargo (windows).
if file is not exist, just creat it with no extension.

	 http.check-revoke = false
	 [source.crates-io]
	 registry = "https://github.com/rust-lang/crates.io-index"
	 replace-with = 'ustc'
	 [source.ustc]
	 registry = "http://mirrors.ustc.edu.cn/crates.io-index"

## RLS server
RLS has an issue [https://github.com/rust-lang/rls/issues/6](https://github.com/rust-lang/rls/issues/6)
RLS not showing all methods