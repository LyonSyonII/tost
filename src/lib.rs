extern crate proc_macro;
use proc_macro::{TokenTree, Ident, Punct, Literal, Group};

use akin::akin;

pub trait Traverser {
    akin! {
        let &name = [ident, punct, literal, group];
        let &tt_type = [Ident, Punct, Literal, Group];
        
        fn take_~*name(&mut self) -> Option<*tt_type>;
        fn take_~*name~_n(&mut self, n: usize) -> Option<Vec<*tt_type>>;
        fn take_consequent_~*name(&mut self) -> Option<Vec<*tt_type>>;
    }
}

impl Traverser for proc_macro::token_stream::IntoIter {
    akin! {
        let &name = [ident, punct, literal, group];
        let &tt_type = [Ident, Punct, Literal, Group];
        
        fn take_~*name(&mut self) -> Option<*tt_type> {
            self.next().and_then(extract_~*name)
        }
        fn take_~*name~_n(&mut self, n: usize) -> Option<Vec<*tt_type>> {
            let mut vec = Vec::with_capacity(n);
            for _ in 0..n {
                if let Some(tt) = self.next().and_then(extract_~*name) {
                    vec.push(tt)
                } else {
                    return None
                }
            }
            
            Some(vec)
        }
        fn take_consequent_~*name(&mut self) -> Option<Vec<*tt_type>> {
            let mut vec = Vec::new();
            while let Some(tt) = self.next().and_then(extract_~*name) {
                vec.push(tt)
            }

            if vec.len() > 0 {
                Some(vec)
            } else {
                None
            }
        }
    }
}

akin! {
    let &name = [ident, punct, literal, group];
    let &tt_type = [Ident, Punct, Literal, Group];
    
    fn extract_~*name(tt: TokenTree) -> Option<*tt_type> {
        match tt {
            TokenTree::*tt_type(ret) => Some(ret),
            _ => None
        }
    }
}