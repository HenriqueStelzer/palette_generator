#![allow(dead_code)]

use three_d::*;

pub fn init_background() -> Result<(), Box<dyn std::error::Error>> {
    // TODO 1: Window + Context
    let window = Window::new(WindowSettings {
        max_size: None,
        initial_size: None,
        ..Default::default()
    })?;

    // TODO 2: Carregar .glb
    Loader::load(&["palette.glb"], move |mut assets| {
        let model = assets.deserialize("palette.glb").unwrap();
        let model = Model::<PhysicalMaterial>::new(&context, &model).unwrap();
    });
    // TODO 3: Gerar grelha


    // TODO 4: Render loop



    Ok(())
}