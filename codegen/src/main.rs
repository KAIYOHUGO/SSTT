use prettyplease::unparse;
use proc_macro2::TokenStream;
use quote::quote;
use serde_json::from_reader;
use std::{fs::File, io::Write};
use syn::parse2;

mod try_fold;
mod try_take;

fn main() {
    let file = File::open("syn.json").expect("cannot find syn.json");
    let def = from_reader(file).expect("malformed syn definitions file");
    write_out("src/try_fold.rs", try_fold::gen(&def));
    write_out("src/try_take.rs", try_take::gen(&def));
}

fn write_out(path: &str, content: TokenStream) {
    let mut file = File::create(path).unwrap();

    writeln!(
        file,
        "// Codegen file with version `{}`",
        env!("CARGO_PKG_VERSION")
    )
    .unwrap();
    writeln!(file, "// !!!Don't modify this file manually!!!",).unwrap();

    writeln!(file, "// {}{}", "-".repeat(20), "\n".repeat(3)).unwrap();

    let content = quote! {
        pub use tmp::*;
        #[allow(unused_parens, unused_variables, unreachable_patterns)]
        mod tmp{
            #content
        }
    };

    // For Debug Only
    #[cfg(feature = "raw")]
    write!(file, "{}", content.to_string()).unwrap();

    #[cfg(not(feature = "raw"))]
    write!(file, "{}", unparse(&parse2(content).unwrap())).unwrap();
}
