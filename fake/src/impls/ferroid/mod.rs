//! Fake _ferroid_ generation.

use ferroid::{
    Base32SnowExt, Base32UlidExt, SnowflakeDiscordId, SnowflakeInstagramId, SnowflakeMastodonId,
    SnowflakeTwitterId, ULID,
};

use crate::{Dummy, Faker};

pub struct FerroidULID;
pub struct FerroidTwitterId;
pub struct FerroidDiscordId;
pub struct FerroidMastodonId;
pub struct FerroidInstagramId;

// --- ULID ---
impl Dummy<Faker> for ULID {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let time_part: u128 = rng.random_range(0..(1 << ULID::TIMESTAMP_BITS));
        let rand_part: u128 = rng.random_range(0..(1 << ULID::RANDOM_BITS));
        ULID::from(time_part, rand_part)
    }
}
impl Dummy<FerroidULID> for ULID {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FerroidULID, rng: &mut R) -> Self {
        let time_part: u128 = rng.random_range(0..(1 << ULID::TIMESTAMP_BITS));
        let rand_part: u128 = rng.random_range(0..(1 << ULID::RANDOM_BITS));
        ULID::from(time_part, rand_part)
    }
}
impl Dummy<FerroidULID> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &FerroidULID, rng: &mut R) -> Self {
        ULID::dummy_with_rng(config, rng).encode().to_string()
    }
}

// --- SnowflakeTwitterId ---
impl Dummy<Faker> for SnowflakeTwitterId {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let timestamp: u64 = rng.random_range(0..(1 << SnowflakeTwitterId::TIMESTAMP_BITS));
        let machine_id: u64 = rng.random_range(0..(1 << SnowflakeTwitterId::MACHINE_ID_BITS));
        let sequence: u64 = rng.random_range(0..(1 << SnowflakeTwitterId::SEQUENCE_BITS));
        SnowflakeTwitterId::from(timestamp, machine_id, sequence)
    }
}
impl Dummy<FerroidTwitterId> for SnowflakeTwitterId {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FerroidTwitterId, rng: &mut R) -> Self {
        let timestamp: u64 = rng.random_range(0..(1 << SnowflakeTwitterId::TIMESTAMP_BITS));
        let machine_id: u64 = rng.random_range(0..(1 << SnowflakeTwitterId::MACHINE_ID_BITS));
        let sequence: u64 = rng.random_range(0..(1 << SnowflakeTwitterId::SEQUENCE_BITS));
        SnowflakeTwitterId::from(timestamp, machine_id, sequence)
    }
}
impl Dummy<FerroidTwitterId> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &FerroidTwitterId, rng: &mut R) -> Self {
        SnowflakeTwitterId::dummy_with_rng(config, rng)
            .encode()
            .to_string()
    }
}

// --- SnowflakeDiscordId ---
impl Dummy<Faker> for SnowflakeDiscordId {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let timestamp: u64 = rng.random_range(0..(1 << SnowflakeDiscordId::TIMESTAMP_BITS));
        let machine_id: u64 = rng.random_range(0..(1 << SnowflakeDiscordId::MACHINE_ID_BITS));
        let sequence: u64 = rng.random_range(0..(1 << SnowflakeDiscordId::SEQUENCE_BITS));
        SnowflakeDiscordId::from(timestamp, machine_id, sequence)
    }
}
impl Dummy<FerroidDiscordId> for SnowflakeDiscordId {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FerroidDiscordId, rng: &mut R) -> Self {
        let timestamp: u64 = rng.random_range(0..(1 << SnowflakeDiscordId::TIMESTAMP_BITS));
        let machine_id: u64 = rng.random_range(0..(1 << SnowflakeDiscordId::MACHINE_ID_BITS));
        let sequence: u64 = rng.random_range(0..(1 << SnowflakeDiscordId::SEQUENCE_BITS));
        SnowflakeDiscordId::from(timestamp, machine_id, sequence)
    }
}
impl Dummy<FerroidDiscordId> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &FerroidDiscordId, rng: &mut R) -> Self {
        SnowflakeDiscordId::dummy_with_rng(config, rng)
            .encode()
            .to_string()
    }
}

// --- SnowflakeMastodonId ---
impl Dummy<Faker> for SnowflakeMastodonId {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let timestamp: u64 = rng.random_range(0..(1 << SnowflakeMastodonId::TIMESTAMP_BITS));
        let machine_id: u64 = rng.random_range(0..(1 << SnowflakeMastodonId::MACHINE_ID_BITS));
        let sequence: u64 = rng.random_range(0..(1 << SnowflakeMastodonId::SEQUENCE_BITS));
        SnowflakeMastodonId::from(timestamp, machine_id, sequence)
    }
}
impl Dummy<FerroidMastodonId> for SnowflakeMastodonId {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FerroidMastodonId, rng: &mut R) -> Self {
        let timestamp: u64 = rng.random_range(0..(1 << SnowflakeMastodonId::TIMESTAMP_BITS));
        let machine_id: u64 = rng.random_range(0..(1 << SnowflakeMastodonId::MACHINE_ID_BITS));
        let sequence: u64 = rng.random_range(0..(1 << SnowflakeMastodonId::SEQUENCE_BITS));
        SnowflakeMastodonId::from(timestamp, machine_id, sequence)
    }
}
impl Dummy<FerroidMastodonId> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &FerroidMastodonId, rng: &mut R) -> Self {
        SnowflakeMastodonId::dummy_with_rng(config, rng)
            .encode()
            .to_string()
    }
}

// --- SnowflakeInstagramId ---
impl Dummy<Faker> for SnowflakeInstagramId {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let timestamp: u64 = rng.random_range(0..(1 << SnowflakeInstagramId::TIMESTAMP_BITS));
        let machine_id: u64 = rng.random_range(0..(1 << SnowflakeInstagramId::MACHINE_ID_BITS));
        let sequence: u64 = rng.random_range(0..(1 << SnowflakeInstagramId::SEQUENCE_BITS));
        SnowflakeInstagramId::from(timestamp, machine_id, sequence)
    }
}
impl Dummy<FerroidInstagramId> for SnowflakeInstagramId {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &FerroidInstagramId, rng: &mut R) -> Self {
        let timestamp: u64 = rng.random_range(0..(1 << SnowflakeInstagramId::TIMESTAMP_BITS));
        let machine_id: u64 = rng.random_range(0..(1 << SnowflakeInstagramId::MACHINE_ID_BITS));
        let sequence: u64 = rng.random_range(0..(1 << SnowflakeInstagramId::SEQUENCE_BITS));
        SnowflakeInstagramId::from(timestamp, machine_id, sequence)
    }
}
impl Dummy<FerroidInstagramId> for String {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(config: &FerroidInstagramId, rng: &mut R) -> Self {
        SnowflakeInstagramId::dummy_with_rng(config, rng)
            .encode()
            .to_string()
    }
}
