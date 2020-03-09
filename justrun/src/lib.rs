#[cfg(test)]

#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


mod tests {
    #[test]
    fn it_works() {
        let v = vec![1, 2, 3, 4, 5];
        let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
    }
}
