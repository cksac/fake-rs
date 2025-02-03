use crate::faker::internet::raw::*;
use crate::faker::lorem::raw::Word;
use crate::faker::name::raw::FirstName;
use crate::locales::Data;
use crate::{Dummy, Fake, Faker};
use deunicode::AsciiChars;
use rand::distr::{Distribution, Uniform};
use rand::seq::IndexedRandom;
use rand::Rng;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

impl<L: Data> Dummy<FreeEmailProvider<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FreeEmailProvider<L>, rng: &mut R) -> Self {
        let s = *L::INTERNET_FREE_EMAIL_PROVIDER.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<FreeEmailProvider<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &FreeEmailProvider<L>, rng: &mut R) -> Self {
        L::INTERNET_FREE_EMAIL_PROVIDER.choose(rng).unwrap()
    }
}

impl<L: Data> Dummy<DomainSuffix<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DomainSuffix<L>, rng: &mut R) -> Self {
        let s = *L::INTERNET_DOMAIN_SUFFIX.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<DomainSuffix<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &DomainSuffix<L>, rng: &mut R) -> Self {
        L::INTERNET_DOMAIN_SUFFIX.choose(rng).unwrap()
    }
}

impl<L: Data + Copy> Dummy<FreeEmail<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &FreeEmail<L>, rng: &mut R) -> Self {
        let username: String = Username(c.0).fake_with_rng(rng);
        let provider: String = FreeEmailProvider(c.0).fake_with_rng(rng);
        format!("{}@{}", username.ascii_chars(), provider)
    }
}

impl<L: Data + Copy> Dummy<SafeEmail<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &SafeEmail<L>, rng: &mut R) -> Self {
        let username: String = FirstName(c.0).fake_with_rng::<&str, _>(rng).to_lowercase();
        let domain = ["com", "net", "org"].choose(rng).unwrap();
        format!("{}@example.{}", username, domain)
    }
}

impl<L: Data + Copy> Dummy<Username<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Username<L>, rng: &mut R) -> Self {
        match Faker.fake_with_rng::<u8, _>(rng) {
            0 => FirstName(c.0).fake_with_rng::<&str, _>(rng).to_lowercase(),
            1 | 2 => format!(
                "{}.{}",
                Word(c.0).fake_with_rng::<&str, _>(rng),
                FirstName(c.0).fake_with_rng::<&str, _>(rng).to_lowercase()
            ),
            3 | 4 => format!(
                "{}{}",
                FirstName(c.0).fake_with_rng::<&str, _>(rng).to_lowercase(),
                Faker.fake_with_rng::<u8, _>(rng)
            ),
            _ => format!(
                "{}_{}",
                FirstName(c.0).fake_with_rng::<&str, _>(rng).to_lowercase(),
                Word(c.0).fake_with_rng::<&str, _>(rng),
            ),
        }
    }
}

impl<L: Data> Dummy<Password<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &Password<L>, rng: &mut R) -> Self {
        let len: usize = c.1.fake_with_rng(rng);
        let s: Option<String> = (0..len)
            .map(|_| Some(*L::INTERNET_PASSWORD_CHARS.choose(rng)?))
            .collect();
        s.unwrap_or_default()
    }
}

impl<L: Data> Dummy<IPv4<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &IPv4<L>, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(u8::MIN, u8::MAX).expect("u8::MIN <= u8::MAX");
        format!(
            "{}.{}.{}.{}",
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
        )
    }
}

impl<L: Data> Dummy<IPv4<L>> for Ipv4Addr {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &IPv4<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng::<Ipv4Addr, _>(rng)
    }
}

impl<L: Data> Dummy<IPv6<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &IPv6<L>, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(u16::MIN, u16::MAX).expect("u16::MIN <= u16::MAX");
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

impl<L: Data> Dummy<IPv6<L>> for Ipv6Addr {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &IPv6<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng::<Ipv6Addr, _>(rng)
    }
}

impl<L: Data + Copy> Dummy<IP<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(c: &IP<L>, _rng: &mut R) -> Self {
        IPv4(c.0).fake()
    }
}

impl<L: Data> Dummy<IP<L>> for IpAddr {
    #[inline]
    fn dummy_with_rng<R: Rng + ?Sized>(_: &IP<L>, rng: &mut R) -> Self {
        Faker.fake_with_rng::<IpAddr, _>(rng)
    }
}

impl<L: Data> Dummy<MACAddress<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &MACAddress<L>, rng: &mut R) -> Self {
        let u = Uniform::new_inclusive(u8::MIN, u8::MAX).expect("u8::MIN <= u8::MAX");
        format!(
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
            u.sample(rng),
        )
    }
}

impl<L: Data> Dummy<UserAgent<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &UserAgent<L>, rng: &mut R) -> Self {
        let s = *L::INTERNET_USER_AGENT.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<UserAgent<L>> for &str {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &UserAgent<L>, rng: &mut R) -> Self {
        L::INTERNET_USER_AGENT.choose(rng).unwrap()
    }
}
