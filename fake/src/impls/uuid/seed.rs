use crate::Dummy;

#[derive(Debug)]
pub struct UuidSeedFaker<'a> {
    slice: &'a u128,
}

impl<'a> Dummy<UuidSeedFaker<'a>> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidSeedFaker<'a>, _: &mut R) -> Self {
        uuid::Uuid::from_u128(*config.slice)
    }
}
