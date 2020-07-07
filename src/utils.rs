pub fn StringToInt<T>(s: &String) -> T {
    return s.parse::<T>().unwrap() ;
}
