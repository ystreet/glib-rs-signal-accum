use glib::prelude::*;

glib::wrapper! {
    pub struct Test(
        ObjectSubclass<imp::Test>);
}

impl Default for Test {
    fn default() -> Self {
        Test::new()
    }
}

impl Test {
    pub fn new() -> Self {
        glib::Object::new()
    }
}

mod imp {
    use glib::subclass::*;
    use glib::subclass::prelude::*;

    pub struct Test {}

    #[glib::object_subclass]
    impl ObjectSubclass for Test {
        const NAME: &'static str = "TempTest";
        type Type = super::Test;
        type ParentType = glib::Object;

        fn new() -> Self {
            Self {}
        }
    }

    impl ObjectImpl for Test {
        fn signals() -> &'static [Signal] {
            use once_cell::sync::Lazy;
            static SIGNALS: Lazy<Vec<Signal>> = Lazy::new(|| {
                vec![Signal::builder("start")
                    .run_last()
                    .class_handler(move |_token, _args| {
                            println!("class handler");
                            None
                        })
                    .accumulator(move |_hint, output, input| {
                        println!("accumulator");
                        *output = input.clone();
                        false
                    })
                    .build()
                ]
            });
            SIGNALS.as_ref()
        }
    }
}

fn main() {
    let test = Test::new();
    test.connect_closure("start", false, glib::closure!(|_obj: glib::Object| {
            println!("signal");
        }));
    test.emit_by_name::<()>("start", &[]);
}
