---
title: 2222
date: 24-03-2022
desc: asdf things
template: _templates/post
tags: [asdf, things, stuff]
---

# My 2nd post

My 2nd post

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Page {
  pub title: String,
  pub date: String,
  pub desc: String,
  #[serde(rename = "type")]
  pub template: String,
  pub tags: Vec<String>,
  #[serde(skip_deserializing)]
  pub build_date: String,
  #[serde(skip_deserializing)]
  pub path: String,
  #[serde(skip_serializing, skip_deserializing)]
  pub data: String,
}
```