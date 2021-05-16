# Mazes for Programmers

The code in this repository is inspired by the excellent book [_Mazes for Programmers_](http://www.mazesforprogrammers.com/) by Jamis Buck.

This is a hobby project just for fun, so there are dead-ends and a dearth of tests and liberal uses of `unwrap`. I would suggest not using or copying this code, but you can use it to see how someone has slowly and poorly tried to implement the various algorithms in that book in Rust.

I've been reading it in my spare time very slowly over the past few years and I come back to it irregularly. For that reason, you are unlikely to see a complete implementation here anytime soon.

## To Run

To create a maze with 5 rows and 5 columns, try this:

```sh
$ cargo run -- -r 5 -c 5
Aldous Broder
+----+----+----+----+----+
|         |         |    |
+    +----+    +----+    +
|              |         |
+    +----+    +----+    +
|    |         |         |
+    +    +----+----+    +
|    |         |    |    |
+    +----+----+    +    +
|                        |
+----+----+----+----+----+

```

You can add a distance map with the arg `--with-distance-map`

```sh
$ cargo run -- -r 5 -c 5 --with-distance-map
Aldous Broder
+----+----+----+----+----+
| 04 | 07   06   07   08 |
+    +----+    +----+----+
| 03   04   05   06   07 |
+    +----+----+----+----+
| 02   03   04   05   06 |
+    +----+    +    +    +
| 01   02 | 05 | 06 | 07 |
+    +----+----+----+    +
| 00   01 | 10   09   08 |
+----+----+----+----+----+
```

To output your maze as a PNG, use the `outfile` argument (there won't be any output):
```sh
cargo run -- -r 20 -c 20 --outfile hashgrid_aldous_broder.png
```
