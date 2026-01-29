// Fakers data structure
// Add new fakers here by adding to the appropriate category
const fakersData = [
    {
        id: "faker-name",
        icon: "👤",
        title: "Name",
        fakers: [
            { code: "FirstName()", desc: "Generate a first name" },
            { code: "LastName()", desc: "Generate a last name" },
            { code: "Title()", desc: "Generate a name title (Mr., Mrs., etc.)" },
            { code: "Suffix()", desc: "Generate a name suffix (Jr., Sr., etc.)" },
            { code: "Name()", desc: "Generate a full name" },
            { code: "NameWithTitle()", desc: "Generate a name with title" }
        ]
    },
    {
        id: "faker-address",
        icon: "📍",
        title: "Address",
        fakers: [
            { code: "CityPrefix()", desc: "City name prefix" },
            { code: "CitySuffix()", desc: "City name suffix" },
            { code: "CityName()", desc: "Generate a city name" },
            { code: "CountryName()", desc: "Generate a country name" },
            { code: "CountryCode()", desc: "Generate a country code" },
            { code: "StreetSuffix()", desc: "Street suffix (St., Ave., etc.)" },
            { code: "StreetName()", desc: "Generate a street name" },
            { code: "TimeZone()", desc: "Generate a timezone" },
            { code: "StateName()", desc: "Generate a state name" },
            { code: "StateAbbr()", desc: "State abbreviation" },
            { code: "SecondaryAddressType()", desc: "Secondary address type (Apt., Suite, etc.)" },
            { code: "SecondaryAddress()", desc: "Generate a secondary address" },
            { code: "ZipCode()", desc: "Generate a zip code" },
            { code: "PostCode()", desc: "Generate a postal code" },
            { code: "BuildingNumber()", desc: "Generate a building number" },
            { code: "Latitude()", desc: "Generate a latitude coordinate" },
            { code: "Longitude()", desc: "Generate a longitude coordinate" },
            { code: "Geohash(precision: u8)", desc: "Generate a geohash" }
        ]
    },
    {
        id: "faker-internet",
        icon: "🌐",
        title: "Internet",
        fakers: [
            { code: "FreeEmailProvider()", desc: "Free email provider domain" },
            { code: "DomainSuffix()", desc: "Domain suffix (.com, .org, etc.)" },
            { code: "FreeEmail()", desc: "Generate a free email address" },
            { code: "SafeEmail()", desc: "Generate a safe email (example.com)" },
            { code: "Username()", desc: "Generate a username" },
            { code: "Password(len_range: Range<usize>)", desc: "Generate a password" },
            { code: "IPv4()", desc: "Generate an IPv4 address" },
            { code: "IPv6()", desc: "Generate an IPv6 address" },
            { code: "IP()", desc: "Generate an IP address (v4 or v6)" },
            { code: "MACAddress()", desc: "Generate a MAC address" },
            { code: "UserAgent()", desc: "Generate a user agent string" }
        ]
    },
    {
        id: "faker-lorem",
        icon: "📝",
        title: "Lorem Ipsum",
        fakers: [
            { code: "Word()", desc: "Generate a single word" },
            { code: "Words(count: Range<usize>)", desc: "Generate multiple words" },
            { code: "Sentence(count: Range<usize>)", desc: "Generate a sentence" },
            { code: "Sentences(count: Range<usize>)", desc: "Generate multiple sentences" },
            { code: "Paragraph(count: Range<usize>)", desc: "Generate a paragraph" },
            { code: "Paragraphs(count: Range<usize>)", desc: "Generate multiple paragraphs" }
        ]
    },
    {
        id: "faker-company",
        icon: "🏢",
        title: "Company",
        fakers: [
            { code: "CompanySuffix()", desc: "Company suffix (Inc., LLC, etc.)" },
            { code: "CompanyName()", desc: "Generate a company name" },
            { code: "Buzzword()", desc: "Generate a buzzword" },
            { code: "BuzzwordMiddle()", desc: "Middle part of buzzword phrase" },
            { code: "BuzzwordTail()", desc: "Tail part of buzzword phrase" },
            { code: "CatchPhrase()", desc: "Generate a catchphrase" },
            { code: "BsVerb()", desc: "Business speak verb" },
            { code: "BsAdj()", desc: "Business speak adjective" },
            { code: "BsNoun()", desc: "Business speak noun" },
            { code: "Bs()", desc: "Full business speak phrase" },
            { code: "Profession()", desc: "Generate a profession" },
            { code: "Industry()", desc: "Generate an industry" }
        ]
    },
    {
        id: "faker-job",
        icon: "💼",
        title: "Job",
        fakers: [
            { code: "Seniority()", desc: "Job seniority level" },
            { code: "Field()", desc: "Job field" },
            { code: "Position()", desc: "Job position" },
            { code: "Title()", desc: "Complete job title" }
        ]
    },
    {
        id: "faker-phone",
        icon: "📱",
        title: "Phone",
        fakers: [
            { code: "PhoneNumber()", desc: "Generate a phone number" },
            { code: "CellNumber()", desc: "Generate a cell phone number" }
        ]
    },
    {
        id: "faker-datetime",
        icon: "📅",
        title: "Date & Time",
        fakers: [
            { code: "Time()", desc: "Generate a time" },
            { code: "Date()", desc: "Generate a date" },
            { code: "DateTime()", desc: "Generate a datetime" },
            { code: "Duration()", desc: "Generate a duration" },
            { code: "DateTimeBefore(dt)", desc: "DateTime before specified time" },
            { code: "DateTimeAfter(dt)", desc: "DateTime after specified time" },
            { code: "DateTimeBetween(start, end)", desc: "DateTime between two times" }
        ],
        note: "⚠️ Requires <code>chrono</code> or <code>time</code> feature"
    },
    {
        id: "faker-color",
        icon: "🎨",
        title: "Color",
        fakers: [
            { code: "HexColor()", desc: "Hex color (#RRGGBB)" },
            { code: "RgbColor()", desc: "RGB color" },
            { code: "RgbaColor()", desc: "RGBA color with alpha" },
            { code: "HslColor()", desc: "HSL color" },
            { code: "HslaColor()", desc: "HSLA color with alpha" },
            { code: "Color()", desc: "Random color object" }
        ],
        note: "⚠️ Requires <code>random_color</code> feature"
    },
    {
        id: "faker-number",
        icon: "🔢",
        title: "Number",
        fakers: [
            { code: "Digit()", desc: "Generate a single digit" },
            { code: "NumberWithFormat(fmt: &str)", desc: "Number with format (^ = 1-9, # = 0-9)" }
        ]
    },
    {
        id: "faker-boolean",
        icon: "✅",
        title: "Boolean",
        fakers: [
            { code: "Boolean(ratio: u8)", desc: "Boolean with probability ratio" }
        ]
    },
    {
        id: "faker-currency",
        icon: "💰",
        title: "Currency & Finance",
        fakers: [
            { code: "CurrencyCode()", desc: "Currency code (USD, EUR, etc.)" },
            { code: "CurrencyName()", desc: "Currency name" },
            { code: "CurrencySymbol()", desc: "Currency symbol ($, €, etc.)" },
            { code: "Bic()", desc: "Bank Identifier Code" },
            { code: "Isin()", desc: "International Securities ID" },
            { code: "CreditCardNumber()", desc: "Generate a credit card number" }
        ]
    },
    {
        id: "faker-barcode",
        icon: "📊",
        title: "Barcode",
        fakers: [
            { code: "Isbn()", desc: "ISBN (10 or 13)" },
            { code: "Isbn10()", desc: "10-digit ISBN" },
            { code: "Isbn13()", desc: "13-digit ISBN" }
        ]
    },
    {
        id: "faker-filesystem",
        icon: "📁",
        title: "Filesystem",
        fakers: [
            { code: "FilePath()", desc: "Generate a file path" },
            { code: "FileName()", desc: "Generate a file name" },
            { code: "FileExtension()", desc: "File extension" },
            { code: "DirPath()", desc: "Generate a directory path" },
            { code: "MimeType()", desc: "Generate a MIME type" },
            { code: "Semver()", desc: "Semantic version" },
            { code: "SemverStable()", desc: "Stable semantic version" },
            { code: "SemverUnstable()", desc: "Unstable semantic version" }
        ]
    },
    {
        id: "faker-http",
        icon: "🌐",
        title: "HTTP",
        fakers: [
            { code: "RfcStatusCode()", desc: "RFC HTTP status code" },
            { code: "ValidStatusCode()", desc: "Valid HTTP status code" }
        ],
        note: "⚠️ Requires <code>http</code> feature"
    },
    {
        id: "faker-markdown",
        icon: "📄",
        title: "Markdown",
        fakers: [
            { code: "ItalicWord()", desc: "Italic formatted word" },
            { code: "BoldWord()", desc: "Bold formatted word" },
            { code: "Link()", desc: "Markdown link" },
            { code: "BulletPoints(count)", desc: "Bullet point list" },
            { code: "ListItems(count)", desc: "Numbered list items" },
            { code: "BlockQuoteSingleLine(count)", desc: "Single line blockquote" },
            { code: "BlockQuoteMultiLine(count)", desc: "Multi-line blockquote" },
            { code: "Code(count)", desc: "Code block" }
        ]
    },
    {
        id: "faker-administrative",
        icon: "🏥",
        title: "Administrative",
        fakers: [
            { code: "HealthInsuranceCode()", desc: "Health insurance code" }
        ]
    },
    {
        id: "faker-automotive",
        icon: "🚗",
        title: "Automotive",
        fakers: [
            { code: "LicencePlate()", desc: "Vehicle license plate" }
        ]
    },
    {
        id: "faker-uuid",
        icon: "🆔",
        title: "UUID",
        fakers: [
            { code: "UUIDv1()", desc: "UUID version 1" },
            { code: "UUIDv3()", desc: "UUID version 3" },
            { code: "UUIDv4()", desc: "UUID version 4" },
            { code: "UUIDv5()", desc: "UUID version 5" }
        ],
        note: "⚠️ Requires <code>uuid</code> feature"
    },
    {
        id: "faker-ferroid",
        icon: "🔗",
        title: "Ferroid IDs",
        fakers: [
            { code: "FerroidULID()", desc: "Ferroid ULID" },
            { code: "FerroidTwitterId()", desc: "Twitter/X ID format" },
            { code: "FerroidInstagramId()", desc: "Instagram ID format" },
            { code: "FerroidMastodonId()", desc: "Mastodon ID format" },
            { code: "FerroidDiscordId()", desc: "Discord ID format" }
        ],
        note: "⚠️ Requires <code>ferroid</code> feature"
    },
    {
        id: "faker-decimal",
        icon: "💯",
        title: "Decimal Numbers",
        fakers: [
            { code: "Decimal()", desc: "Random decimal" },
            { code: "PositiveDecimal()", desc: "Positive decimal" },
            { code: "NegativeDecimal()", desc: "Negative decimal" },
            { code: "NoDecimalPoints()", desc: "Decimal without fractional part" }
        ],
        note: "⚠️ Requires <code>rust_decimal</code> feature"
    },
    {
        id: "faker-bigdecimal",
        icon: "🔢",
        title: "Big Decimal Numbers",
        fakers: [
            { code: "BigDecimal()", desc: "Random big decimal" },
            { code: "PositiveBigDecimal()", desc: "Positive big decimal" },
            { code: "NegativeBigDecimal()", desc: "Negative big decimal" },
            { code: "NoBigDecimalPoints()", desc: "Big decimal without fractional part" }
        ],
        note: "⚠️ Requires <code>bigdecimal</code> feature"
    }
];

// Export for use in script.js
if (typeof module !== 'undefined' && module.exports) {
    module.exports = fakersData;
}
