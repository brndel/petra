use proc_macro::TokenStream;
use syn::{Error, Ident};

use crate::column::Column;

pub fn impl_table(ast: syn::DeriveInput) -> Result<TokenStream, Error> {
  let name = &ast.ident;

  let mut primary_field = None;
  let mut columns = vec![];

  match ast.data {
    syn::Data::Struct(data) => match &data.fields {
      syn::Fields::Named(fields) => {
        for field in fields.named.iter() {
          let column = Column::parse(field.to_owned(), &mut primary_field)?;

          columns.push(column);
        }
      }
      _ => Err(Error::new_spanned(name, "Expected Named Fields"))?,
    },
    _ => Err(Error::new_spanned(name, "Expected Struct"))?,
  }

  let primary_field =
    primary_field.ok_or_else(|| Error::new_spanned(name, "No primary field set"))?;

  let insert_struct_quote = insert_quote(name, &columns);
  let table_impl_quote = impl_quote(name, &columns, &primary_field);
  let link_impl_quote = link_quote(name, &columns);


  Ok(
    quote! {
      #insert_struct_quote

      #table_impl_quote

      #link_impl_quote
    }
    .into(),
  )
}

fn insert_quote(
  name: &Ident,
  columns: &Vec<Column>,
) -> quote::__private::TokenStream {
  let insert_name = &format_ident!("{}Insert", name);
  let columns = columns.iter().filter(|c| !c.is_primary());
  let idents = columns.clone().map(|c| c.ident.clone());
  let fields = columns.clone().map(|c| c.field.clone());

  let column_names = columns.clone().map(|c| c.ident.to_string());
  let placeholder_names = column_names.clone().map(|name| format!(":{}", name));
  let placeholder_names2 = placeholder_names.clone();

  quote!(
    #[derive(serde::Deserialize)]
    pub struct #insert_name {
      #(#fields,)*
    }

    impl data::Insertable<#name> for #insert_name {

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

      fn bind(self, statement: &mut data::sqlite::Statement) -> data::sqlite::Result<()> {
        statement.bind_iter::<_, (_, data::sqlite::Value)>([
          #(
            (#placeholder_names2, self.#idents.into()),
          )*
        ])
      }
    }
  )
}

fn impl_quote(
  name: &Ident,
  columns: &Vec<Column>,
  primary_field: &Ident,
) -> quote::__private::TokenStream {
  let table_name = name.to_string();
  let primary_name = primary_field.to_string();

  let idents = columns.iter().map(|c| c.ident.clone());
  let ident_names = idents.clone().map(|i| i.to_string());

  quote!(
    #[automatically_derived]
    impl Table for #name {
      fn table_name() -> &'static str {
        #table_name
      }

      fn primary_name() -> &'static str {
        #primary_name
      }

      fn get_columns() -> Vec<data::Column> {
        vec![
          #(#columns,)*
        ]
      }

      fn read(statement: &data::sqlite::Statement) -> data::sqlite::Result<Self> {
        Ok(Self {
          #(
            #idents: statement.read(#ident_names)?,
          )*
        })
      }

      fn get_primary(&self) -> i64 {
        self.#primary_field
      }
    }
  )
}

fn link_quote(
  name: &Ident,
  columns: &Vec<Column>
) -> quote::__private::TokenStream {
  let filter_columns = columns.iter().filter(|c| c.modifier.link.is_some());

  let name = filter_columns.clone().map(|_| name.clone());
  let filter_types = filter_columns.clone().map(|c| c.modifier.link.to_owned().unwrap());
  let filter_idents = filter_columns.clone().map(|c| c.ident.to_string());

  quote!(
    #(
    impl data::Link<#filter_types> for #name {
      fn get_name() -> &'static str {
        #filter_idents
      }
    }
    )*
  )
}