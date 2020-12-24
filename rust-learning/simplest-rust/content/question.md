# About rust

- Start Date: 20191014

### Q1:  T and compare 

```rust
fn lastest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut lastest = list[0];
    for &item in list.iter() {
        if item > lastest {
            lastest = item;
        }
    }
    lastest
}

fn main() {
    let num_list = vec![3, 50, 55, 56, 7, 89, 20];
    let result = lastest::<i32>(&num_list);
    println!("lastest item is {}", result);
    let char_list = vec!['k', 'm', 'g', 't', 'h'];
    let result = lastest(&char_list);
    println!("lastest item is {}", result);
    let f_list = vec![3.0, 50.0, 55.0, 56.1, 7.1, 89.7, 20.1];
    let result = lastest(&f_list);
    println!("lastest item is {}", result);
    let str_list = vec!["ak", "lm", "kg", "zth", "zmh"];
    let result = lastest(&str_list);
    println!("lastest item is {}", result);
}
```
---

### Q2 How to find a file. 

-  entry_path
-   std::fs::DirEntry
-   
```rust
use std::fs;
use std::os::unix::fs::DirEntryExt;

if let Ok(entries) = fs::read_dir(".") {
    for entry in entries {
        if let Ok(entry) = entry {
            // Here, `entry` is a `DirEntry`.
            println!("{:?}: {}", entry.file_name(), entry.ino());
        }
    }
}
```

```rust



```

---

### Q3: How to connect sqlite3. 
```
use rusqlite::{Connection, Result, NO_PARAMS};

fn main() -> Result<()> {
    let mut conn = Connection::open("cats.db")?;

    successful_tx(&mut conn)?;

    let res = rolled_back_tx(&mut conn);
    assert!(res.is_err());

    Ok(())
}
```
- create:

```
use rusqlite::{Connection, Result};
use rusqlite::NO_PARAMS;

fn main() -> Result<()> {
    let conn = Connection::open("cats.db")?;

    conn.execute(
        "create table if not exists cat_colors (
             id integer primary key,
             name text not null unique
         )",
        NO_PARAMS,
    )?;
    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             name text not null,
             color_id integer not null references cat_colors(id)
         )",
        NO_PARAMS,
    )?;

    Ok(())
}
```

- what about blog?

### Q4: How to config a application. 
```
//Parse TOML into your own structs using Serde.
#[macro_use]
extern crate serde_derive;
extern crate toml;

use toml::de::Error;
use std::collections::HashMap;

#[derive(Deserialize)]
struct Config {
    package: Package,
    dependencies: HashMap<String, String>,
}

#[derive(Deserialize)]
struct Package {
    name: String,
    version: String,
    authors: Vec<String>,
}

fn main() -> Result<(), Error> {
    let toml_content = r#"
          [package]
          name = "your_package"
          version = "0.1.0"
          authors = ["You! <you@example.org>"]

          [dependencies]
          serde = "1.0"
          "#;

    let package_info: Config = toml::from_str(toml_content)?;

    assert_eq!(package_info.package.name, "your_package");
    assert_eq!(package_info.package.version, "0.1.0");
    assert_eq!(package_info.package.authors, vec!["You! <you@example.org>"]);
    assert_eq!(package_info.dependencies["serde"], "1.0");

    Ok(())
}
```

### Q5: Error handle
io::Error && reqwest::Error   in same file how to do it which return. ***usring error-chain***
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### Q6: Option
```rust
enum Option<T> {
    None,
    Some(T),
}
```
