use crate::faker::creditcard::raw::CreditCardNumber;
use crate::locales::Data;
use crate::Dummy;
use rand::seq::{IndexedRandom, IteratorRandom};
use rand::RngExt;

type PrefixCreditcard<'a> = (u8, Option<&'a [u8]>, &'a [usize]);

static PREFIX_LENGTHS: &[PrefixCreditcard] = &[
    (b'3', Some(b"47"), &[13]),    // American Express
    (b'4', None, &[12, 15]),       // Visa
    (b'5', Some(b"12345"), &[14]), // MasterCard
];

impl<L: Data> Dummy<CreditCardNumber<L>> for String {
    fn dummy_with_rng<R: RngExt + ?Sized>(_: &CreditCardNumber<L>, rng: &mut R) -> String {
        let (prefix, opt_prefix, lens): PrefixCreditcard = *PREFIX_LENGTHS.choose(rng).unwrap();
        let len = *lens.choose(rng).unwrap();
        let mut bytes = vec![prefix];
        if let Some(opts) = opt_prefix {
            bytes.push(*opts.choose(rng).unwrap());
        }
        bytes.extend((1..len).map(|_| (b'0'..(b'9' + 1)).choose(rng).unwrap()));
        let checksum = bytes
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let v = (*b - b'0') as u32;
                // for each even-indexed digit, take the cross sum of the double
                if (i & 1) == 0 {
                    if v > 4 {
                        v * 2 - 9
                    } else {
                        v * 2
                    }
                } else {
                    // for the odd-indexed just add the value
                    v
                }
            })
            .sum::<u32>();
        bytes.push((((100000 - checksum) % 10) as u8) + b'0');
        // this is safe because the prefix and all digits we add are valid UTF-8
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}
