# rust-by-example
rust by example exercises

```
install rust then put this in dotfiles

rust() {
  name=$(basename $1 .rs)
  rustc $@ && ./$name && rm $name
}
```

```
rust 001-hello-world.rs
```