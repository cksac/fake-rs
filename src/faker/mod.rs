mod lorem;
mod name;
mod number;
mod boolean;
mod internet;
mod company;
mod address;
mod phone_number;

pub use ::Faker;
pub use self::lorem::Lorem;
pub use self::name::Name;
pub use self::number::Number;
pub use self::boolean::Boolean;
pub use self::internet::Internet;
pub use self::company::Company;
pub use self::address::Address;
pub use self::phone_number::PhoneNumber;