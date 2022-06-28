#[cfg(test)]
mod tests {
    use log::debug;
    use rust_exam::type_of;
    use rust_exam::Point3D;

    #[test]
    fn box_test() {
        let mut boxi: Box<i32> = Box::new(1);
        assert_eq!(*boxi, 1);
        debug!("boxi{}=[{}]", type_of(&boxi), boxi);
        boxi = 2.into();
        debug!("boxi{}=[{}]", type_of(&boxi), boxi);
        assert_eq!(*boxi, 2);

        // let i2: i32 = *boxi + 1;
        // let var_name = *boxi;
        boxi = Box::new(*boxi + 1);

        debug!("boxi{}=[{}]", type_of(&boxi), boxi);
        assert_eq!(*boxi, 3);
    }
}

// #[cfg(test)] // cargo testを走らせた時にだけ、 テストコードをコンパイル
// #[test]
// fn box_test() {
//     let mut boxi: Box<i32> = Box::new(1);
//     assert_eq!(*boxi, 1);
//     debug!("boxi{}=[{}]", type_of(&boxi), boxi);
//     boxi = 2.into();
//     debug!("boxi{}=[{}]", type_of(&boxi), boxi);
//     assert_eq!(*boxi, 2);

//     // let i2: i32 = *boxi + 1;
//     // let var_name = *boxi;
//     boxi = Box::new(*boxi + 1);

//     debug!("boxi{}=[{}]", type_of(&boxi), boxi);
//     assert_eq!(*boxi, 3);
// }
