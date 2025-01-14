use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use crate::args::EnumAttrArgs;

use super::EnumInfo;

impl EnumInfo {
    /// Generate C API and Lua FFI.
    pub fn generate(&self, attr_args: EnumAttrArgs) -> TokenStream {
        // Original enum source code
        let source = &self.source;

        let self_ident = format_ident!("{}", self.name);
        let start_index = attr_args.start_index();
        let repr_type = if let Some(repr_type) = attr_args.repr() {
            repr_type
        } else {
            let max_discriminant = self.variants.max_discriminant(start_index);

            if max_discriminant > u32::MAX as u64 {
                "u64"
            } else if max_discriminant > u16::MAX as u64 {
                "u32"
            } else if max_discriminant > u8::MAX as u64 {
                "u16"
            } else {
                "u8"
            }
            .into()
        };
        let repr_type_ident = format_ident!("{repr_type}");

        let variant_pairs = self.variants.get_pairs(start_index);
        let constant_items: Vec<_> = variant_pairs
            .iter()
            .map(|(name, _)| {
                let const_ident = format_ident!("{}_{name}", self.name);
                let variant_ident = format_ident!("{name}");

                quote! {
                    #[no_mangle]
                    pub const #const_ident: #repr_type_ident = #self_ident::#variant_ident.value();
                }
            })
            .collect();
        let value_items: Vec<_> = variant_pairs
            .iter()
            .map(|(name, d)| {
                let variant_ident = format_ident!("{name}");

                quote! {
                    Self::#variant_ident => #d as #repr_type_ident,
                }
            })
            .collect();

        let to_string_c_ident = format_ident!("{}_ToString", self.name);

        if attr_args.gen_lua_ffi() {
            self.generate_ffi(&attr_args);
        }

        quote! {
            #[repr(#repr_type_ident)]
            #source

            impl #self_ident {
                pub const fn value(&self) -> #repr_type_ident {
                    match self {
                        #(#value_items)*
                    }
                }
            }

            impl ToString for #self_ident {
                fn to_string(&self) -> String {
                    format!("{:?}", self)
                }
            }

            #(#constant_items)*

            #[no_mangle]
            pub extern "C" fn #to_string_c_ident(this: #self_ident) -> *const libc::c_char {
                let res = this.to_string();

                static_string!(res)
            }
        }
    }
}
