# rust-by-example
rust by example exercises

```
install rust 
curl https://sh.rustup.rs -sSf | bash -s -- -y
```

```
then put this in dotfiles
rust() {
  name=$(basename $1 .rs)
  rustc $@ && ./$name && rm $name
}
```

```
rust 01-hello-world/001-hello-world.rs
```