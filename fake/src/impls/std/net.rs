use crate::{Dummy, Fake, Faker};
use rand::distr::{Distribution, Uniform};
use rand::Rng;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

impl Dummy<Faker> for Ipv4Addr {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(u8::MIN, u8::MAX).expect("u8::MIN <= u8::MAX");
        Ipv4Addr::new(u.sample(rng), u.sample(rng), u.sample(rng), u.sample(rng))
    }
}

impl Dummy<Faker> for Ipv6Addr {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(u16::MIN, u16::MAX).expect("u16::MIN <= u16::MAX");
        Ipv6Addr::new(
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
        )
    }
}

impl Dummy<Faker> for IpAddr {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        if Faker.fake_with_rng::<bool, _>(rng) {
            IpAddr::V4(Faker.fake_with_rng::<Ipv4Addr, _>(rng))
        } else {
            IpAddr::V6(Faker.fake_with_rng::<Ipv6Addr, _>(rng))
        }
    }
}

impl Dummy<Faker> for SocketAddrV4 {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let ip: Ipv4Addr = Faker.fake_with_rng(rng);
        let port: u16 = Faker.fake_with_rng(rng);
        SocketAddrV4::new(ip, port)
    }
}

impl Dummy<Faker> for SocketAddrV6 {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let ip: Ipv6Addr = Faker.fake_with_rng(rng);
        let port: u16 = Faker.fake_with_rng(rng);
        let flowinfo: u32 = Faker.fake_with_rng(rng);
        let scope_id: u32 = Faker.fake_with_rng(rng);
        SocketAddrV6::new(ip, port, flowinfo, scope_id)
    }
}
