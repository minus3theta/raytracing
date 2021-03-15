use obj::Obj;

fn main() -> anyhow::Result<()> {
    let pot = Obj::load("res/teapot.obj")?;
    let vtxs = &pot.data.position;
    let polys = &pot.data.objects[0].groups[0].polys;
    dbg!(vtxs.len());
    dbg!(polys.len());
    let it = polys.iter().flat_map(|p| p.0.iter()).map(|t| t.0);
    dbg!((it.clone().min(), it.max()));

    Ok(())
}
