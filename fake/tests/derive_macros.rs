use fake::{Dummy, Fake, Faker};
use rand::SeedableRng;

fn rng() -> rand_chacha::ChaCha20Rng {
    // Fixing the RNG So we have more deterministic tests
    // as we are only testing the derive macros, not the RNG
    let seed = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0,
    ];
    rand_chacha::ChaCha20Rng::from_seed(seed)
}

mod field_options {
    use super::*;

    mod enum_type {
        use super::*;

        #[test]
        fn no_overrides() {
            #[derive(Dummy, Debug, Eq, PartialEq)]
            enum MyEnum {
                One,
                Two,
            }

            let o: MyEnum = Faker.fake_with_rng(&mut rng());

            assert_eq!(o, MyEnum::Two);
        }

        #[test]
        #[should_panic(expected = "can not create an empty enum")]
        fn with_no_variants() {
            #[derive(Dummy, Debug, Eq, PartialEq)]
            enum MyEnum {}

            let _o: MyEnum = Faker.fake_with_rng(&mut rng());
        }

        #[test]
        fn with_tuple() {
            #[derive(Dummy, Debug, Eq, PartialEq)]
            enum MyEnum {
                One,
                Two(
                    #[dummy(faker = "1..100")] i32,
                    #[dummy(default)] i32,
                    #[dummy(expr = "1")] i32,
                ),
            }

            let o: MyEnum = Faker.fake_with_rng(&mut rng());

            assert_eq!(o, MyEnum::Two(89, 0, 1));
        }

        #[test]
        fn with_struct() {
            #[derive(Dummy, Debug, Eq, PartialEq)]
            enum MyEnum {
                One,
                Two {
                    #[dummy(faker = "1..100")]
                    x: i32,
                    #[dummy(default)]
                    y: i32,
                    #[dummy(expr = "1")]
                    z: i32,
                },
            }

            let o: MyEnum = Faker.fake_with_rng(&mut rng());

            assert_eq!(o, MyEnum::Two { x: 89, y: 0, z: 1 });
        }
    }

    mod unit_struct {
        use super::*;

        #[test]
        fn no_overrides() {
            #[derive(Dummy)]
            struct Obj;

            let _o: Obj = Faker.fake_with_rng(&mut rng());

            // nothing to really assert other than it compiles and runs
        }
    }

    mod tuple_struct {
        use super::*;

        #[test]
        fn no_overrides() {
            #[derive(Dummy)]
            struct Obj(i32);

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.0, -1377781642);
        }

        #[test]
        fn override_range() {
            #[derive(Dummy)]
            struct Obj(#[dummy(faker = "100..200")] i32);

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.0, 156);
        }

        #[test]
        fn with_enum() {
            #[derive(Dummy, Debug, Eq, PartialEq)]
            enum MyEnum {
                One,
                Two,
            }
            #[derive(Dummy)]
            struct Obj(MyEnum);

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.0, MyEnum::Two);
        }

        #[test]
        fn with_default() {
            #[derive(Dummy)]
            struct Obj(#[dummy(default)] String);

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.0, "");
        }

        #[test]
        fn with_override_faker() {
            #[derive(Dummy)]
            struct Obj(#[dummy(faker = "fake::faker::name::en::Name()")] String);

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.0, "Marietta Maggio");
        }

        #[test]
        fn with_override_expr_i32() {
            #[derive(Dummy)]
            struct Obj(#[dummy(expr = "42")] i32);

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.0, 42);
        }
    }

    mod struct_type {
        use super::*;

        #[test]
        fn no_overrides() {
            #[derive(Dummy)]
            struct Obj {
                pub name: String,
            }

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.name, "5KuGzxfjPN9Ha");
        }

        #[test]
        fn with_override_range() {
            #[derive(Dummy)]
            struct Obj {
                #[dummy(faker = "100..200")]
                pub id: i32,
            }

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.id, 156);
        }

        #[test]
        fn with_override_faker() {
            #[derive(Dummy)]
            struct Obj {
                #[dummy(faker = "fake::faker::name::en::Name()")]
                pub name: String,
            }

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.name, "Marietta Maggio");
        }

        #[test]
        fn with_enum() {
            #[derive(Dummy, Debug, Eq, PartialEq)]
            enum MyEnum {
                One,
                Two,
            }
            #[derive(Dummy)]
            struct Obj {
                pub value: MyEnum,
            }

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.value, MyEnum::Two);
        }

        #[test]
        fn with_default() {
            #[derive(Dummy)]
            struct Obj {
                #[dummy(default)]
                pub value: String,
            }

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.value, "");
        }

        #[test]
        fn with_override_expr_i32() {
            #[derive(Dummy)]
            struct Obj {
                #[dummy(expr = "42")]
                pub value: i32,
            }

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.value, 42);
        }

        #[test]
        fn with_override_expr_string() {
            #[derive(Dummy)]
            struct Obj {
                #[dummy(expr = "\"My string\".into()")]
                pub value: String,
            }

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.value, "My string");
        }

        #[test]
        fn with_override_expr_from_fn() {
            fn my_default() -> String {
                "My String".into()
            }
            #[derive(Dummy)]
            struct Obj {
                #[dummy(expr = "my_default()")]
                pub value: String,
            }

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.value, my_default());
        }

        #[test]
        fn with_override_expr_str() {
            #[derive(Dummy)]
            struct Obj {
                #[dummy(expr = "\"My string\"")]
                pub value: &'static str,
            }

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.value, "My string");
        }

        #[test]
        #[allow(dead_code)]
        fn with_override_expr_enum() {
            #[derive(Eq, PartialEq, Debug)]
            enum MyEnum {
                One,
                Two,
            }
            #[derive(Dummy)]
            struct Obj {
                #[dummy(expr = "MyEnum::One")]
                pub value: MyEnum,
            }

            let o: Obj = Faker.fake_with_rng(&mut rng());

            assert_eq!(o.value, MyEnum::One);
        }
    }
}

mod test_trait_scope {
    #[test]
    #[allow(dead_code)]
    fn it_generates_without_fake_in_scope() {
        mod outer {
            #[derive(fake::Dummy)]
            pub struct Outer {
                pub message: String,
            }

            #[derive(fake::Dummy)]
            pub struct Outer2 {
                #[dummy(faker = "1000..2000")]
                pub id: usize,
            }
        }
    }
}

mod test_generic {
    use super::*;

    #[test]
    #[allow(dead_code)]
    fn generic_struct() {
        #[derive(Eq, PartialEq, Debug, Dummy)]
        struct MyStruct<T, U> {
            f1: T,
            f2: U,
        }

        let o: MyStruct<u8, f32> = Faker.fake_with_rng(&mut rng());

        assert_eq!(o.f1, 118);
        assert_eq!(o.f2, 0.56344515);
    }

    #[test]
    #[allow(dead_code)]
    fn generic_enum() {
        #[derive(Eq, PartialEq, Debug, Dummy)]
        enum MyEnum<T, U> {
            F1(T),
            F2(U),
        }

        let o: MyEnum<u8, f32> = Faker.fake_with_rng(&mut rng());

        assert_eq!(o, MyEnum::F2(0.8961542));
    }

    #[test]
    #[allow(dead_code)]
    fn generic_tuple() {
        #[derive(Eq, PartialEq, Debug, Dummy)]
        struct MyTuple<T, U>(T, U);

        let o: MyTuple<u8, f32> = Faker.fake_with_rng(&mut rng());
        assert_eq!(o.0, 118);
        assert_eq!(o.1, 0.56344515);
    }
}