use crate::Dummy;

#[derive(Debug)]
pub struct UuidV5Faker<'a> {
    namespace: &'a uuid::Uuid,
    name: &'a [u8],
}

impl<'a> Dummy<UuidV5Faker<'a>> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV5Faker<'a>, _: &mut R) -> Self {
        uuid::Uuid::new_v3(config.namespace, config.name)
    }
}
