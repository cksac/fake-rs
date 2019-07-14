use crate::faker::lorem::Word;
use crate::faker::name::FirstName;
use crate::locales::{Data, EN};
use crate::{Dummy, Fake, Faker};
use rand::distributions::{Distribution, Uniform};
use rand::seq::SliceRandom;
use rand::Rng;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::ops;

pub struct FreeEmailProvider<L>(pub L);

impl<L: Data> Dummy<FreeEmailProvider<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FreeEmailProvider<L>, rng: &mut R) -> Self {
        let s = *L::INTERNET_FREE_EMAIL_PROVIDER.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<FreeEmailProvider<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FreeEmailProvider<L>, rng: &mut R) -> Self {
        let s = *L::INTERNET_FREE_EMAIL_PROVIDER.choose(rng).unwrap();
        s
    }
}

pub struct DomainSuffix<L>(pub L);

impl<L: Data> Dummy<DomainSuffix<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DomainSuffix<L>, rng: &mut R) -> Self {
        let s = *L::INTERNET_DOMAIN_SUFFIX.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<DomainSuffix<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DomainSuffix<L>, rng: &mut R) -> Self {
        let s = *L::INTERNET_DOMAIN_SUFFIX.choose(rng).unwrap();
        s
    }
}

pub struct FreeEmail<L>(pub L);

impl<L: Data + Copy> Dummy<FreeEmail<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &FreeEmail<L>, rng: &mut R) -> Self {
        let username: String = Username(c.0).fake_with_rng(rng);
        let provider: String = FreeEmailProvider(c.0).fake_with_rng(rng);
        format!("{}@{}", username, provider)
    }
}

pub struct SafeEmail<L>(pub L);

impl<L: Data + Copy> Dummy<SafeEmail<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &SafeEmail<L>, rng: &mut R) -> Self {
        let username: String = FirstName(EN)
            .fake_with_rng::<&str, _>(rng)
            .replace("'", "")
            .to_lowercase();
        let domain = ["com", "net", "org"].choose(rng).unwrap();
        format!("{}@example.{}", username, domain)
    }
}

pub struct Username<L>(pub L);

impl<L: Data> Dummy<Username<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Username<L>, rng: &mut R) -> Self {
        match Faker.fake_with_rng::<u8, _>(rng) {
            0 => FirstName(EN)
                .fake_with_rng::<&str, _>(rng)
                .replace("'", "")
                .to_lowercase(),
            1 | 2 => format!(
                "{}.{}",
                Word(EN).fake_with_rng::<&str, _>(rng),
                FirstName(EN)
                    .fake_with_rng::<&str, _>(rng)
                    .replace("'", "")
                    .to_lowercase()
            ),
            3 | 4 => format!(
                "{}{}",
                FirstName(EN)
                    .fake_with_rng::<&str, _>(rng)
                    .replace("'", "")
                    .to_lowercase(),
                Faker.fake_with_rng::<u8, _>(rng)
            ),
            _ => format!(
                "{}_{}",
                FirstName(EN)
                    .fake_with_rng::<&str, _>(rng)
                    .replace("'", "")
                    .to_lowercase(),
                Word(EN).fake_with_rng::<&str, _>(rng),
            ),
        }
    }
}

pub struct Password<L>(pub L, pub ops::Range<usize>);

impl<L: Data> Dummy<Password<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Password<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let s: Option<String> = (0..len)
            .map(|_| Some(*L::INTERNET_PASSWORD_CHARS.choose(rng)?))
            .collect();
        s.unwrap_or_default()
    }
}

pub struct IPv4;

impl Dummy<IPv4> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &IPv4, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(u8::min_value(), u8::max_value());
        format!(
            "{}.{}.{}.{}",
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
        )
    }
}

impl Dummy<IPv4> for Ipv4Addr {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &IPv4, rng: &mut R) -> Self {
        Faker.fake_with_rng::<Ipv4Addr, _>(rng)
    }
}

pub struct IPv6;

impl Dummy<IPv6> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &IPv6, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(u16::min_value(), u16::max_value());
        format!(
            "{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}:{:X}",
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

impl Dummy<IPv6> for Ipv6Addr {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &IPv6, rng: &mut R) -> Self {
        Faker.fake_with_rng::<Ipv6Addr, _>(rng)
    }
}

pub struct IP;

impl Dummy<IP> for IpAddr {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &IP, rng: &mut R) -> Self {
        Faker.fake_with_rng::<IpAddr, _>(rng)
    }
}

pub struct Color;

impl Dummy<Color> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Color, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(u8::min_value(), u8::max_value());
        format!(
            "#{:02X}{:02X}{:02X}",
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
        )
    }
}

pub struct UserAgent<L>(pub L);

impl<L: Data> Dummy<UserAgent<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &UserAgent<L>, rng: &mut R) -> Self {
        let s = *L::INTERNET_USER_AGENT.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<UserAgent<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &UserAgent<L>, rng: &mut R) -> Self {
        let s = *L::INTERNET_USER_AGENT.choose(rng).unwrap();
        s
    }
}
