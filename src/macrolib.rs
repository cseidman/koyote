#![macro_use]
macro_rules! ar_elem {
    ( $ar:ident $($index:expr)+ ) => (
        [
            $(
                $ar[$index],
             )+
        ]
    );
}

macro_rules! bytes_f64 {
    ($fnum:expr) => (
        f64::from_bits(u64::from_be_bytes($fnum));
    )
}

macro_rules! f64_bytes {
    ($fnum:expr) => (
        $fnum.to_bits().to_be_bytes() ;
    )
}