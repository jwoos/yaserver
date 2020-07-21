use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};
use std::vec::Vec;

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

pub struct RouterTrie {
    // TODO add Arc and RwLock for concurrent access
    children: HashMap<String, RouterTrie>,
    path: Option<PathBuf>,
    terminator: bool,
}

impl RouterTrie {
    pub fn new() -> RouterTrie {
        return RouterTrie {
            children: HashMap::new(),
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

            if entry.metadata()?.is_dir() {
                base.children.insert(
                    partial_path.clone(),
                    RouterTrie::construct(String::from(path.to_str().unwrap()))?,
                );
            } else {
                let mut child = RouterTrie::new();
                child.path = Some(entry.path());
                child.terminator = true;

                base.children.insert(partial_path, child);
            }
        }

        return Ok(base);
    }

    pub fn find(&self, path: &mut Path) -> bool {
        let mut current = self;

        for tok in path.into_iter() {
            let os_path = current.path.as_ref();

            if os_path.is_none() {
                return false;
            }

            match tok {
                Token::Invalid => {
                    return false;
                }
                Token::Literal(literal) => {
                    let partial_path =
                        String::from(os_path.unwrap().file_name().unwrap().to_str().unwrap());

                    if *literal == partial_path {
                        if let Some(child) = current.children.get(literal) {
                            current = child;
                        } else {
                            return false;
                        }
                    }

                    return false;
                }
                Token::Pattern(_) | Token::Regex(_) => {
                    // TODO not implemented
                    return false;
                }
            }
        }

        if !current.terminator {
            if let Some(_) = current.children.get("index.html") {
                path.add(Token::Literal(String::from("index.html")));
                return true;
            }
        }

        return true;
    }
}

mod tests {
    use crate::http::path::Path;
    use crate::router::RouterTrie;

    #[test]
    fn construct() {
        let trie = RouterTrie::construct(String::from(".")).unwrap();

        let mut path = Path::parse(String::from("/a/b/c")).unwrap();
        let res = trie.find(&mut path);
        assert!(!res);

        let mut path = Path::parse(String::from("/src/main.rs")).unwrap();
        let res = trie.find(&mut path);
        assert!(res);
    }
}
