// SPDX-License-Identifier: MIT OR Apache-2.0
// ELF analysis skeleton

use object::Object;
use object::ObjectSection;
use object::ObjectSymbol;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ElfSection {
    pub name: String,
    pub vma: u64,
    pub size: u64,
    pub section_type: String,
}

#[derive(Debug, Clone)]
pub struct ElfSymbol {
    pub name: String,
    pub address: u64,
    pub size: u64,
    pub sym_type: String,
}

#[derive(Debug, Clone)]
pub struct ElfInfo {
    pub sections: Vec<ElfSection>,
    pub symbols: Vec<ElfSymbol>,
    pub arch: String,
}

pub fn analyze_elf(elf_path: &Path) -> anyhow::Result<ElfInfo> {
    let data = std::fs::read(elf_path)?;
    let obj = object::File::parse(&*data)?;

    let arch = format!("{:?}", obj.architecture());

    let mut sections = Vec::new();
    for section in obj.sections() {
        let name = section.name().unwrap_or("<unnamed>").to_string();
        let vma = section.address();
        let size = section.size();
        if size == 0 {
            continue;
        }
        let section_type = format!("{:?}", section.kind());
        sections.push(ElfSection {
            name,
            vma,
            size,
            section_type,
        });
    }

    let mut symbols = Vec::new();
    for symbol in obj.symbols() {
        if let Ok(name) = symbol.name() {
            if name.is_empty() {
                continue;
            }
            let address = symbol.address();
            let size = symbol.size();
            let sym_type = format!("{:?}", symbol.kind());
            symbols.push(ElfSymbol {
                name: name.to_string(),
                address,
                size,
                sym_type,
            });
        }
    }

    Ok(ElfInfo {
        sections,
        symbols,
        arch,
    })
}
