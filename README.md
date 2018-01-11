# rust-by-example
rust by example exercises

```
install rust then put this in dotfiles
run like --- rust 01-hello-world.rs

rust() {
  name=$(basename $1 .rs)
  rustc $@ && ./$name && rm $name
}

```