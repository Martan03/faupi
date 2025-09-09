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

## Specification

Specification is either `.yaml` or `.json` file. It contains list of endpoint
specifications. Each endpoint specification sets what the API Mock server
should respond on each URL with different HTTP methods. The endpoint specification contains:
- HTTP method (`method`)
    - `Get`, `Head`, `Post`, `Put`, `Delete`, `Connect`, `Options`, `Trace`, 
    `Path`
- Endpoint URL (`url`)
    - See [specification URL](#specification-url).
- Response (`response`)
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
- HTTP response status (`status`)
    - 200 = OK, 404 = Not Found,...
- HTTP response body (`body`)
    - See [specification response body](#specification-response-body).

#### Specification response body

To be able to support more dynamic responses, response body supports variables.
Currently only way to set the variables is from the URL parameters. 

To use a variable inside of a body value, you can add `$` followed by the
variable name (such as `$name`). To prevent ambiguity, you can also wrap the
variable name inside of curly brackets (`${name}`) - this way you can chain
a variable and static string after each other without having to use space.

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
```

This endpoint specification returns response with HTTP response status 200 - OK
and body contains object with attributes `id` and `message`. The `message`
contains static string, but the `id` gets expanded to the URL `id` parameter 
value. For example, when API receives HTTP request with URL `/api/example/36`,
the response body will contain object with `id` set to `36`.

## Links

- **Author:** [Martan03](https://github.com/Martan03)
- **GitHub repository:** [faupi](https://github.com/Martan03/faupi)
- **Author website:** [martan03.github.io](https://martan03.github.io)
