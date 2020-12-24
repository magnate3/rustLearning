# Option

### std::option::Option

```
pub enum Option<T> {
    None,
    Some(T),
}
```

## fn

- is_some() is_none()

- contains()

```
#![feature(option_result_contains)]

let x: Option<u32> = Some(2);
assert_eq!(x.contains(&2), true);

let x: Option<u32> = Some(3);
assert_eq!(x.contains(&2), false);

let x: Option<u32> = None;
assert_eq!(x.contains(&2), false);
```

- as_ref()
***Converts from &Option<T> to Option<&T>.***

- map

***ToOwned::to_owned***

