---
title: c@lder Static Site Generator
date: 25-03-2022
desc: c@lder Static Site Generator
template: _templates/default
tags: []
---

# c@lder Static Site Generator

Use markdown in combination with [Tera](https://tera.netlify.app/) to generate static sites. Uses [Rocket](https://rocket.rs) to host files after building.

```cmd
USAGE:
    cargo run -- [OPTIONS] <path>

ARGS:
    <path>    path to generate site from

OPTIONS:
    -b, --build                 flag to generate site
    -h, --help                  Print help information
    -m, --minify                flag to minify the sites html, css, and js code
    -o, --output [<path>...]    path to put generated files in (default "output/")
    -r, --host                  flag to host via a webserver when done
    -V, --version               Print version information
```

Makrdown files support a yaml header
```yaml
---
title: asdf
date: 24-03-2022
desc: asdf things
template: _templates/post
tags: [asdf, things, stuff]
---
```

Files and folders prefixed with an underscore (ex: _templates) will not be rendered or copied over after building