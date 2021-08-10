use crate::Dummy;

#[derive(Debug)]
pub struct UuidV1Faker<'a> {
    timestamp: &'a uuid::v1::Timestamp,
    node_id: &'a [u8],
}

impl<'a> Dummy<UuidV1Faker<'a>> for uuid::Uuid {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &UuidV1Faker<'a>, _: &mut R) -> Self {
        uuid::Uuid::new_v1(*config.timestamp, config.node_id).expect("generate uuid::Uuid::new_v1")
    }
}
