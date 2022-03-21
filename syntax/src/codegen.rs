use crate::syntax::*;
use std::str::FromStr;

use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;

use crate::analysis::ResourceToPriority;

// some helpers

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
            let id_internal = mangled_ident(&id);
            let id = ident(id);
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

            pub unsafe fn run() -> Shared {
                init(Context {
                    local: Local::new(),
                })
            }

            extern "Rust" {
                fn init(cx: Context) -> Shared;
            }
        }
    }
}

fn gen_idle(idle: &Option<Idle>, rtp: &ResourceToPriority) -> (TokenStream, TokenStream) {
    if let Some(idle) = idle {
        let local = local(&idle.local);
        let shared = shared(&idle.shared, rtp);

        (
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
                        fn idle(cx: Context);
                    }
                }
            },
            quote! {
                // idle runs at priority 0
                let priority = Priority::new(0);
                idle::run(&priority);
            },
        )
    } else {
        (quote! {}, quote! {})
    }
}

fn gen_shared(shared: &Vec<Resource>, rtp: &ResourceToPriority) -> TokenStream {
    let (field_res, (field_struct, field_move)): (Vec<_>, (Vec<_>, Vec<_>)) = shared
        .iter()
        .map(|r| {
            let ceil = rtp.get(r).unwrap();
            let ceil = ts(&format!("{}", ceil));

            let Resource { id, ty } = r;
            let id_internal = mangled_ident(&id);
            let id = ident(&id);

            let ty = ts(ty);
            (
                quote! {
                    #[allow(non_upper_case_globals)]
                    pub static #id_internal: Resource<#ty, #ceil> = Resource::new();
                },
                (
                    quote! {
                        pub #id: #ty
                    },
                    quote! {
                        #id_internal.write_maybe_uninit(shared.#id);
                    },
                ),
            )
        })
        .unzip();

    quote! {

        pub struct Shared {
            #(#field_struct),*
        }

        mod resources {
            use super::*;

            #(#field_res)*

            pub unsafe fn move_shared(shared: Shared) {
                #(#field_move)*
            }

        }
    }
}

fn gen_task(task: &Task, device: &Ident, rtp: &ResourceToPriority) -> TokenStream {
    let id = ident(&task.id);
    let shared = shared(&task.shared, rtp);
    let local = local(&task.local);
    let interrupt = ident(&task.binds);
    let priority = task.priority;

    quote! {
        pub mod #id {
            use super::*;

            #local

            #shared

            pub struct Context<'a> {
                pub shared: Shared<'a>,
                pub local: Local<'a>,
            }

            pub fn pend() {
                rtic_arch::pend(#device::Interrupt::#interrupt)
            }

            pub unsafe fn run<'a>(priority: &'a Priority) {
                #id(Context {
                    local: Local::new(),
                    shared: Shared::new(priority),
                });
            }

            extern "Rust" {
                fn #id(cx: Context);
            }

            #[allow(non_snake_case)]
            #[no_mangle]
            unsafe fn #interrupt() {
                let priority = Priority::new(#priority);
                run(&priority);
            }
        }
    }
}

fn gen_tasks(task_set: &TaskSet, rtp: &ResourceToPriority) -> (TokenStream, TokenStream) {
    let mut tasks: Vec<TokenStream> = vec![];
    let mut interrupt_init = vec![];

    let device = ident(&task_set.device);

    for task in &task_set.interrupts {
        tasks.push(gen_task(task, &device, rtp));

        let priority = task.priority;
        let interrupt = ident(&task.binds);

        interrupt_init.push(quote! {
            rtic_arch::unmask(#device::Interrupt::#interrupt);
            rtic_arch::set_priority(#device::Interrupt::#interrupt, #priority);
        })
    }

    (
        quote! {
            #(#tasks)*
        },
        quote! {
            #(#interrupt_init)*
        },
    )
}

fn gen_task_set(task_set: &TaskSet, rtp: &ResourceToPriority) -> TokenStream {
    let init = gen_init(&task_set.init);

    let resources = gen_shared(&task_set.shared, rtp);

    let (idle, idle_call) = gen_idle(&task_set.idle, rtp);

    let (tasks, interrupt_init) = gen_tasks(&task_set, rtp);

    quote! {
        #[allow(unused_imports)]
        use crate::rtic_arch::{self, ResourceProxy, Resource};
        #[allow(unused_imports)]
        use mutex::Mutex;
        #[allow(unused_imports)]
        use rtic_zero::{racy_cell::RacyCell, priority::Priority};
        #[allow(unused_imports)]
        use core::mem::MaybeUninit;

        use cortex_m_semihosting::debug;

        #[no_mangle]
        unsafe extern "C" fn main() -> ! {

            rtic_arch::interrupt_free(|_| {

                let shared = init::run();

                resources::move_shared(shared);

                #interrupt_init

            });


            #idle_call

            debug::exit(debug::EXIT_SUCCESS);

            loop {}
        }

        #resources

        #init

        #idle

        #tasks
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::syntax::test::task_set;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    #[test]
    fn test_gen_task() {
        let task_set = task_set();
        let rtp = crate::analysis::resource_ceiling(&task_set);
        let s = gen_task_set(&task_set, &rtp);

        let path = Path::new("rtic.json");
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

    #[test]
    fn test_gen_task_files() {
        let path = Path::new("rtic.json");
        let task_set = crate::io::load_tasks(&path);

        let rtp = crate::analysis::resource_ceiling(&task_set);
        let s = gen_task_set(&task_set, &rtp);

        let path = Path::new("../app/src/rtic_zero_codegen.rs");
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
