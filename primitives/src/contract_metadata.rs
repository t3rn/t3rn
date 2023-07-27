// Copyright 2018-2020 Parity Technologies (UK) Ltd.
// This file is part of cargo-contract.
//
// cargo-contract is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// cargo-contract is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with cargo-contract.  If not, see <http://www.gnu.org/licenses/>.

use crate::Bytes;
use codec::{Decode, Encode, MaxEncodedLen};
#[cfg(feature = "std")]
use core::fmt::{Display, Formatter, Result as DisplayResult};
use scale_info::TypeInfo;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize, Serializer};
use sp_runtime::RuntimeDebug;
use sp_std::{vec, vec::Vec};

#[derive(RuntimeDebug)]
pub struct Source {
    hash: [u8; 32],
    language: SourceLanguage,
    compiler: SourceCompiler,
}

impl Source {
    /// Constructs a new InkProjectSource.
    pub fn new(hash: [u8; 32], language: SourceLanguage, compiler: SourceCompiler) -> Self {
        Source {
            hash,
            language,
            compiler,
        }
    }
}

/// The language and version in which a smart contract is written.
#[derive(RuntimeDebug)]
pub struct SourceLanguage {
    language: Language,
    version: Bytes,
}

impl SourceLanguage {
    /// Constructs a new SourceLanguage.
    pub fn new(language: Language, version: Vec<u8>) -> Self {
        SourceLanguage { language, version }
    }
}

#[cfg(feature = "std")]
impl Serialize for SourceLanguage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[cfg(feature = "std")]
impl Display for SourceLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        write!(f, "{} {:?}", self.language, self.version)
    }
}

/// The language in which the smart contract is written.
#[derive(RuntimeDebug)]
pub enum Language {
    Ink,
    Solidity,
    AssemblyScript,
}

#[cfg(feature = "std")]
impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Self::Ink => write!(f, "ink!"),
            Self::Solidity => write!(f, "Solidity"),
            Self::AssemblyScript => write!(f, "AssemblyScript"),
        }
    }
}

/// A compiler used to compile a smart contract.
#[derive(RuntimeDebug)]
pub struct SourceCompiler {
    compiler: Compiler,
    version: Vec<u8>,
}

#[cfg(feature = "std")]
impl Display for SourceCompiler {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        write!(f, "{} {:?}", self.compiler, self.version)
    }
}

#[cfg(feature = "std")]
impl Serialize for SourceCompiler {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl SourceCompiler {
    pub fn new(compiler: Compiler, version: Vec<u8>) -> Self {
        SourceCompiler { compiler, version }
    }
}

/// Compilers used to compile a smart contract.
#[derive(RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Compiler {
    RustC,
    Solang,
}

#[cfg(feature = "std")]
impl Display for Compiler {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
        match self {
            Self::RustC => write!(f, "rustc"),
            Self::Solang => write!(f, "solang"),
        }
    }
}

/// Type of the contract.
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode, TypeInfo, Copy, Default, MaxEncodedLen)]
pub enum ContractType {
    #[default]
    System,
    VanillaEvm,
    VanillaWasm,
    VolatileEvm,
    VolatileWasm,
}

impl From<ContractType> for u8 {
    fn from(value: ContractType) -> Self {
        match value {
            ContractType::System => 0,
            ContractType::VanillaEvm => 1,
            ContractType::VanillaWasm => 2,
            ContractType::VolatileEvm => 3,
            ContractType::VolatileWasm => 4,
        }
    }
}

/// Metadata about a smart contract.
#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode, TypeInfo)]
pub struct ContractMetadata {
    metadata_version: Vec<u8>,
    name: Vec<u8>,
    contract_type: ContractType,
    version: Vec<u8>,
    authors: Vec<Vec<u8>>,
    description: Option<Vec<u8>>,
    documentation: Option<Vec<u8>>,
    repository: Option<Vec<u8>>,
    homepage: Option<Vec<u8>>,
    license: Option<Vec<u8>>,
}

impl Default for ContractMetadata {
    fn default() -> Self {
        ContractMetadata {
            metadata_version: b"0.0.1".encode(),
            name: b"Default contract".encode(),
            contract_type: ContractType::VolatileWasm,
            version: b"0.0.1".encode(),
            authors: vec![b"Some author".encode()],
            description: None,
            documentation: None,
            repository: None,
            homepage: None,
            license: None,
        }
    }
}

impl ContractMetadata {
    pub fn new(
        metadata_version: Vec<u8>,
        name: Vec<u8>,
        contract_type: ContractType,
        version: Vec<u8>,
        authors: Vec<Vec<u8>>,
        description: Option<Vec<u8>>,
        documentation: Option<Vec<u8>>,
        repository: Option<Vec<u8>>,
        homepage: Option<Vec<u8>>,
        license: Option<Vec<u8>>,
    ) -> ContractMetadata {
        ContractMetadata {
            metadata_version,
            name,
            contract_type,
            version,
            authors,
            description,
            documentation,
            repository,
            homepage,
            license,
        }
    }

    pub fn system_contract() -> Self {
        ContractMetadata {
            metadata_version: b"0.0.1".encode(),
            name: b"Default contract".encode(),
            contract_type: ContractType::System,
            version: b"0.0.1".encode(),
            authors: vec![b"Some author".encode()],
            description: None,
            documentation: None,
            repository: None,
            homepage: None,
            license: None,
        }
    }

    pub fn get_contract_type(&self) -> &ContractType {
        &self.contract_type
    }

    pub fn with_type(mut self, kind: ContractType) -> Self {
        self.contract_type = kind;
        self
    }
}
