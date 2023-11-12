use proc_macro::TokenStream;
use syn::{Error, Expr, Ident, Lit, ExprLit};

use crate::table::Column;

pub fn impl_table(ast: syn::DeriveInput) -> Result<TokenStream, Error> {
    let mut table_name = None;

    for attr in ast.attrs {
        if let Some(ident) = attr.path().get_ident() {
            if ident.to_string() == "table_name" {
                let expr: Expr = attr.parse_args()?;

                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(name),
                    ..
                }) = expr
                {
                    table_name = Some(name.value())
                }
            }
        }
    }

    let name = &ast.ident;
    let table_name = table_name.unwrap_or_else(|| name.to_string());

    let mut primary = None;
    let mut columns = vec![];

    match ast.data {
        syn::Data::Struct(data) => match &data.fields {
            syn::Fields::Named(fields) => {
                for field in fields.named.iter() {
                    let column = Column::parse(field.to_owned(), &mut primary)?;

                    columns.push(column);
                }
            }
            _ => Err(Error::new_spanned(name, "Expected Named Fields"))?,
        },
        _ => Err(Error::new_spanned(name, "Expected Struct"))?,
    }

    let primary = primary.ok_or_else(|| Error::new_spanned(name, "No primary field set"))?;

    let table_impl_quote = impl_quote(name, &table_name, &columns, &primary);
    let insert_impl_quote = insert_quote(name, &columns);
    let columns_impl_quote = columns_quote(name, &columns);
    let link_impl_quote = link_quote(name, &columns);
    let read_impl_quote = read_quote(name, &columns, &primary);

    Ok(quote! {
      #insert_impl_quote

      #columns_impl_quote

      #table_impl_quote

      #link_impl_quote

      #read_impl_quote
    }
    .into())
}

fn insert_quote(name: &Ident, columns: &Vec<Column>) -> quote::__private::TokenStream {
    let columns = columns.iter();
    let idents = columns.clone().map(|c| c.ident.clone());

    let column_names = columns.clone().map(|c| c.ident.to_string());
    let placeholder_names = column_names.clone().map(|name| format!(":{}", name));
    let placeholder_names2 = placeholder_names.clone();

    quote!(
      impl mensula::Insertable<#name> for #name {

        fn get_column_names() -> &'static [&'static str] {
          &[
            #(#column_names,)*
          ]
        }

        fn get_placeholder_names() -> &'static [&'static str] {
          &[
            #(#placeholder_names,)*
          ]
        }

        fn bind(self, statement: &mut mensula::sqlite::Statement) -> mensula::sqlite::Result<()> {
          statement.bind_iter::<_, (_, mensula::sqlite::Value)>([
            #(
              (#placeholder_names2, Into::<mensula::FilterValue>::into(self.#idents).into()),
            )*
          ])
        }
      }
    )
}

fn columns_quote(name: &Ident, columns: &Vec<Column>) -> quote::__private::TokenStream {
    let columns = columns.iter();
    let column_names = columns.clone().map(|c| c.ident.clone());

    quote!(
      impl #name {
        #(
          pub fn #column_names() -> mensula::Column<Self> {
            #columns
          }
        )*
      }
    )
}

fn impl_quote(
    name: &Ident,
    table_name: &str,
    columns: &Vec<Column>,
    primary: &Column,
) -> quote::__private::TokenStream {
    // let primary_ident = &primary.ident;

    quote!(
      #[automatically_derived]
      impl Table for #name {
        fn table_name() -> &'static str {
          #table_name
        }

        fn primary_column() -> mensula::Column<Self> {
          #primary
        }

        fn get_columns() -> Vec<mensula::Column<Self>> {
          vec![
            #(#columns,)*
          ]
        }

        // fn get_primary(&self) -> mensula::Key {
        //   self.#primary_ident.clone()
        // }
      }
    )
}

fn link_quote(name: &Ident, columns: &Vec<Column>) -> quote::__private::TokenStream {
    let filter_columns = columns.iter().filter_map(|c| {
        if let Some(reference) = &c.modifier.reference {
            if reference.is_link {
                return Some((&c.ident, &reference.ty));
            }
        }
        None
    });

    let name = filter_columns.clone().map(|_| name.clone());
    let link_types = filter_columns.clone().map(|(_, ty)| ty);
    let link_names = filter_columns.clone().map(|(ident, _)| ident.to_string());

    quote!(
      #(
      #[automatically_derived]
      impl mensula::Link<#link_types> for #name {
        fn link_name() -> &'static str {
          #link_names
        }
      }
      )*
    )
}

fn read_quote(
    name: &Ident,
    columns: &Vec<Column>,
    primary: &Column,
) -> quote::__private::TokenStream {
    let primary_name = primary.ident.to_string();

    let idents = columns.iter().map(|c| &c.ident);
    let names = idents.clone().map(|i| i.to_string());
    let names2 = names.clone();

    quote!(
      #[automatically_derived]
      impl mensula::Readable<mensula::Key> for #name {
        fn get_column_names() -> Option<&'static [&'static str]> {
          Some(&[#primary_name])
        }

        fn read(statement: &mensula::sqlite::Statement) -> Option<mensula::Key> {
          Some(statement.read(#primary_name).ok()?)
        }
      }

      #[automatically_derived]
      impl mensula::Readable<Self> for #name {
        fn get_column_names() -> Option<&'static [&'static str]> {
          Some(&[
            #(#names,)*
          ])
        }

        fn read(statement: &mensula::sqlite::Statement) -> Option<Self> {
          Some(Self {
            #(#idents: statement.read(#names2).ok()?,)*
          })
        }
      }
    )
}
