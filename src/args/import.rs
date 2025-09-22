use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
};

use oas3::OpenApiV3Spec;
use pareg::Pareg;

use crate::{
    args::{missing_param_err, next_arg},
    error::{Error, Result},
    specs::mock_config::MockConfig,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub input: PathBuf,
    pub output: PathBuf,
}

#[derive(Debug, Default)]
struct ImportParser {
    input: Option<PathBuf>,
    output: Option<PathBuf>,
}

impl Import {
    pub fn parse(args: &mut Pareg) -> Result<Import> {
        let mut parsed = ImportParser::default();
        while let Some(arg) = args.peek() {
            match arg {
                "-i" | "--input" => parsed.input = Some(next_arg(args)?),
                "-o" | "--output" => parsed.output = Some(next_arg(args)?),
                "--" => {
                    args.next();
                    break;
                }
                _ => break,
            }
        }
        Import::try_from(parsed)
    }

    pub fn run(&self) -> Result<()> {
        let docs = Self::load_openapi(&self.input)?;
        let specs = MockConfig::try_from(docs)?;
        specs.save(&self.output)
    }

    fn load_openapi(file: impl AsRef<Path>) -> Result<OpenApiV3Spec> {
        match file.as_ref().extension().and_then(|s| s.to_str()) {
            Some("yaml") | Some("yml") => Self::openapi_from_yaml(file),
            Some("json") | Some("jsonopenapi") => {
                Self::openapi_from_json(file)
            }
            _ => Err(Error::Msg("Unsupported import file type".into())),
        }
    }

    fn openapi_from_yaml(file: impl AsRef<Path>) -> Result<OpenApiV3Spec> {
        let buffer = BufReader::new(File::open(file)?);
        serde_yaml::from_reader(buffer).map_err(Into::into)
    }

    fn openapi_from_json(file: impl AsRef<Path>) -> Result<OpenApiV3Spec> {
        let buffer = BufReader::new(File::open(file)?);
        serde_json::from_reader(buffer).map_err(Into::into)
    }
}

impl TryFrom<ImportParser> for Import {
    type Error = Error;

    fn try_from(value: ImportParser) -> Result<Self> {
        Ok(Import {
            input: value.input.ok_or_else(|| missing_param_err("--input"))?,
            output: value
                .output
                .ok_or_else(|| missing_param_err("--output"))?,
        })
    }
}
