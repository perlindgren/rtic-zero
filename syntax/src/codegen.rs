use crate::syntax::*;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

fn ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
}

// TODO: Do we really need/want to mangle internal names
// They occur only in generated modules, so name clashes should not be a problem

fn mangled_ident(s: &str) -> Ident {
    Ident::new(s, Span::call_site())
    // Ident::new(&format!("__rtic_internal_{}", s), Span::call_site())
}

fn ts(s: &str) -> TokenStream {
    TokenStream::from_str(s).unwrap()
}

fn shared(resources: &Vec<Resource>, rtp: &ResourceToPriority) -> TokenStream {
    let (field_ty, field_new): (Vec<_>, Vec<_>) = resources
        .iter()
        .map(|r| {
            let ceil = rtp.get(r).unwrap();
            let ceil = ts(&format!("{}", ceil));

            let Resource { id, ty } = r;
            let id_internal = mangled_ident(id);
            let id = ident(id);
            let ty = ts(ty);

            (
                quote! {
                    pub #id: ResourceProxy<'a, #ty, #ceil>
                },
                quote! {
                    #id: ResourceProxy::new(&resources::#id_internal, priority)
                },
            )
        })
        .unzip();

    quote! {

        pub struct Shared<'a> {
            #(#field_ty),*
        }

        impl<'a> Shared<'a> {
            pub unsafe fn new(priority: &'a Priority) -> Self {
                Self {
                    #(#field_new),*
                }
            }
        }

    }
}

fn local(resources: &Vec<ResourceInit>) -> TokenStream {
    let (field_ty, (field_cell, field_new)): (Vec<_>, (Vec<_>, Vec<_>)) = resources
        .iter()
        .map(|ResourceInit { id, ty, value }| {
            let id = ident(id);
            let id_internal = ident(&format!("__rtic_internal_{}", id));
            let ty = TokenStream::from_str(ty).unwrap();
            let value = TokenStream::from_str(value).unwrap();
            (
                quote! { pub #id: &'a mut #ty },
                (
                    quote! {
                        #[allow(non_upper_case_globals)]
                        static #id_internal: RacyCell<#ty> = RacyCell::new(#value)
                    },
                    quote! {

                        #id: &mut *#id_internal.get_mut()
                    },
                ),
            )
        })
        .unzip();

    quote! {

        #(#field_cell);*;

        pub struct Local<'a> {
            #(#field_ty),*
        }

        impl<'a> Local<'a> {
             pub unsafe fn new() -> Self {
                Self {
                    #(#field_new),*
                }
            }
        }

    }
}

fn gen_init(init: &Init) -> TokenStream {
    let local = local(&init.local);

    quote! {

        pub mod init {

            use super::*;

            #local

            pub struct Context<'a> {
                pub local: Local<'a>,
            }

            pub unsafe fn run() {
                init(Context {
                    local: Local::new(),
                });
            }

            extern "Rust" {
                fn init(cx: Context);
            }
        }
    }
}

fn gen_idle(idle: &Idle, rtp: &ResourceToPriority) -> TokenStream {
    let local = local(&idle.local);
    let shared = shared(&idle.shared, rtp);

    quote! {

        pub mod idle {

            use super::*;

            #local

            #shared

            pub struct Context<'a> {
                pub local: Local<'a>,
                pub shared: Shared<'a>,
            }

            pub unsafe fn run<'a>(priority: &'a Priority) {
                idle(Context {
                    local: Local::new(),
                    shared: Shared::new(priority),
                });
            }

            extern "Rust" {
                fn idle(cx: Context) -> !;
            }
        }
    }
}

fn gen_task(task: &Task, rtp: &ResourceToPriority) -> TokenStream {
    let id = ident(&task.id);
    let shared = shared(&task.shared, rtp);
    let local = local(&task.local);

    quote! {
        mod #id {
            use super::*;

            #local

            #shared

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
    let init = gen_init(&task_set.init);

    let (idle, idle_call) = match &task_set.idle {
        Some(idle) => (gen_idle(idle, rtp), quote! {idle::run();}),
        None => (quote! {}, quote! {}),
    };

    // gen.push(gen_init(&task_set.init));
    // gen_idle(&task_set.idle);

    for task in &task_set.tasks {
        tasks.push(gen_task(task, rtp));
    }

    quote! {
        #[allow(unused_imports)]
        use crate::rtic_arch::{ResourceProxy, Resource};
        #[allow(unused_imports)]
        use mutex::Mutex;
        #[allow(unused_imports)]
        use rtic_zero::racy_cell::RacyCell;
        use cortex_m_semihosting::debug;

        #[no_mangle]
        unsafe extern "C" fn main() -> ! {

            init::run();

            #idle_call

            debug::exit(debug::EXIT_SUCCESS);

            loop {}
        }

         //#(#tasks)*

        #init

        #idle
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
        let rtp = crate::analysis::resource_ceiling(&task_set);
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
