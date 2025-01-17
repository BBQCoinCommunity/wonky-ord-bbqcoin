use super::*;

#[derive(Boilerplate)]
pub(crate) struct DunesHtml {
  pub(crate) entries: Vec<(DuneId, DuneEntry)>,
}

impl PageContent for DunesHtml {
  fn title(&self) -> String {
    "Runes".to_string()
  }
}
