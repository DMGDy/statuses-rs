# statuses-rs

## what does it do
Right now it only gets the machines RAM.
Additionally now gets connection ssid, connection strength as ascii, and connection indicator icon.

```
--mem
--mem -i
--wifi-info
--connection-status
```
can look through source to figure out not too hard

## why

Most times people implement system status widgets they are lazy and write the tool in a scripting language because
it is easy. Writing such a small tool can be easily done in a compiled language like Rust. This is that.


## How to build

`cargo build --release`

## How to use
left to user as exercise, might have to change some stuff
