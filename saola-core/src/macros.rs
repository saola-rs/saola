#[macro_export]
macro_rules! select {
    ($struct_name:ident, $model:ident, { $($field:ident $(: $custom_type:ty)?),* $(,)? }) => {
        #[derive(Debug, Clone,::saola_core::serde::Serialize, ::saola_core::serde::Deserialize, Default)]
        #[serde(crate = "::saola_core::serde")]
        pub struct $struct_name {
            $(
                pub $field: $crate::__select_field_type!($model, $field, $($custom_type)?),
            )*
        }

        const _: () = {
            $(
                #[allow(non_camel_case_types)]
                type $field = $model::fields::$field;
            )*
        };

        impl ::saola_core::builder::SelectStruct for $struct_name {
            fn selections() -> Vec<::saola_core::query_core::Selection> {
                vec![
                    $(::saola_core::builder::push_selection_with_type::<$crate::__select_field_type!($model, $field, $($custom_type)?)>(stringify!($field))),*
                ]
            }
        }

        impl ::saola_core::builder::GetSelections for $struct_name {
            fn get_selections() -> Vec<::saola_core::query_core::Selection> {
                <Self as ::saola_core::builder::SelectStruct>::selections()
            }
        }

        impl ::saola_core::builder::FromResponseIr for $struct_name {
            fn from_ir(item: ::saola_core::query_core::response_ir::Item) -> ::saola_core::Result<Self> {
                let mut map = match item {
                    ::saola_core::query_core::response_ir::Item::Map(m) => m,
                    ::saola_core::query_core::response_ir::Item::Ref(r) => match r.as_ref() {
                        ::saola_core::query_core::response_ir::Item::Map(m) => m.clone(),
                        _ => return Err(::saola_core::Error::RuntimeError("Expected map in response ref".to_string())),
                    },
                    _ => return Err(::saola_core::Error::RuntimeError(format!("Expected map in response, got {:?}", item))),
                };

                Ok(Self {
                    $(
                        $field: map.shift_remove(stringify!($field))
                            .ok_or_else(|| ::saola_core::Error::RuntimeError(format!("Missing field: {}", stringify!($field))))
                            .and_then(::saola_core::builder::FromResponseIr::from_ir)?,
                    )*
                })
            }
        }

        impl<U: ::saola_core::builder::SelectStruct> ::saola_core::builder::SelectAsTransition<U> for $struct_name {
            type Output = U;
        }
    };
}

#[macro_export]
macro_rules! __select_field_type {
    ($model:ident, $field:ident, ) => {
        $model::fields::$field
    };
    ($model:ident, $field:ident, $custom_type:ty) => {
        $custom_type
    };
}
