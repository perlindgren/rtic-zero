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

// fn local(resources: &Vec<ResourceInit>) -> Vec<TokenStream> {
//     resources
//         .iter()
//         .map(|ResourceInit { id, ty, .. }| {
//             let id = ident(id);
//             let ty = TokenStream::from_str(ty).unwrap();
//             quote! { #id: &mut #ty }
//         })
//         .collect()
// }

fn local(resources: &Vec<ResourceInit>) -> TokenStream {

    
    let ty_fields: Vec<_> = resources
        .iter()
        .map(|ResourceInit { id, ty, .. }| {
            let id = ident(id);
            let ty = TokenStream::from_str(ty).unwrap();
            quote! { #id: &mut #ty }
        })
        .collect();

    let imp_fields: Vec<_> = resources
        .iter()
        .map(|ResourceInit { id, value, .. }| {
            let id = ident(id);
            let value = TokenStream::from_str(value).unwrap();
            quote! { #id: &mut #value }
        })
        .collect();

    quote! {
        pub struct Local {
            #(#ty_fields),*
        }

        impl Local {
            fn new() -> Self {
                Local {
                    #(#imp_fields),*
                }
            }
        }
    }
}

fn gen_init(init: &Init, rtp: &ResourceToPriority) -> TokenStream {
    let local = local(&init.local);

    quote! {
        mod init {
           #local
        }
    }
}

fn gen_task(task: &Task, rtp: &ResourceToPriority) -> TokenStream {
    let id = ident(&task.id);

    let shared = shared(&task.shared);

    let local = local(&task.local);

    // let local_resources = local_resources(&task.local);

    quote! {
        mod #id {
            pub struct Shared {
                #(#shared),*
            }

            #local

            pub struct Context {
                shared: Shared,
                local: Local,
            }
        }
    }
}

use crate::analysis::ResourceToPriority;

fn gen_task_set(task_set: &TaskSet, rtp: &ResourceToPriority) -> TokenStream {
    let mut tasks = vec![];
    let init = gen_init(&task_set.init, rtp);

    // gen.push(gen_init(&task_set.init));
    // gen_idle(&task_set.idle);

    for task in &task_set.tasks {
        tasks.push(gen_task(task, rtp));
    }

    quote! {
        mod app {
            use rtic::export::*;

            //#(#tasks)*

            #init

        }
    }
}

fn gen_shared() {}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[test]
    fn test_gen_task() {
        let task_set = task_set();
        let rtp = crate::analysis::resource_ceiling(&task_set.tasks);
        let s = gen_task_set(&task_set, &rtp);

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
