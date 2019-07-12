pub use google_field_selector_derive::FieldSelector;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, LinkedList, VecDeque};
/// FieldSelector provides a google api compatible field selector. This value can
/// be provided in the "fields" attribute of many google api's to request which
/// subfields to include in a a response. Implementations of FieldSelector will
/// typically be automatically generated from a procedural macro using
/// `#[derive(FieldSelector)]`.
pub trait FieldSelector {
    fn field_selector() -> String {
        let mut selector = String::new();
        Self::field_selector_with_ident("", &mut selector);
        selector
    }

    fn field_selector_with_ident(ident: &str, selector: &mut String);
}

fn append_ident_to_selector(ident: &str, selector: &mut String) {
    match selector.chars().rev().nth(0) {
        Some(',') | None => {}
        _ => selector.push_str(","),
    }
    selector.push_str(ident);
}

// The google api allows specifying attributes of elements within containers
// enclosing it in parens '()'. FieldSelector is implemented for Vec, HashSet,
// and BTreeSet to support this functionality.
fn container_selector<T>(ident: &str, selector: &mut String)
where
    T: FieldSelector,
{
    append_ident_to_selector(ident, selector);
    let mut inner_selector = String::new();
    T::field_selector_with_ident("", &mut inner_selector);
    if !inner_selector.is_empty() {
        selector.push_str("(");
        selector.push_str(&inner_selector);
        selector.push_str(")");
    };
}

macro_rules! leaf_selector_for {
    ($t:ty) => {
        impl FieldSelector for $t {
            fn field_selector_with_ident(ident: &str, selector: &mut String) {
                append_ident_to_selector(ident, selector);
            }
        }
    };
}

leaf_selector_for!(bool);
leaf_selector_for!(char);
leaf_selector_for!(i8);
leaf_selector_for!(i16);
leaf_selector_for!(i32);
leaf_selector_for!(i64);
leaf_selector_for!(i128);
leaf_selector_for!(isize);
leaf_selector_for!(u8);
leaf_selector_for!(u16);
leaf_selector_for!(u32);
leaf_selector_for!(u64);
leaf_selector_for!(u128);
leaf_selector_for!(usize);
leaf_selector_for!(f32);
leaf_selector_for!(f64);
leaf_selector_for!(String);

// For field selection we treat Options as invisible, proxying to the inner type.
impl<T> FieldSelector for Option<T>
where
    T: FieldSelector,
{
    fn field_selector_with_ident(ident: &str, selector: &mut String) {
        T::field_selector_with_ident(ident, selector)
    }
}

// implement FieldSelector for std::collections types.
// Vec, VecDeque, HashSet, BTreeSet, LinkedList, all act as containers of other elements.

impl<T> FieldSelector for Vec<T>
where
    T: FieldSelector,
{
    fn field_selector_with_ident(ident: &str, selector: &mut String) {
        container_selector::<T>(ident, selector)
    }
}

impl<T> FieldSelector for VecDeque<T>
where
    T: FieldSelector,
{
    fn field_selector_with_ident(ident: &str, selector: &mut String) {
        container_selector::<T>(ident, selector)
    }
}

impl<T, H> FieldSelector for HashSet<T, H>
where
    T: FieldSelector,
{
    fn field_selector_with_ident(ident: &str, selector: &mut String) {
        container_selector::<T>(ident, selector)
    }
}

impl<T> FieldSelector for BTreeSet<T>
where
    T: FieldSelector,
{
    fn field_selector_with_ident(ident: &str, selector: &mut String) {
        container_selector::<T>(ident, selector)
    }
}

impl<T> FieldSelector for LinkedList<T>
where
    T: FieldSelector,
{
    fn field_selector_with_ident(ident: &str, selector: &mut String) {
        container_selector::<T>(ident, selector)
    }
}

// HashMap and BTreeMap are not considered containers for the purposes of
// selections. The google api does not provide a mechanism to specify fields of
// key/value pairs.

impl<K, V, H> FieldSelector for HashMap<K, V, H> {
    fn field_selector_with_ident(ident: &str, selector: &mut String) {
        append_ident_to_selector(ident, selector)
    }
}

impl<K, V> FieldSelector for BTreeMap<K, V> {
    fn field_selector_with_ident(ident: &str, selector: &mut String) {
        append_ident_to_selector(ident, selector)
    }
}
