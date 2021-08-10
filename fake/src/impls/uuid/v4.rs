use crate::Dummy;

#[derive(Debug)]
pub struct UuidV4Faker;

impl Dummy<UuidV4Faker> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &UuidV4Faker, _: &mut R) -> Self {
        uuid::Uuid::new_v4()
    }
}
