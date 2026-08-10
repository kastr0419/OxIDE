// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright 2026 rust-embedded-ide contributors

use anyhow::Result;

#[derive(Clone, Debug, Default)]
pub struct SvdDevice {
    pub name: String,
    pub peripherals: Vec<SvdPeripheral>,
}

#[derive(Clone, Debug)]
pub struct SvdPeripheral {
    pub name: String,
    pub description: String,
    pub base_address: u64,
    pub registers: Vec<SvdRegister>,
}

#[derive(Clone, Debug)]
pub struct SvdRegister {
    pub name: String,
    pub description: String,
    pub address_offset: u64,
    pub access: String,
    pub fields: Vec<SvdField>,
}

#[derive(Clone, Debug)]
pub struct SvdField {
    pub name: String,
    pub description: String,
    pub bit_offset: u8,
    pub bit_width: u8,
}

pub fn parse_svd(xml: &str) -> Result<SvdDevice> {
    let doc = roxmltree::Document::parse(xml)?;
    let root = doc.root_element();

    let name = find_text(&root, "name").unwrap_or_default().to_string();
    let mut peripherals = Vec::new();

    if let Some(peripherals_node) = root.children().find(|n| n.has_tag_name("peripherals")) {
        for periph_node in peripherals_node
            .children()
            .filter(|n| n.has_tag_name("peripheral"))
        {
            // Skip derived peripherals for now (they reference another)
            if periph_node.attribute("derivedFrom").is_some() {
                continue;
            }
            let pname = find_text(&periph_node, "name").unwrap_or("?").to_string();
            let pdesc = find_text(&periph_node, "description")
                .unwrap_or("")
                .to_string();
            let base_addr = find_text(&periph_node, "baseAddress")
                .and_then(parse_hex_or_dec)
                .unwrap_or(0);

            let mut registers = Vec::new();
            if let Some(regs_node) = periph_node.children().find(|n| n.has_tag_name("registers")) {
                for reg_node in regs_node.children().filter(|n| n.has_tag_name("register")) {
                    let rname = find_text(&reg_node, "name").unwrap_or("?").to_string();
                    let rdesc = find_text(&reg_node, "description")
                        .unwrap_or("")
                        .to_string();
                    let offset = find_text(&reg_node, "addressOffset")
                        .and_then(parse_hex_or_dec)
                        .unwrap_or(0);
                    let access = find_text(&reg_node, "access")
                        .unwrap_or("read-write")
                        .to_string();

                    let mut fields = Vec::new();
                    if let Some(fields_node) =
                        reg_node.children().find(|n| n.has_tag_name("fields"))
                    {
                        for field_node in fields_node.children().filter(|n| n.has_tag_name("field"))
                        {
                            let fname = find_text(&field_node, "name").unwrap_or("?").to_string();
                            let fdesc = find_text(&field_node, "description")
                                .unwrap_or("")
                                .to_string();
                            let bit_offset = find_text(&field_node, "bitOffset")
                                .and_then(|s| s.parse::<u8>().ok())
                                .unwrap_or(0);
                            let bit_width = find_text(&field_node, "bitWidth")
                                .and_then(|s| s.parse::<u8>().ok())
                                .unwrap_or(1);
                            fields.push(SvdField {
                                name: fname,
                                description: fdesc,
                                bit_offset,
                                bit_width,
                            });
                        }
                    }
                    registers.push(SvdRegister {
                        name: rname,
                        description: rdesc,
                        address_offset: offset,
                        access,
                        fields,
                    });
                }
            }
            peripherals.push(SvdPeripheral {
                name: pname,
                description: pdesc,
                base_address: base_addr,
                registers,
            });
        }
    }

    Ok(SvdDevice { name, peripherals })
}

fn find_text<'a>(node: &roxmltree::Node<'a, '_>, tag: &str) -> Option<&'a str> {
    node.children()
        .find(|n| n.has_tag_name(tag))
        .and_then(|n| n.text())
}

fn parse_hex_or_dec(s: &str) -> Option<u64> {
    let s = s.trim();
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u64::from_str_radix(hex, 16).ok()
    } else {
        s.parse::<u64>().ok()
    }
}
