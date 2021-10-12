# wORM

This document covers the desired final syntax for wORM + wORM Derive.

_ _ _

```rust
let obj_vec = Query::<Object>::select()
	.join::<AnotherObj>()
	.where_eq(Object::NAME, "Test").and()
	.where_eq(Object::ACTIVE, true)
	.execute();
```
