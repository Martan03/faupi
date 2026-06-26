# faupi

Blazingly fast API Mock Server written in Rust.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Import OpenAPI specification](#import-openapi-specification)
- [Specification](#specification)
    - [Templates](#templates)
    - [Specification URL](#specification-url)
    - [Specification request](#specification-request)
    - [Specification response](#specification-response)
        - [Single response](#single-response)
        - [Multiple response](#multiple-response)
    - [Specification request/response body](#specification-requestresponse-body)
    - [Specification example](#specification-example)
    - [Fake object](#fake-object)
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

> [!NOTE]
> In all the examples, the path to the project binary is substituted by
> `faupi`.

To run the actual Mock API server, you can start it by running:

```bash
faupi serve -s specs.yaml
faupi serve -s specs.yaml -a 123.345.67.89 -p 1234 --cors
```

where the `specs.yaml` contains the specification of the Mock API. More about
that in the [specification section](#specification). The second command
showcases running the server on custom address and port, and with enabled CORS.

More details about all the functionality can be found in the help:

```bash
faupi -h
```

## Import OpenAPI specification

> [!WARNING]
> This only supports basic OpenAPI specification files

If you already have an OpenAPI specification file, you can import it with
`faupi` and convert it to the `faupi` specification file. You can do it like
this:

```bash
faupi import -i docs.jsonopenapi -o faupi-spec.yaml
```

## Specification

Specification is either `.yaml` or `.json` file. The root of the file must
contain two attributes:

- `templates`: A list of reusable body objects (optional).
- `specs`: A list of endpoint specifications.

Each endpoint specification sets what API Mock server should respond with on
each URL for different HTTP methods. The endpoint specification contains:

- HTTP method (`method`)
    - `Get`, `Head`, `Post`, `Put`, `Delete`, `Connect`, `Options`, `Trace`,
      `Path`
- Endpoint URL (`url`)
    - See [specification URL](#specification-url).
- Request (`request`) - optional + only POST, PUT and PATCH methods
- Response (`response`) - optional
    - See [specification response](#specification-response).

### Templates

Templates allow you to define a body structure and reuse it across multiple
requests or responses.

Template names must contain only alphanumeric characters and underscores, and
must start with a letter or underscore. You can reference a template anywhere
in a body by using the `$ref` variable (e.g. `$ref.my_template`).

### Specification URL

The URL in the specification can be a normal URL string, such as
`/api/example/36`. This URL has hardcoded ID, which is not ideal for API
testing. For this purpose, `faupi` contains option to add parameters into the
URL.

To add parameter to the URL, enclose the parameter definition in the curly
brackets. A variable definition consists of a name and an optional type
(defaults to `string`). If we parametrize the mentioned URL, we get
`/api/example/{id:number}`.

Currently supported URL parameter types are:

- `string`
- `number`

### Specification request

The specification request validates the incoming request body. This is optional
and can be used only with POST, PUT and PATCH methods.

Validation compares the incoming JSON against your expected schema. You can
enforce exact values, or use the `type` keyword for structural validation.
Supported types are `string`, `number`, `boolean`, `array`, `object`, and
`any`.

```yaml
request:
    username: "admin" # Exact value match
    age:
        type: number # Type constraint (any number)
```

If the incoming request does not match the expected structure or values, the
server automatically returns a `400 Bad Request`.

### Specification response

The specification response corresponds to the HTTP response returned by the API
Mock server. `faupi` supports both **single** responses and **multiple**
responses with picking strategies.

#### Single response

Contains a static response block:

- HTTP response status (`status`) - defaults to `200`.
    - 200 = OK, 404 = Not Found,...
- HTTP response delay (`delay`) - defaults to no delay.
    - Time the server waits before sending response (in milliseconds).
- HTTP response body (`body`) - defaults to `null`.
    - See [specification response body](#specification-requestresponse-body).

#### Multiple response

Allows simulating flaky APIs, state changes, or other cases of endpoint
changing responses. It requires:

- `strategy`: How to pick the response (`random` or `cycle`).
- `responses`: An array of response objects (same as in single response).

```yaml
response:
    strategy: cycle
    responses:
        - status: 202
          body: "Pending"
        - status: 200
          body: "Complete"
```

### Specification request/response body

To support dynamic requests/responses, the body supports variables. Currently,
variables can be populated from URL parameters or fake data generators.

To use a variable inside of a body value, add `$` followed by the variable name
(e.g. `$name`). To prevent ambiguity, you can also wrap the variable name
inside of curly brackets (`${name}`) - this way you can chain a variable and
static string after each other without having to use a space.

`faupi` also supports generating random variable values using the special
`fake` object. You can use built-in attributes to generate random value,
such as first name, last name, profesion and more. You use it as a normal
variable with `$` - e.g. `$fake.name`. To see all the `fake` object attributes,
visit [fake object section](#fake-object).

### Specification example

This example demonstrates usage of templates, request body validation and
multiple response strategies combined.

```yaml
templates:
    user:
        id: $id
        name: $fake.name
        profession: $fake.profession

specs:
    - method: Get
      url: /api/users/{id:number}
      response:
          status: 200
          body: $ref.user

    - method: Post
      url: /api/users/{id:number}
      request:
          role:
              type: string
          age:
              type: number
      response:
          strategy: cycle
          responses:
              - status: 202
                body: "Processing creation..."
              - status: 201
                body: "User created!"
```

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
