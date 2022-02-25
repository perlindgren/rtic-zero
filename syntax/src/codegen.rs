use crate::syntax::*;

// fn gen_task_set(task_set: &TaskSet) {
//     let mut gen = String::new();
//     gen.push(gen_init(&task_set.init));
//     gen_idle(&task_set.idle);

//     for i in task_set.tasks {
//         gen_task();
//     }
// }
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

fn gen_init(init: &Init) -> String {
    "mod init {
    }"
    .into()
}

fn ident(i: &str) -> Ident {
    Ident::new(i, Span::call_site())
}

fn shared(resources: &Vec<Resource>) -> Vec<TokenStream> {
    resources
        .iter()
        .map(|Resource { id, ty }| {
            let id = ident(id);
            let ty = TokenStream::from_str(ty).unwrap();
            quote! {#id: #ty}
        })
        .collect()
}

fn local(resources: &Vec<ResourceInit>) -> Vec<TokenStream> {
    resources
        .iter()
        .map(|ResourceInit { id, ty, .. }| {
            let id = ident(id);
            let ty = TokenStream::from_str(ty).unwrap();
            quote! { #id: &mut #ty }
        })
        .collect()
}

use std::str::FromStr;
fn gen_task(task: &Task) -> String {
    let id = ident(&task.id);

    let shared = shared(&task.shared);

    let local: Vec<_> = local(&task.local);

    let q = quote! {
        pub mod #id {
            pub struct Shared {
                #(#shared),*
            }
            pub struct Local {
                #(#local),*
            }
            pub struct Context {
                shared: Shared,
                local: Local,
            }
        }
    };
    q.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[test]
    fn test_gen_task() {
        let task_set = task_set();
        let s = gen_task(&task_set.tasks[0]);

        let path = Path::new("out.rs");
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(s.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}
