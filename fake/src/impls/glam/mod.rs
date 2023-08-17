use crate::{Dummy, Fake, Faker};
use glam::{Mat4, Vec2, Vec3, Vec4};

impl Dummy<Faker> for Mat4 {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_f: &Faker, rng: &mut R) -> Self {
        let col_1: Vec4 = Faker.fake_with_rng(rng);
        let col_2: Vec4 = Faker.fake_with_rng(rng);
        let col_3: Vec4 = Faker.fake_with_rng(rng);
        let col_4: Vec4 = Faker.fake_with_rng(rng);
        Mat4::from_cols(col_1, col_2, col_3, col_4)
    }
}

impl Dummy<Faker> for Vec3 {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_f: &Faker, rng: &mut R) -> Self {
        let x: f32 = Faker.fake_with_rng(rng);
        let y: f32 = Faker.fake_with_rng(rng);
        let z: f32 = Faker.fake_with_rng(rng);
        Vec3::new(x, y, z)
    }
}

impl Dummy<Faker> for Vec2 {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_f: &Faker, rng: &mut R) -> Self {
        let x: f32 = Faker.fake_with_rng(rng);
        let y: f32 = Faker.fake_with_rng(rng);
        Vec2::new(x, y)
    }
}

impl Dummy<Faker> for Vec4 {
    fn dummy_with_rng<R: rand::Rng + ?Sized>(_f: &Faker, rng: &mut R) -> Self {
        let x: f32 = Faker.fake_with_rng(rng);
        let y: f32 = Faker.fake_with_rng(rng);
        let z: f32 = Faker.fake_with_rng(rng);
        let w: f32 = Faker.fake_with_rng(rng);
        Vec4::new(x, y, z, w)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use glam::{vec2, vec3, vec4};
    use rand::rngs::StdRng;
    use rand::{Rng, SeedableRng};

    const SEED: [u8; 32] = [
        1, 0, 0, 0, 23, 0, 0, 0, 200, 1, 0, 0, 210, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        1, 3, 5, 0,
    ];

    #[test]
    fn fake_vec2() {
        let rng = &mut StdRng::from_seed(SEED);
        let expected = vec2(rng.gen(), rng.gen());
        let rng = &mut StdRng::from_seed(SEED);
        let fake = Faker.fake_with_rng::<Vec2, _>(rng);
        assert_eq!(expected, fake);
    }

    #[test]
    fn fake_vec3() {
        let rng = &mut StdRng::from_seed(SEED);
        let expected = vec3(rng.gen(), rng.gen(), rng.gen());
        let rng = &mut StdRng::from_seed(SEED);
        let fake = Faker.fake_with_rng::<Vec3, _>(rng);
        assert_eq!(expected, fake);
    }

    #[test]
    fn fake_vec4() {
        let rng = &mut StdRng::from_seed(SEED);
        let expected = vec4(rng.gen(), rng.gen(), rng.gen(), rng.gen());
        let rng = &mut StdRng::from_seed(SEED);
        let fake = Faker.fake_with_rng::<Vec4, _>(rng);
        assert_eq!(expected, fake);
    }

    #[test]
    fn fake_mat4() {
        let rng = &mut StdRng::from_seed(SEED);
        let expected: Vec<f32> = (0..16).map(|_| rng.gen()).collect();
        let rng = &mut StdRng::from_seed(SEED);
        let fake = Faker.fake_with_rng::<Mat4, _>(rng);
        assert_eq!(expected[0..16], fake.to_cols_array());
    }
}
