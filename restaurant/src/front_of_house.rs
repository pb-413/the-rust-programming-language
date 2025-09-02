// 7.5 pull to its own file

pub mod hosting; // 7.3 made public

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
