# faupi changelog

## v0.3.0 - Attack of the Templates

### Features

- Templates in specification file
- Expected request type and field values
- Open API importer request support
- Multiple endpoint response support with pick strategy (random or cycle)

### Breaking Changes

- Enforce valid template names (only alphanumeric characters and underscores,
  starting with a letter or underscore) in specification file
- Endpoints must now be nested under a root `specs:` attribute in specification
  file to accommodate the new `templates:` block

## v0.2.0 - Lord of the Fakes

### Features

- Response delay
- Fake data in specification
- Open API v3.1 importer

### Changes

- Add default values to some specification file fields

## v0.1.0 - Initial Release

### Features

- Specification file
- URLs with parameters
- Specification endpoint response variables
- HTTP server
- Logging
