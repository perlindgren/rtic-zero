use crate::syntax::*;

// fn gen_task_set(task_set: &TaskSet) {
//     let mut gen = String::new();
//     gen.push(gen_init(&task_set.init));
//     gen_idle(&task_set.idle);

//     for i in task_set.tasks {
//         gen_task();
//     }
// }
use proc_macro2::Ident;
use proc_macro2::Span;
use quote::quote;
// use syn::Ident;
use syn::*;

fn ident(i: &str) -> Ident {
    Ident::new(i, Span::call_site())
}
fn gen_init(init: &Init) -> String {
    "mod init {
    }"
    .into()
}

fn gen_task(task: &Task) -> String {
    let id = ident(&task.id);
    let q = quote! { mod #id {} };
    q.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_gen_task() {
        let task_set = task_set();
        let s = gen_task(&task_set.tasks[0]);

        print!("{}", s);
    }
}
