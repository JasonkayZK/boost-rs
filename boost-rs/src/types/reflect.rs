pub fn get_type_name_of<T>(_: &T) -> String {
    std::any::type_name::<T>().to_string()
}
