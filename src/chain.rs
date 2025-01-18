use bitcoin::util::address;
use {super::*, clap::ValueEnum};

#[derive(Default, ValueEnum, Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum Chain {
  #[default]
  #[clap(alias("main"))]
  Mainnet,
  #[clap(alias("test"))]
  Testnet,
  Signet,
  Regtest,
}

impl Chain {
  pub(crate) fn network(self) -> Network {
    match self {
      Self::Mainnet => Network::Bitcoin,
      Self::Testnet => Network::Testnet,
      Self::Signet => Network::Signet,
      Self::Regtest => Network::Regtest,
    }
  }

  pub(crate) fn default_rpc_port(self) -> u16 {
    match self {
      Self::Mainnet => 59332,
      Self::Regtest => 18332,
      Self::Signet => 38332,
      Self::Testnet => 19332,
    }
  }

  pub(crate) fn inscription_content_size_limit(self) -> Option<usize> {
    match self {
      Self::Mainnet | Self::Regtest => None,
      Self::Testnet | Self::Signet => None,
    }
  }

  pub(crate) fn first_inscription_height(self) -> u32 {
    match self {
      Self::Mainnet => 0,
      Self::Regtest => 0,
      Self::Signet => 0,
      Self::Testnet => 0,
    }
  }

  pub(crate) fn first_rune_height(self) -> u32 {
    match self {
      Self::Mainnet => 0,
      Self::Regtest => 0,
      Self::Signet => 0,
      Self::Testnet => 0,
    }
  }

  pub(crate) fn genesis_block(self) -> Block {
    let genesis_hex: &str = match self {
      Self::Mainnet => "0100000000000000000000000000000000000000000000000000000000000000000000002518ace274baca3f70cc1e6e989b166de0b16028ebfb409c2f9fef21af8df288b9aa8e4ef0ff0f1ecfc5517c0101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff2304ffff001d01041b31332f30372f3230313220424251436f696e20737461727465642effffffff0100ea56fa000000004341040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac00000000",
      Self::Regtest => "0100000000000000000000000000000000000000000000000000000000000000000000002518ace274baca3f70cc1e6e989b166de0b16028ebfb409c2f9fef21af8df288b9aa8e4ef0ff0f1ecfc5517c0101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff2304ffff001d01041b31332f30372f3230313220424251436f696e20737461727465642effffffff0100ea56fa000000004341040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac00000000",
      Self::Signet => "0100000000000000000000000000000000000000000000000000000000000000000000002518ace274baca3f70cc1e6e989b166de0b16028ebfb409c2f9fef21af8df288b9aa8e4ef0ff0f1ecfc5517c0101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff2304ffff001d01041b31332f30372f3230313220424251436f696e20737461727465642effffffff0100ea56fa000000004341040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac00000000",
      Self::Testnet => "0100000000000000000000000000000000000000000000000000000000000000000000002518ace274baca3f70cc1e6e989b166de0b16028ebfb409c2f9fef21af8df288f6028c4ef0ff0f1e38c3f6160101000000010000000000000000000000000000000000000000000000000000000000000000ffffffff2304ffff001d01041b31332f30372f3230313220424251436f696e20737461727465642effffffff0100ea56fa000000004341040184710fa689ad5023690c80f3a49c8f13f8d45b8c857fbcbc8bc4a8e4d3eb4b10f4d4604fa08dce601aaf0f470216fe1b51850b4acf21b179c45070ac7b03a9ac00000000",
    };
    let genesis_buf: Vec<u8> = hex::decode(genesis_hex).unwrap();
    bitcoin::consensus::deserialize(&genesis_buf).unwrap()
  }

  pub(crate) fn address_from_script(
    self,
    script: &Script,
  ) -> Result<Address, address::Error> {
    Address::from_script(script, self.network())
  }

  pub(crate) fn join_with_data_dir(self, data_dir: &Path) -> PathBuf {
    match self {
      Self::Mainnet => data_dir.to_owned(),
      Self::Testnet => data_dir.join("testnet3"),
      Self::Signet => data_dir.join("signet"),
      Self::Regtest => data_dir.join("regtest"),
    }
  }
}

impl Display for Chain {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(
      f,
      "{}",
      match self {
        Self::Mainnet => "mainnet",
        Self::Regtest => "regtest",
        Self::Signet => "signet",
        Self::Testnet => "testnet",
      }
    )
  }
}
