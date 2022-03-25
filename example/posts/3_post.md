---
title: 3333
date: 25-03-2022
desc: asdf things
template: _templates/post
tags: [asdf, things, stuff]
---

# My 3rd post

My 3rd post

```rust
fn pretty_unknown_title(file: &PathBuf) -> String {
  let key = file
    .file_name()
    .unwrap()
    .to_string_lossy()
    .to_string()
    .replace(".md", "");
  let mut v = key
    .replace('_', " ")
    .split(' ')
    .map(|s| {
      let mut c = s.chars();
      match c.next() {
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
      }
    })
    .collect::<Vec<String>>();
  v.retain(|s| !s.is_empty());
  v.join(" ")
}
```