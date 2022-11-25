use crate::namespace::Namespace;

mod namespace;

fn main() {
    let mut namespace = Namespace::new();
    namespace.mk_dir("dfs/data-node/ok");
    namespace.mk_dir("dfs/name-node/err");
    namespace.create_small_file("dfs/text.rs");
    namespace.create_large_file("large.rs");
    dbg!(namespace);
}
