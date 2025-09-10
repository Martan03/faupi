use fake::{
    Fake,
    faker::{
        address::raw::*, barcode::raw::*, company::raw::*, creditcard::raw::*,
        currency::raw::*, filesystem::raw::*, finance::raw::*,
        internet::raw::*, job::raw::*, name::raw::*, number::raw::*,
        phone_number::raw::*,
    },
};

use crate::server::url::var::UrlVar;

pub fn get_fake<
    L: Copy + fake::locales::Data + fake::faker::impls::address::CityNameGenFn,
>(
    attr: &str,
    locale: L,
) -> Option<UrlVar> {
    Some(match attr {
        // address module
        "building_number" => UrlVar::String(BuildingNumber(locale).fake()),
        "city_name" => UrlVar::String(CityName(locale).fake()),
        "city_prefix" => UrlVar::String(CityPrefix(locale).fake()),
        "city_suffix" => UrlVar::String(CitySuffix(locale).fake()),
        "country_code" => UrlVar::String(CountryCode(locale).fake()),
        "latitude" => UrlVar::String(Latitude(locale).fake()),
        "longitude" => UrlVar::String(Longitude(locale).fake()),
        "post_code" => UrlVar::String(PostCode(locale).fake()),
        "secondary_address" => UrlVar::String(SecondaryAddress(locale).fake()),
        "secondary_address_type" => {
            UrlVar::String(SecondaryAddressType(locale).fake())
        }
        "state_abbr" => UrlVar::String(StateAbbr(locale).fake()),
        "state_name" => UrlVar::String(StateName(locale).fake()),
        "street_name" => UrlVar::String(StreetName(locale).fake()),
        "street_suffix" => UrlVar::String(StreetSuffix(locale).fake()),
        "time_zone" => UrlVar::String(TimeZone(locale).fake()),
        "zip_code" => UrlVar::String(ZipCode(locale).fake()),

        // barcode module
        "isbn" => UrlVar::String(Isbn(locale).fake()),
        "isbn10" => UrlVar::String(Isbn10(locale).fake()),
        "isbn13" => UrlVar::String(Isbn13(locale).fake()),

        // boolean module
        // "boolean" => UrlVar::String(Boolean(locale).fake()),

        // company module
        "bs" => UrlVar::String(Bs(locale).fake()),
        "bs_adj" => UrlVar::String(BsAdj(locale).fake()),
        "bs_noun" => UrlVar::String(BsNoun(locale).fake()),
        "bs_verb" => UrlVar::String(BsVerb(locale).fake()),
        "buzzword" => UrlVar::String(Buzzword(locale).fake()),
        "buzzword_middle" => UrlVar::String(BuzzwordMiddle(locale).fake()),
        "buzzword_tail" => UrlVar::String(BuzzwordTail(locale).fake()),
        "catch_phrase" => UrlVar::String(CatchPhrase(locale).fake()),
        "company_name" => UrlVar::String(CompanyName(locale).fake()),
        "company_suffix" => UrlVar::String(CompanySuffix(locale).fake()),
        "industry" => UrlVar::String(Industry(locale).fake()),
        "profession" => UrlVar::String(Profession(locale).fake()),

        // creditcard module
        "credit_card_number" => {
            UrlVar::String(CreditCardNumber(locale).fake())
        }

        // currency module
        "currency_code" => UrlVar::String(CurrencyCode(locale).fake()),
        "currency_name" => UrlVar::String(CurrencyName(locale).fake()),
        "currency_symbol" => UrlVar::String(CurrencySymbol(locale).fake()),

        // filesystem module
        "dir_path" => UrlVar::String(DirPath(locale).fake()),
        "file_extension" => UrlVar::String(FileExtension(locale).fake()),
        "file_name" => UrlVar::String(FileName(locale).fake()),
        "file_path" => UrlVar::String(FilePath(locale).fake()),
        "mime_type" => UrlVar::String(MimeType(locale).fake()),
        "semver" => UrlVar::String(Semver(locale).fake()),
        "semver_stable" => UrlVar::String(SemverStable(locale).fake()),
        "semver_unstable" => UrlVar::String(SemverUnstable(locale).fake()),

        // finance module
        "bic" => UrlVar::String(Bic(locale).fake()),
        "isin" => UrlVar::String(Isin(locale).fake()),

        // internet module
        "domain_suffix" => UrlVar::String(DomainSuffix(locale).fake()),
        "free_email" => UrlVar::String(FreeEmail(locale).fake()),
        "free_email_provider" => {
            UrlVar::String(FreeEmailProvider(locale).fake())
        }
        "ip" => UrlVar::String(IP(locale).fake()),
        "ipv4" => UrlVar::String(IPv4(locale).fake()),
        "ipv6" => UrlVar::String(IPv6(locale).fake()),
        "mac_address" => UrlVar::String(MACAddress(locale).fake()),
        "password" => UrlVar::String(Password(locale, 8..20).fake()),
        "safe_email" => UrlVar::String(SafeEmail(locale).fake()),
        "user_agent" => UrlVar::String(UserAgent(locale).fake()),
        "username" => UrlVar::String(Username(locale).fake()),

        // job module
        "field" => UrlVar::String(Field(locale).fake()),
        "position" => UrlVar::String(Position(locale).fake()),
        "seniority" => UrlVar::String(Seniority(locale).fake()),
        "job_title" => {
            UrlVar::String(fake::faker::job::raw::Title(locale).fake())
        }

        // name module
        "first_name" => UrlVar::String(FirstName(locale).fake()),
        "last_name" => UrlVar::String(LastName(locale).fake()),
        "name" => UrlVar::String(Name(locale).fake()),
        "name_with_title" => UrlVar::String(NameWithTitle(locale).fake()),
        "suffix" => UrlVar::String(Suffix(locale).fake()),
        "title" => {
            UrlVar::String(fake::faker::name::raw::Title(locale).fake())
        }

        // number module
        "digit" => UrlVar::String(Digit(locale).fake()),

        // phone_number module
        "cell_number" => UrlVar::String(CellNumber(locale).fake()),
        "phone_number" => UrlVar::String(PhoneNumber(locale).fake()),

        _ => return None,
    })
}
