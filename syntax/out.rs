mod app {
    use rtic::export::*;
    mod init {
        pub struct Local {
            ri2: &mut u64,
        }
        impl Local {
            fn new() -> Self {
                Local { ri2: &mut 64 }
            }
        }
    }
}
