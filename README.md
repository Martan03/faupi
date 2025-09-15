# faupi

Blazingly fast API Mock Server written in Rust.

## Table of Contents
- [Installation](#installation)
- [Usage](#usage)
- [Specification](#specification)
    - [Specification URL](#specification-url)
    - [Specification response](#specification-response)
        - [Specification response body](#specification-response-body)
    - [Specification example](#specification-example)
- [Import OpenAPI specification](#import-openapi-specification)
- [Links](#links)

## Installation

Currently there's no other way of installing then building it yourself. You need
to have Rust Toolchain installed (see
[rust installation page](https://www.rust-lang.org/tools/install)). When you
have the Rust Toochain, you can build the project with `cargo`:
```bash
cargo build -r
```

After it's done compiling, the binary will be `target/release/faupi`.

## Usage

> Note: In all the examples, the path to the project binary is substituted by 
> `faupi`.

To run the actual Mock API server, you can start it by running:
```bash
faupi serve -s specs.yaml
faupi serve -s specs.yaml -a 123.345.67.89 -p 1234
```

where the `specs.yaml` contains the specification of the Mock API. More about
that in the [specification section](#specification). The second command 
showcases running the server on custom address and port.

More details about all the functionality can be found in the help:
```bash
faupi -h
```

## Import OpenAPI specification

> Note: this only supports basic OpenAPI specification files

If you already have an OpenAPI specification file, you can import it with 
`faupi` and convert it to the `faupi` specification file. You can do it like 
this:
```bash
faupi import -i docs.jsonopenapi -o faupi-spec.yaml
```

## Specification

Specification is either `.yaml` or `.json` file. It contains list of endpoint
specifications. Each endpoint specification sets what the API Mock server
should respond on each URL with different HTTP methods. The endpoint specification contains:
- HTTP method (`method`)
    - `Get`, `Head`, `Post`, `Put`, `Delete`, `Connect`, `Options`, `Trace`, 
    `Path`
- Endpoint URL (`url`)
    - See [specification URL](#specification-url).
- Response (`response`) - optional
    - See [specification response](#specification-response).

### Specification URL

URL in the specification can be normal URL string, such as `/api/example/36`.
This URL has hardcoded ID, which is not ideal for API testing. For this purpose,
`faupi` contains option to add parameters into the URL.

To add parameter to the URL, you have to enclose the parameter definition in the
curly brackets. Variable definition consists of name and optional type - type
defaults to `string`. If we parametrize the mentioned URL, we get 
`/api/example/{id: number}`.

In the above example, we used the type `number`, which suits the ID better.
Currently supported URL parameter types are:
- `string`
- `number`

### Specification response

Specification response corresponds to the HTTP response returned by the API
Mock server. It contains:
- HTTP response status (`status`) - defaults to `200`.
    - 200 = OK, 404 = Not Found,...
- HTTP response delay (`delay`) - defaults to no delay.
    - Time the server waits before sending response (in milliseconds).
- HTTP response body (`body`) - defaults to `null`.
    - See [specification response body](#specification-response-body).

#### Specification response body

To be able to support more dynamic responses, response body supports variables.
Currently only way to set the variables is from the URL parameters. 

To use a variable inside of a body value, you can add `$` followed by the
variable name (such as `$name`). To prevent ambiguity, you can also wrap the
variable name inside of curly brackets (`${name}`) - this way you can chain
a variable and static string after each other without having to use a space.

`faupi` also supports generating random variable values. It contains a special
object-like variable - `fake`. You can then use one of the built-in object
attributes to generate random value, such as first name, last name, profesion
and more. You use it as a normal variable with `$` - `$fake.name`. To see
all the `fake` object attributes, visit [fake object section](#fake-object).

### Specification example 
To add API GET endpoint on URL `/api/example/{id:number}`, we can add this
endpoint specification to out specification file:
```yaml
- method: Get
  url: /api/example/{id:number}
  response:
    status: 200
    body:
      id: $id
      message: Hope you like faupi!
      review: $fake.name recommends using it.
```

This endpoint specification returns response with HTTP response status 200 - OK
and body contains object with attributes `id` and `message`. The `message`
contains static string, but the `id` gets expanded to the URL `id` parameter 
value. For example, when API receives HTTP request with URL `/api/example/36`,
the response body will contain object with `id` set to `36`.

### Fake object

#### Address

| Attribute                | Description                   |
| ------------------------ | ----------------------------- |
| `building_number`        | Random building number        |
| `city_name`              | Random city name              |
| `city_prefix`            | Random city prefix            |
| `city_suffix`            | Random city suffix            |
| `country_code`           | Random country code           |
| `latitude`               | Random latitude               |
| `longitude`              | Random longitude              |
| `post_code`              | Random postal code            |
| `secondary_address`      | Random secondary address      |
| `secondary_address_type` | Random secondary address type |
| `state_abbr`             | Random state abbreviation     |
| `state_name`             | Random state name             |
| `street_name`            | Random street name            |
| `street_suffix`          | Random street suffix          |
| `time_zone`              | Random time zone              |
| `zip_code`               | Random ZIP code               |

#### Barcode

| Attribute | Description    |
| --------- | -------------- |
| `isbn`    | Random ISBN    |
| `isbn10`  | Random ISBN-10 |
| `isbn13`  | Random ISBN-13 |

#### Company

| Attribute         | Description                 |
| ----------------- | --------------------------- |
| `bs`              | Random business slogan      |
| `bs_adj`          | Random business adjective   |
| `bs_noun`         | Random business noun        |
| `bs_verb`         | Random business verb        |
| `buzzword`        | Random buzzword             |
| `buzzword_middle` | Random buzzword middle word |
| `buzzword_tail`   | Random buzzword tail word   |
| `catch_phrase`    | Random company catch phrase |
| `company_name`    | Random company name         |
| `company_suffix`  | Random company suffix       |
| `industry`        | Random industry             |
| `profession`      | Random profession           |

#### Credit card

| Attribute            | Description               |
| -------------------- | ------------------------- |
| `credit_card_number` | Random credit card number |

#### Currency

| Attribute         | Description            |
| ----------------- | ---------------------- |
| `currency_code`   | Random currency code   |
| `currency_name`   | Random currency name   |
| `currency_symbol` | Random currency symbol |

#### Filesystem

| Attribute         | Description             |
| ----------------- | ----------------------- |
| `dir_path`        | Random directory path   |
| `file_extension`  | Random file extension   |
| `file_name`       | Random file name        |
| `file_path`       | Random file path        |
| `mime_type`       | Random MIME type        |
| `semver`          | Random semantic version |
| `semver_stable`   | Random stable semver    |
| `semver_unstable` | Random unstable semver  |

#### Finance

| Attribute | Description |
| --------- | ----------- |
| `bic`     | Random BIC  |
| `isin`    | Random ISIN |

#### Internet

| Attribute             | Description                  |
| --------------------- | ---------------------------- |
| `domain_suffix`       | Random domain suffix         |
| `free_email`          | Random free email address    |
| `free_email_provider` | Random free email provider   |
| `ip`                  | Random IP address            |
| `ipv4`                | Random IPv4 address          |
| `ipv6`                | Random IPv6 address          |
| `mac_address`         | Random MAC address           |
| `password`            | Random password (8–20 chars) |
| `safe_email`          | Random safe email address    |
| `user_agent`          | Random user agent string     |
| `username`            | Random username              |

#### Job

| Attribute   | Description          |
| ----------- | -------------------- |
| `field`     | Random job field     |
| `position`  | Random job position  |
| `seniority` | Random job seniority |
| `job_title` | Random job title     |

#### Name

| Attribute         | Description            |
| ----------------- | ---------------------- |
| `first_name`      | Random first name      |
| `last_name`       | Random last name       |
| `name`            | Random full name       |
| `name_with_title` | Random name with title |
| `suffix`          | Random suffix          |
| `title`           | Random title           |

#### Number

| Attribute | Description         |
| --------- | ------------------- |
| `digit`   | Random single digit |

#### Phone number

| Attribute      | Description         |
| -------------- | ------------------- |
| `cell_number`  | Random cell number  |
| `phone_number` | Random phone number |

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [faupi](https://github.com/Martan03/faupi)
- **Author website:** [martan03.github.io](https://martan03.github.io)
