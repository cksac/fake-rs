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

#[test]
fn enum_type() {
    #[derive(Dummy, Debug, Eq, PartialEq)]
    enum MyEnum {
        One,
        Two,
    }

    let o: MyEnum = Faker.fake_with_rng(&mut rng());

    assert_eq!(o, MyEnum::Two);
}

#[test]
fn struct_no_overrides() {
    #[derive(Dummy)]
    struct Obj {
        pub name: String,
    }

    let o: Obj = Faker.fake_with_rng(&mut rng());

    assert_eq!(o.name, "5KuGzxfjPN9Ha");
}

#[test]
fn struct_with_override_range() {
    #[derive(Dummy)]
    struct Obj {
        #[dummy(faker = "(100..200)")]
        pub id: i32,
    }

    let o: Obj = Faker.fake_with_rng(&mut rng());

    assert_eq!(o.id, 156);
}

#[test]
fn struct_with_override_faker() {
    #[derive(Dummy)]
    struct Obj {
        #[dummy(faker = "fake::faker::name::en::Name()")]
        pub name: String,
    }

    let o: Obj = Faker.fake_with_rng(&mut rng());

    assert_eq!(o.name, "Marietta Maggio");
}

#[test]
fn struct_with_enum() {
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
fn struct_with_override_fixed_i32() {
    #[derive(Dummy)]
    struct Obj {
        #[dummy(fixed = "42")]
        pub value: i32,
    }

    let o: Obj = Faker.fake_with_rng(&mut rng());

    assert_eq!(o.value, 42);
}

#[test]
fn struct_with_override_fixed_string() {
    #[derive(Dummy)]
    struct Obj {
        #[dummy(fixed = "\"My string\".into()")]
        pub value: String,
    }

    let o: Obj = Faker.fake_with_rng(&mut rng());

    assert_eq!(o.value, "My string");
}

#[test]
fn struct_with_override_fixed_from_fn() {
    fn my_default() -> String {
        "My String".into()
    }
    #[derive(Dummy)]
    struct Obj {
        #[dummy(fixed = "my_default()")]
        pub value: String,
    }

    let o: Obj = Faker.fake_with_rng(&mut rng());

    assert_eq!(o.value, my_default());
}

#[test]
fn struct_with_override_fixed_str() {
    #[derive(Dummy)]
    struct Obj {
        #[dummy(fixed = "\"My string\"")]
        pub value: &'static str,
    }

    let o: Obj = Faker.fake_with_rng(&mut rng());

    assert_eq!(o.value, "My string");
}

#[test]
#[allow(dead_code)]
fn struct_with_override_fixed_enum() {
    #[derive(Eq, PartialEq, Debug)]
    enum MyEnum {
        One,
        Two,
    }
    #[derive(Dummy)]
    struct Obj {
        #[dummy(fixed = "MyEnum::One")]
        pub value: MyEnum,
    }

    let o: Obj = Faker.fake_with_rng(&mut rng());

    assert_eq!(o.value, MyEnum::One);
}
