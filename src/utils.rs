/* -------------------------------------------
* Convert string to different numeric types
----------------------------------------------*/
pub trait StringToInt<T> {
    fn Convert(s:&String) -> T ;
}

impl StringToInt<u16> for u16 {
    fn Convert(s: &String) -> u16 {
        return s.parse::<u16>().unwrap() ;
    }
}
impl StringToInt<u32> for u32 {
    fn Convert(s: &String) -> u32 {
        return s.parse::<u32>().unwrap() ;
    }
}
impl StringToInt<i64> for i64 {
    fn Convert(s: &String) -> i64 {
        return s.parse::<i64>().unwrap() ;
    }
}

impl StringToInt<f32> for f32 {
    fn Convert(s: &String) -> f32 {
        return s.parse::<f32>().unwrap() ;
    }
}

impl StringToInt<f64> for f64 {
    fn Convert(s: &String) -> f64 {
        return s.parse::<f64>().unwrap() ;
    }
}

