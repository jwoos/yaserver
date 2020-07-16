use std::collections::HashMap;
use std::vec::Vec;
use std::sync::{RwLock, Arc};
use std::fs;
use std::io;
use std::path::PathBuf;

use crate::http::path::{Path, Token};
use ascii::{AsciiChar, AsciiStr};

/*
 *pub struct Router {
 *    cache: HashMap<String, String>,
 *    static_cache: HashMap<String, Vec<u8>>,
 *}
 *
 *impl Router {
 *    pub fn new(path: &AsciiStr) -> Router {
 *        return Router{};
 *    }
 *}
 */

// TODO add flag to show validity
pub struct RouterTrie {
    children: Arc<RwLock<HashMap<String, RouterTrie>>>,
    path: Option<PathBuf>,
    terminator: bool,
}

impl RouterTrie {
    pub fn new() -> RouterTrie {
        return RouterTrie {
            children: Arc::new(RwLock::new(HashMap::new())),
            path: None,
            terminator: false,
        };
    }

    pub fn construct(base_directory: String) -> io::Result<RouterTrie> {
        let mut base = RouterTrie::new();
        base.terminator = false;

        let dir_iter = fs::read_dir(base_directory)?;

        for dir in dir_iter {
            let entry = dir?;
            let path = entry.path();
            let partial_path = String::from(entry.file_name().to_str().unwrap());
            let mut children = base.children.write().unwrap();

            if entry.metadata()?.is_dir() {
                children.insert(partial_path.clone(), RouterTrie::construct(partial_path)?);
            } else {
                let mut child = RouterTrie::new();
                child.path = Some(entry.path());
                child.terminator = true;

                children.insert(partial_path, child);
            }
        }

        return Ok(base);
    }

    pub fn find(&self, path: &Path) -> bool {
        let mut current = self;

        for tok in path {
            let path = current.path.as_ref();

            if path.is_none() {
                return false;
            }

            match tok {
                Token::Invalid => {
                    return false;
                },
                Token::Literal(literal) => {
                    let partial_path = String::from(path.unwrap().file_name().unwrap().to_str().unwrap());
                    if *literal == partial_path {
                        let children_lock = current.children.clone();
                        let children = children_lock.read().unwrap();
                        if let Some(child) = children.get(literal) {
                            current = child;
                        } else {
                            return false;
                        }
                    }

                    return false;
                },
                Token::Pattern(_) | Token::Regex(_) => {
                    // TODO not implemented
                    return false;
                }
            }
        }

        if !current.terminator {
            let children_lock = current.children.clone();
            let children = children_lock.read().unwrap();
            if let Some(_) = children.get("index.html") {
                return true;
            }
        }

        // return something else
        return true;
    }
}

mod tests {
    use crate::router::RouterTrie;
    use crate::http::path::Path;

    #[test]
    fn construct() {
        let trie = RouterTrie::construct(".").unwrap();
        let res = trie.find(Path::parse("/a/b/c").unwrap());
    }
}
