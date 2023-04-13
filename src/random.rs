use rand::{prelude::ThreadRng, seq::SliceRandom};

struct TypeDef {
    name: &'static str,
    generics: u8,
}
impl TypeDef {
    const fn new(name: &'static str, generics: u8) -> Self {
        Self {name, generics}
    }
}

const RECURSION_LIMIT: u8 = 1;
const ARRAY_CHANCE: f64 = 0.1;
const TYPES: &[TypeDef] = &[
    TypeDef::new("_", 0),
    TypeDef::new("bool", 0),
    TypeDef::new("char", 0),
    TypeDef::new("i8", 0),
    TypeDef::new("i16", 0),
    TypeDef::new("i32", 0),
    TypeDef::new("i64", 0),
    TypeDef::new("isize", 0),
    TypeDef::new("u8", 0),
    TypeDef::new("u16", 0),
    TypeDef::new("u32", 0),
    TypeDef::new("u64", 0),
    TypeDef::new("usize", 0),
    TypeDef::new("f32", 0),
    TypeDef::new("f64", 0),
    TypeDef::new("&str", 0),
    TypeDef::new("String", 0),
    TypeDef::new("()", 0),
    TypeDef::new("Box", 1),
    TypeDef::new("Vec", 1),
    TypeDef::new("HashSet", 1),
    TypeDef::new("Result", 2),
    TypeDef::new("HashMap", 2),
];

pub fn random_type(rng: &mut ThreadRng, recursion_depth: u8) -> String {
    let mut rtype = TYPES.choose(rng).unwrap();

    if recursion_depth >= RECURSION_LIMIT {
        while rtype.generics != 0 {
            rtype = TYPES.choose(rng).unwrap();
        }
    }

    let mut generics = String::new();

    if rtype.generics > 0 {
        generics.push('<');
        for _ in 0..rtype.generics {
            generics.push_str(&random_type(rng, recursion_depth+1));
            generics.push_str(", ");
        }
        generics.push_str(&random_type(rng, recursion_depth+1));
        generics.push('>');
    }
    
    format!("{}{generics}", rtype.name)
}
