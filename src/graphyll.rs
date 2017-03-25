extern crate yaml_rust;
use yaml_rust::{YamlLoader, YamlEmitter};


struct BuildGraph {
    
}
pub fn test() {
    let s =
"
nodes:
  - syscalls:
      - kramdown src/post1.md > dest/post1.html
    dependencies:
      - src/post1.md
    targets:
      - dest/post1.html
  - syscalls:
      - kramdown src/post2.md > dest/post2.html
    dependencies:
      - src/post2.md
    targets:
      - dest/post2.html
";
    let docs = YamlLoader::load_from_str(s).unwrap();
    let doc = &docs[0];
    println!("{:?}", doc);
}
