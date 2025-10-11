use crate::faker::commerce::raw::*;
use crate::faker::numerify_sym;
use crate::locales::Data;
use crate::Dummy;
use rand::seq::IndexedRandom;
use rand::Rng;

impl<L: Data> Dummy<CommerceColor<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CommerceColor<L>, rng: &mut R) -> Self {
        let s = *L::COMMERCE_COLOR.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CommerceDepartment<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CommerceDepartment<L>, rng: &mut R) -> Self {
        let s = *L::COMMERCE_DEPARTMENT.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CommerceProductAdjective<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CommerceProductAdjective<L>, rng: &mut R) -> Self {
        let s = *L::COMMERCE_PRODUCT_ADJECTIVE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CommerceProductMaterial<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CommerceProductMaterial<L>, rng: &mut R) -> Self {
        let s = *L::COMMERCE_PRODUCT_MATERIAL.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CommerceProductType<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CommerceProductType<L>, rng: &mut R) -> Self {
        let s = *L::COMMERCE_PRODUCT_TYPE.choose(rng).unwrap();
        s.into()
    }
}

impl<L: Data> Dummy<CommerceProduct<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CommerceProduct<L>, rng: &mut R) -> Self {
        let fmt = L::COMMERCE_PRODUCT;
        let adjective = *L::COMMERCE_PRODUCT_ADJECTIVE.choose(rng).unwrap();
        let material = *L::COMMERCE_PRODUCT_MATERIAL.choose(rng).unwrap();
        let product = *L::COMMERCE_PRODUCT_TYPE.choose(rng).unwrap();
        fmt.replace("{Adjective}", adjective)
            .replace("{Material}", material)
            .replace("{Product}", product)
    }
}

impl<L: Data + Copy> Dummy<CommerceProductPrice<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CommerceProductPrice<L>, rng: &mut R) -> Self {
        let fmt = L::COMMERCE_PRODUCT_PRICE;
        numerify_sym(fmt, rng)
    }
}

impl<L: Data + Copy> Dummy<CommercePromotionCode<L>> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &CommercePromotionCode<L>, rng: &mut R) -> Self {
        let fmt = L::COMMERCE_PROMOTION_CODE;
        numerify_sym(fmt, rng)
    }
}
