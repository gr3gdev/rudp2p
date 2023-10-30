#[derive(Debug, Clone)]
pub(crate) struct ParseType {
    pub(crate) name: syn::Ident,
    pub(crate) generics: Vec<ParseType>,
}

impl Default for ParseType {
    fn default() -> Self {
        Self {
            name: syn::Ident::new("", proc_macro2::Span::call_site()),
            generics: Default::default(),
        }
    }
}

pub(crate) fn parse_path(path: &syn::Path) -> ParseType {
    path.segments
        .iter()
        .map(|s| {
            let name = &s.ident;
            let generics_types = match &s.arguments {
                syn::PathArguments::None => vec![],
                syn::PathArguments::AngleBracketed(a) => a
                    .args
                    .iter()
                    .map(|e| match e {
                        syn::GenericArgument::Type(t) => parse_type(t),
                        _ => ParseType::default(),
                    })
                    .collect::<Vec<ParseType>>(),
                syn::PathArguments::Parenthesized(p) => p
                    .inputs
                    .iter()
                    .map(|i| parse_type(i))
                    .collect::<Vec<ParseType>>(),
            };
            ParseType {
                name: name.clone(),
                generics: generics_types.clone(),
            }
        })
        .collect::<Vec<ParseType>>()
        .first()
        .unwrap()
        .clone()
}

pub(crate) fn parse_type(field_type: &syn::Type) -> ParseType {
    match field_type {
        syn::Type::Array(a) => ParseType {
            name: syn::Ident::new("Array", proc_macro2::Span::call_site()),
            generics: vec![parse_type(&a.elem)],
        },
        syn::Type::Path(p) => parse_path(&p.path),
        _ => ParseType::default(),
    }
}

pub(crate) enum FieldType {
    Usize,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Uint128,
    Isize,
    Iint8,
    Iint16,
    Iint32,
    Iint64,
    Iint128,
    String,
    List(syn::Ident),
    Option(syn::Ident),
    Array(syn::Ident),
    Iter(syn::Ident),
    Map(ParseType, ParseType),
    Boolean,
    Struct,
    Char,
}

impl FieldType {
    #[cfg(target_pointer_width = "64")]
    pub(crate) fn get_size(&self) -> Option<i32> {
        match self {
            FieldType::Usize => Some(8),
            FieldType::Uint8 => Some(1),
            FieldType::Uint16 => Some(2),
            FieldType::Uint32 => Some(4),
            FieldType::Uint64 => Some(8),
            FieldType::Uint128 => Some(16),
            FieldType::Isize => Some(8),
            FieldType::Iint8 => Some(1),
            FieldType::Iint16 => Some(2),
            FieldType::Iint32 => Some(4),
            FieldType::Iint64 => Some(8),
            FieldType::Iint128 => Some(16),
            FieldType::String => None,
            FieldType::List(_) => None,
            FieldType::Array(_) => None,
            FieldType::Iter(_) => None,
            FieldType::Boolean => None,
            FieldType::Struct => None,
            FieldType::Option(_) => None,
            FieldType::Map(_, _) => None,
            FieldType::Char => None,
        }
    }

    #[cfg(target_pointer_width = "32")]
    pub(crate) fn get_size(&self) -> Option<i32> {
        match self {
            FieldType::Usize => Some(4),
            FieldType::Uint8 => Some(1),
            FieldType::Uint16 => Some(2),
            FieldType::Uint32 => Some(4),
            FieldType::Uint64 => Some(8),
            FieldType::Uint128 => Some(16),
            FieldType::Isize => Some(4),
            FieldType::Iint8 => Some(1),
            FieldType::Iint16 => Some(2),
            FieldType::Iint32 => Some(4),
            FieldType::Iint64 => Some(8),
            FieldType::Iint128 => Some(16),
            FieldType::String => None,
            FieldType::List(_) => None,
            FieldType::Array(_) => None,
            FieldType::Iter(_) => None,
            FieldType::Boolean => None,
            FieldType::Struct => None,
            FieldType::Option(_) => None,
            FieldType::Map(_, _) => None,
            FieldType::Char => None,
        }
    }

    pub(crate) fn find_by_type(s_type: ParseType) -> Self {
        match s_type.name.to_string().as_str() {
            "char" => Self::Char,
            "usize" => Self::Usize,
            "u8" => Self::Uint8,
            "u16" => Self::Uint16,
            "u32" => Self::Uint32,
            "u64" => Self::Uint64,
            "u128" => Self::Uint128,
            "isize" => Self::Isize,
            "i8" => Self::Iint8,
            "i16" => Self::Iint16,
            "i32" => Self::Iint32,
            "i64" => Self::Iint64,
            "i128" => Self::Iint128,
            "String" => Self::String,
            "Array" => Self::Array(
                s_type
                    .generics
                    .first()
                    .expect("Generic <T> must be set with [<T>; size]")
                    .name
                    .clone(),
            ),
            "IntoIter" => Self::Iter(
                s_type
                    .generics
                    .first()
                    .expect("Generic <T> must be set with [<T>; size]")
                    .name
                    .clone(),
            ),
            "bool" => Self::Boolean,
            "Vec" => Self::List(
                s_type
                    .generics
                    .first()
                    .expect("Generic <T> must be set with Vec<T>")
                    .name
                    .clone(),
            ),
            "VecDeque" => Self::List(
                s_type
                    .generics
                    .first()
                    .expect("Generic <T> must be set with VecDeque<T>")
                    .name
                    .clone(),
            ),
            "BinaryHeap" => Self::List(
                s_type
                    .generics
                    .first()
                    .expect("Generic <T> must be set with BinaryHeap<T>")
                    .name
                    .clone(),
            ),
            "HashSet" => Self::List(
                s_type
                    .generics
                    .first()
                    .expect("Generic <T> must be set with HashSet<T>")
                    .name
                    .clone(),
            ),
            "BTreeSet" => Self::List(
                s_type
                    .generics
                    .first()
                    .expect("Generic <T> must be set with BTreeSet<T>")
                    .name
                    .clone(),
            ),
            "LinkedList" => Self::List(
                s_type
                    .generics
                    .first()
                    .expect("Generic <T> must be set with LinkedList<T>")
                    .name
                    .clone(),
            ),
            "Option" => Self::Option(
                s_type
                    .generics
                    .first()
                    .expect("Generic <T> must be set with Option<T>")
                    .name
                    .clone(),
            ),
            "HashMap" => Self::Map(
                s_type
                    .generics
                    .get(0)
                    .expect("HashMap must have a key")
                    .clone(),
                s_type
                    .generics
                    .get(1)
                    .expect("HashMap must have values")
                    .clone(),
            ),
            "BTreeMap" => Self::Map(
                s_type
                    .generics
                    .get(0)
                    .expect("BTreeMap must have a key")
                    .clone(),
                s_type
                    .generics
                    .get(1)
                    .expect("BTreeMap must have values")
                    .clone(),
            ),
            _ => Self::Struct,
        }
    }
}
