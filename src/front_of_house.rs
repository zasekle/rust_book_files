pub(crate) mod hosting;

mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}

    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }

    //Even if the struct is marker pub, each field must be marked pub to allow access. Breakfast
    // must also have an initialized implemented (summer below).
    pub struct Breakfast {
        pub toast: String,
        fruit: String
    }

    impl Breakfast {
        fn summer(toast: &str) -> Breakfast {
            Breakfast{
                toast: String::from(toast),
                fruit: String::from("peaches")
            }
        }
    }

    //All fields of an enum are by default public if the enum is public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn winning() {
    println!("winning()");
}