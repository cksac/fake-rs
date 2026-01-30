// Examples data structure
// Add new examples here by adding to the array
const examplesData = [
    {
        title: "Basic Usage with Faker",
        language: "rust",
        code: `use fake::{Faker, Fake};

// Generate default fake values
let name: String = Faker.fake();
let age: u8 = Faker.fake();
let tuple: (u8, u32, f32) = Faker.fake();
let option: Option<String> = Faker.fake();

println!("Name: {}", name);
println!("Age: {}", age);`
    },
    {
        title: "Using Specific Fakers with Locales",
        language: "rust",
        code: `use fake::faker::name::raw::*;
use fake::faker::lorem::raw::*;
use fake::locales::*;
use fake::Fake;

// Generate names in different locales
let en_name: String = Name(EN).fake();
let zh_name: String = Name(ZH_CN).fake();
let ja_name: String = Name(JA_JP).fake();

// Generate lorem text
let words: Vec<String> = Words(EN, 3..5).fake();
let sentence: String = Sentence(FR_FR, 5..10).fake();`
    },
    {
        title: "Convenient Functions (English Locale)",
        language: "rust",
        code: `use fake::faker::internet::en::*;
use fake::faker::address::en::*;
use fake::Fake;

// Using convenient functions without specifying locale
let email: String = SafeEmail().fake();
let password: String = Password(10..20).fake();
let city: String = CityName().fake();
let country: String = CountryName().fake();`
    },
    {
        title: "Derive Macro for Custom Types",
        language: "rust",
        code: `use fake::{Dummy, Fake, Faker};
use fake::faker::name::en::Name;

#[derive(Debug, Dummy)]
pub struct User {
    #[dummy(faker = "1000..9999")]
    id: usize,
    
    #[dummy(faker = "Name()")]
    name: String,
    
    #[dummy(faker = "18..80")]
    age: u8,
    
    active: bool,
    
    #[dummy(expr = "\\"admin@example.com\\".to_string()")]
    email: String,
    
    #[dummy(default)]
    metadata: String,
}

let user: User = Faker.fake();
println!("{:?}", user);`
    },
    {
        title: "Using Ranges for Primitives",
        language: "rust",
        code: `use fake::Fake;

// Generate values within a range
let id: u32 = (1000..9999).fake();
let price: f64 = (10.0..100.0).fake();
let username: String = (5..15).fake();  // String of length 5-15`
    },
    {
        title: "Generating Collections",
        language: "rust",
        code: `use fake::faker::name::en::Name;
use fake::Fake;

// Using tuple config for vectors
let names: Vec<String> = (Name(), 5..10).fake();

// Using macro for collections
let users = fake::vec![String as Name(); 10];

// Nested collections
let matrix = fake::vec![String as Name(); 3, 4];  // Vec<Vec<String>>`
    },
    {
        title: "Deterministic Generation with Seeds",
        language: "rust",
        code: `use fake::{Faker, Fake};
use fake::rand::SeedableRng;
use fake::rand::rngs::StdRng;

// Use a fixed seed for reproducible results
let seed = [1u8; 32];
let mut rng = StdRng::from_seed(seed);

// Generate with seeded RNG
let value1: u32 = Faker.fake_with_rng(&mut rng);
let value2: u32 = Faker.fake_with_rng(&mut rng);

// Same seed = same sequence
let mut rng2 = StdRng::from_seed(seed);
assert_eq!(value1, Faker.fake_with_rng(&mut rng2));`
    },
    {
        title: "Date & Time Generation",
        language: "rust",
        code: `use fake::faker::chrono::en::*;
use fake::Fake;
use chrono::{DateTime, Utc};

// Generate random dates and times
let date: chrono::NaiveDate = Date().fake();
let time: chrono::NaiveTime = Time().fake();
let datetime: DateTime<Utc> = DateTime().fake();

// Generate relative to specific time
let now = Utc::now();
let past: DateTime<Utc> = DateTimeBefore(now).fake();
let future: DateTime<Utc> = DateTimeAfter(now).fake();

// Generate within a range
let start = Utc::now();
let end = start + chrono::Duration::days(30);
let within: DateTime<Utc> = DateTimeBetween(start, end).fake();`,
        note: "⚠️ Requires <code>chrono</code> feature"
    },
    {
        title: "UUID Generation",
        language: "rust",
        code: `use fake::uuid::*;
use fake::Fake;
use uuid::Uuid;

let uuid_v1: Uuid = UUIDv1.fake();
let uuid_v4: Uuid = UUIDv4.fake();
let uuid_v7: Uuid = UUIDv7.fake();`,
        note: "⚠️ Requires <code>uuid</code> feature"
    },
    {
        title: "HTTP Status Codes",
        language: "rust",
        code: `use fake::faker::http::en::*;
use fake::Fake;
use http::StatusCode;

let status: StatusCode = RfcStatusCode().fake();
let valid_status: StatusCode = ValidStatusCode().fake();`,
        note: "⚠️ Requires <code>http</code> feature"
    },
    {
        title: "Picsum Image URLs",
        language: "rust",
        code: `use fake::faker::picsum::en::*;
use fake::faker::impls::picsum::ImageOptions;
use fake::Fake;

// Simple random image
let url: String = Image().fake();

// Grayscale image
let gray_url: String = ImageGrayscale().fake();

// Blurred image
let blur_url: String = ImageBlur().fake();

// Custom image with options
let opts = ImageOptions::new()
    .width(800)
    .height(600)
    .grayscale()
    .blur(5)
    .seed("myseed");
let custom_url: String = ImageCustom(opts).fake();`
    },
    {
        title: "Either Type Support",
        language: "rust",
        code: `use fake::faker::phone_number::en::{CellNumber, PhoneNumber};
use fake::{utils::{either, WrappedVal}, Dummy, Fake, Faker};

#[derive(Debug, Dummy)]
struct Contact {
    #[dummy(faker = "either(PhoneNumber(), CellNumber())", wrapper = "WrappedVal")]
    phone: String,
}

let contact: Contact = Faker.fake();`,
        note: "⚠️ Requires <code>either</code> feature (enabled by default)"
    },
    {
        title: "CLI Usage",
        language: "bash",
        code: `# Generate a random name (default: English)
fake Name

# Generate 5 names in Chinese
fake -r5 -lzh_cn Name

# Generate 3 email addresses
fake -r3 FreeEmail

# Generate passwords with custom length
fake -r5 Password --min 12 --max 24

# Generate company names in French
fake -l fr_fr CompanyName

# Generate addresses
fake -r3 CityName
fake ZipCode
fake CountryName`
    }
];

// Export for use in script.js
if (typeof module !== 'undefined' && module.exports) {
    module.exports = examplesData;
}
