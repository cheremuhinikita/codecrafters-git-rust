use std::{fs, path::Path};

use super::hash_object::HashObject;
use crate::{
    obj::{
        store,
        tree::{Tree, TreeEntry, TreeEntryMode},
        Object,
    },
    Result,
};

pub struct WriteTree;

impl WriteTree {
    pub fn inner(root: impl AsRef<Path>) -> Result<String> {
        let mut tree_entries = Vec::<TreeEntry>::new();

        for entry in fs::read_dir(root)? {
            let path = entry?.path();

            let name = path.file_name().unwrap().to_str().unwrap();

            if name == ".git" {
                continue;
            }

            let tree_entry = if path.is_dir() {
                let sha = WriteTree::inner(&path)?;

                TreeEntry::new(TreeEntryMode::Tree, name, sha)
            } else {
                let sha = HashObject::new(&path.to_string_lossy()).inner()?;

                TreeEntry::new(TreeEntryMode::Blob, name, sha)
            };

            tree_entries.push(tree_entry);
        }

        let mut tree = Tree::new(tree_entries);
        tree.sort_entries();

        let object = Object::from_tree(tree);

        store::write(&object)
    }

    pub fn exec(self) -> Result<()> {
        let sha = WriteTree::inner(".")?;

        println!("{}", sha);

        Ok(())
    }
}
