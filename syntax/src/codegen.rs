use crate::syntax::*;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

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

fn gen_init(init: &Init) -> TokenStream {
    let local: Vec<_> = local(&init.local);

    quote! {
        mod init {

            pub struct Local {
                #(#local),*
            }

            pub struct Context {
                local: Local,
            }
        }
    }
}

fn gen_task(task: &Task) -> TokenStream {
    let id = ident(&task.id);

    let shared = shared(&task.shared);

    let local: Vec<_> = local(&task.local);

    quote! {
        mod #id {
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
    }
}

fn gen_task_set(task_set: &TaskSet) -> TokenStream {
    let mut tasks = vec![];

    // gen.push(gen_init(&task_set.init));
    // gen_idle(&task_set.idle);

    for task in &task_set.tasks {
        tasks.push(gen_task(task));
    }

    quote! {
        use 
        mod app {

            #(#tasks)*

        }
    }
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
        let s = gen_task_set(&task_set);

        let path = Path::new("out.rs");
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };

        match file.write_all(s.to_string().as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }
}
