use anyhow::anyhow;
use btf::types::Btf;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect::<Vec<_>>();
    let file = args.get(1).ok_or_else(|| anyhow!("?"))?;

    let bin_data = std::fs::read(file)?;
    let obj_file = object::File::parse(&*bin_data)?;

    let btf = Btf::load(&obj_file).map_err(|e| anyhow!("{}", e))?;
    for item in btf.core_reloc_secs().iter() {
        println!("{:#?}", item);
    }
    Ok(())
}
