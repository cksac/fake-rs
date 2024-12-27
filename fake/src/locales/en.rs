use crate::{faker::impls::address::CityNameGenFn, locales::Data};

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
pub struct EN;

impl Data for EN {}

impl CityNameGenFn for EN {}
