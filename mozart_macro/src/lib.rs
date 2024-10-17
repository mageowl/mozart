use proc_macro::TokenStream;
use proc_macro2::Span;
use proc_macro_crate::{crate_name, FoundCrate};
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident, Index};

#[proc_macro_derive(Obj)]
pub fn derive_obj(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Name of struct
    let name = input.ident;
    // Generics
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    // Get crate name
    let mozart = import_mozart();

    // Draw each child after drawing self
    let fields = fields(input.data);

    TokenStream::from(quote! {
        impl #impl_generics #mozart::obj::Obj for #name #ty_generics #where_clause {
            fn draw_children(&self, ctx: &mut #mozart::gl::GraphicsContext) {
                #mozart::maybe_draw!(self, ctx);

                // Draw children
                #(#mozart::maybe_draw_children!(&self.#fields, ctx);)*
            }

            fn update_children(&mut self, game: &mut #mozart::game::Game, delta: f32) {
                // Update children
                #(#mozart::maybe_update_children!(&mut self.#fields, game, delta);)*

                use #mozart::obj::maybe::MaybeUpdate;
                (&mut &mut #mozart::obj::maybe::Wrapper(self)).maybe_update(game, delta);
            }
        }
    })
}

#[proc_macro_derive(Obj2d)]
pub fn derive_obj_2d(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    // Name of struct
    let name = input.ident;
    // Generics
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    // Get crate name
    let mozart = import_mozart();

    TokenStream::from(quote! {
        impl #impl_generics #mozart::obj::Obj2d for #name #ty_generics #where_clause {
            fn transform(&self) -> &#mozart::math::transform::Transform {
                &self.transform
            }

            fn transform_mut(&mut self) -> &mut #mozart::math::transform::Transform {
                &mut self.transform
            }
        }
    })
}

fn fields(data: Data) -> Vec<proc_macro2::TokenStream> {
    match data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => fields
                .named
                .iter()
                .map(|f| f.ident.clone().into_token_stream())
                .collect(),
            Fields::Unnamed(ref fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| Index::from(i).into_token_stream())
                .collect(),
            Fields::Unit => Vec::new(),
        },
        _ => todo!(),
    }
}

fn import_mozart() -> Ident {
    const PARENT_NAME: &str = "mozart";
    let found_crate = crate_name(PARENT_NAME).expect("mozart is present in Cargo.toml");

    Ident::new(
        match found_crate {
            FoundCrate::Itself => PARENT_NAME,
            FoundCrate::Name(ref str) => str,
        },
        Span::call_site(),
    )
}
