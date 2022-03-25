---
title: c@lder Static Site Generator
date: 25-03-2022
desc: c@lder Static Site Generator
template: _templates/default
tags: []
---

# c@lder Static Site Generator

## Building a static site generator in a day

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Etiam tempor orci eu lobortis elementum. Vitae ultricies leo integer malesuada nunc vel risus commodo. Ornare quam viverra orci sagittis eu volutpat odio facilisis mauris. Eget mi proin sed libero enim. Vel fringilla est ullamcorper eget nulla facilisi. Proin libero nunc consequat interdum varius sit amet mattis vulputate. Scelerisque eleifend donec pretium vulputate sapien nec sagittis aliquam. Eu facilisis sed odio morbi. Eget lorem dolor sed viverra. Eget gravida cum sociis natoque penatibus. Rhoncus mattis rhoncus urna neque viverra justo nec ultrices dui. Nunc faucibus a pellentesque sit amet porttitor eget. Elementum integer enim neque volutpat ac tincidunt vitae semper quis. Scelerisque in dictum non consectetur a. Fusce id velit ut tortor pretium viverra suspendisse. Aliquet bibendum enim facilisis gravida neque. Vel orci porta non pulvinar neque laoreet suspendisse interdum.

Adipiscing diam donec adipiscing tristique. Ipsum dolor sit amet consectetur adipiscing. Amet consectetur adipiscing elit ut aliquam purus sit. Diam quam nulla porttitor massa. Nibh cras pulvinar mattis nunc sed blandit libero volutpat. Elit ut aliquam purus sit amet. Purus sit amet luctus venenatis. Lobortis mattis aliquam faucibus purus in. Non quam lacus suspendisse faucibus interdum posuere. Nunc non blandit massa enim. In vitae turpis massa sed elementum tempus egestas. Enim neque volutpat ac tincidunt. Luctus venenatis lectus magna fringilla urna porttitor rhoncus dolor purus. Neque vitae tempus quam pellentesque. Et odio pellentesque diam volutpat commodo sed egestas egestas. Nunc lobortis mattis aliquam faucibus purus. Lacus vestibulum sed arcu non odio euismod lacinia at quis. Ac feugiat sed lectus vestibulum mattis ullamcorper.

Sagittis orci a scelerisque purus semper eget duis. Integer feugiat scelerisque varius morbi enim nunc faucibus. Neque sodales ut etiam sit amet nisl. Malesuada proin libero nunc consequat. Nisl suscipit adipiscing bibendum est ultricies integer quis auctor elit. Nam libero justo laoreet sit amet cursus sit amet dictum. Velit dignissim sodales ut eu sem. At lectus urna duis convallis convallis tellus. Viverra justo nec ultrices dui sapien eget. Viverra ipsum nunc aliquet bibendum enim.

```rust
use rocket::Rocket;

pub fn host(output: &std::path::PathBuf) -> std::result::Result<(), ()> {
  let t = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap()
    .block_on(async {
      Rocket::build()
        .mount("/", rocket::fs::FileServer::from(output))
        .ignite()
        .await?
        .launch()
        .await
    });
  match t {
    Ok(_) => Ok(()),
    Err(_) => Err(()),
  }
}
```
