pub fn vec_to_tuple_2<T: Default + Clone>(v: Vec<T>) -> (T, T) {
    (
        v.get(0).cloned().unwrap_or(Default::default()),
        v.get(1).cloned().unwrap_or(Default::default()),
    )
}
