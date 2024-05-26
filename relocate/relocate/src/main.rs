use anyhow::{anyhow, bail};
use btf::types::{Btf, BtfCoreRelocKind, BtfType};
use object_wrapper::{build::{self, elf::SectionData}, object::elf};

fn register_name_to_arg_idx(s: impl AsRef<str>) -> anyhow::Result<usize> {
    match s.as_ref() {
        "di" => Ok(1),
        "si" => Ok(2),
        s => bail!("Unsupported: {}", s),
    }
}

fn arg_idx_to_register_name_and_offset(idx: usize) -> anyhow::Result<(String, usize)> {
    Ok(match idx {
        1 => (String::from("x0"), 0),
        2 => (String::from("x1"), 8),
        s => bail!("Unsupported: {}", s),
    })
}

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let file = args.get(1).ok_or_else(|| anyhow!("?"))?;

    let bin_data = std::fs::read(file)?;

    // let mut binary = Binary::load(file)?;
    let mut binary = object_wrapper::object::build::elf::Builder::read(bin_data.as_slice())?;

    let obj_file = object::File::parse(&*bin_data)?;
    let btf = Btf::load(&obj_file).map_err(|e| anyhow!("{}", e))?;
    println!("{:?}", binary.sections);
    for item in btf.core_reloc_secs().iter() {
        println!("program: {}", item.name);
        let section_data = binary
            .sections
            .iter_mut()
            .find(|s| s.name.to_string() == item.name)
            .unwrap();

        for rec in item.recs.iter() {
            let base_type = if let BtfType::Struct(v) = btf.type_by_id(rec.type_id) {
                v
            } else {
                bail!("Should be a struct");
            };

            println!("{:#?}", rec);
            assert!(matches!(rec.kind, BtfCoreRelocKind::ByteOff));
            let field_name = &base_type.members[*rec.access_spec.last().unwrap()].name;
            println!("Field name: {}", field_name);
            let arg_index = register_name_to_arg_idx(field_name)?;
            let (new_reg, new_off) = arg_idx_to_register_name_and_offset(arg_index)?;
            println!("new register name {} new byte offset {}", new_reg, new_off);
            if let SectionData::Data(bytes) = &mut section_data.data {
                let bytes = bytes.to_mut();

                let modify_range = &mut bytes[rec.insn_off as usize + 4..rec.insn_off as usize + 8];
                modify_range[0] = (new_off & 0xff) as u8;
                modify_range[1] = ((new_off >> 8) & 0xff) as u8;
                modify_range[2] = ((new_off >> 16) & 0xff) as u8;
                modify_range[3] = ((new_off >> 24) & 0xff) as u8;
            } else {
                bail!("Should be bytes");
            }
            // section_data.data;
        }
    }
    println!("done...");
    // binary.update();
    // binary.save("../uprobe.bpf.new.o")?;
    let ext_sec = binary
        .sections
        .iter_mut()
        .find(|s| s.name.to_string() == ".BTF.ext")
        .unwrap();
    ext_sec.delete = true;
    let shstrtab = binary.sections.add();
    shstrtab.name = ".shstrtab".into();
    shstrtab.sh_type = elf::SHT_STRTAB;
    shstrtab.data  = build::elf::SectionData::SectionString;
    let mut output_vec = vec![];
    binary.write(&mut output_vec)?;
    std::fs::write("../uprobe_new.bpf.o", output_vec)?;
    Ok(())
}
