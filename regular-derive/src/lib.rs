extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

fn impl_regular(ast: &syn::MacroInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl Regular for #name {
            fn save(&self) {
                let path = format!("./{}s.txt", stringify!(#name).to_lowercase());
                let mut f = File::create(path).unwrap();
                f.write(self.to_s().as_bytes()).unwrap();
            }

            fn to_s(&self) -> String {
                // NOTE: assumes Struct has name field, doesn't introspect to
                //       find fields
                format!("{}", self.name)
            }
        }
    }
}

#[proc_macro_derive(Regular)]
pub fn do_the_magic_thing(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();
    // println!("{:?}", s);

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();
    // println!("{:?}", ast);

    // Build the impl
    let gen = impl_regular(&ast);
    // println!("{:?}", gen);

    // Return the generated impl
    gen.parse().unwrap()
}
