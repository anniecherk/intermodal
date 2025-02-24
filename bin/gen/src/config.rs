use crate::common::*;

const PATH: &str = "bin/gen/config.yaml";

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
  pub(crate) changelog: BTreeMap<String, Metadata>,
  pub(crate) examples: Vec<Example>,
  pub(crate) packages: Vec<Package>,
  pub(crate) references: Vec<ReferenceSection>,
  pub(crate) faq: Vec<FaqEntry>,
}

impl Config {
  #[throws]
  pub(crate) fn load(root: &Path) -> Config {
    let file = File::open(root.join(PATH))?;
    serde_yaml::from_reader(file)?
  }
}
