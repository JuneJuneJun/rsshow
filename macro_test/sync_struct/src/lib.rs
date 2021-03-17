use synstructure;
use quote::quote;
use proc_macro2;

struct Storage {
    pub name: String,
    pub sum: u32,
}

fn storage_write() {
    println!("I am in storage_write");
}

trait WalkFields: std::any::Any {
    fn walk_fields(&self, key: &[u8], value: &[u8]);
}

// impl WalkFields for i32 {
//     fn walk_fields(&self, _walk: &mut FnMut(&WalkFields)) {}
// }

fn walkfields_derive(s: synstructure::Structure) -> proc_macro2::TokenStream {
    let body = s.each(|bi| quote! {
        println!(#bi);
    });

    s.gen_impl(quote! {
        gen impl WalkFields for @Self {
            pub fn walk_fields(&self, key: &[u8], value: &[u8]) {
                #body
                println!("key");
                storage_write();
            }
        }
    })
}

synstructure::decl_derive!([WalkFields] => walkfields_derive);

/*
 * Test Case
 */
// #[cfg(test)]
// mod tests {
#[test]
fn good_study() {
    let stu = &Storage {
        name: "abc".to_string(),
        sum: 100,
    };
    let key = "abc".as_bytes();
    let value = "bcd".as_bytes();
    stu.walk_fields(stu, key, value);
}

#[test]
fn it_works() {
    synstructure::test_derive! {
        walkfields_derive {
            struct Storage {
                pub name: String,
                pub sum: u32,
            }
        }
        expands to {
            #[allow(non_upper_case_globals)]
            const _DERIVE_WalkFields_FOR_A: () = {
                impl WalkFields for Storage
                {
                    fn walk_fields(&self, key: &[u8], value: &[u8]) {
                        println!("123");

                    }
                }
            };
        }
    }
}
// }
