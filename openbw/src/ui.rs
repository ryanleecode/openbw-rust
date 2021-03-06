use super::errors::*;
use starcraft_assets;
use std::mem::MaybeUninit;

pub fn generate_bitmap(
    dimensions: &starcraft_assets::chk::Dimensions,
    megatiles: &Vec<starcraft_assets::chk::MegaTileID>,
    assets: &starcraft_assets::Assets,
) -> Result<Vec<[u8; 3]>> {
    let width = dimensions.width * 32;
    let height = dimensions.height * 32;
    let size = width * height;
    let mut out: Vec<[u8; 3]> = Vec::with_capacity(size);
    out.resize(size, unsafe { MaybeUninit::uninit().assume_init() });

    use rayon::prelude::*;
    Ok(out
        .into_par_iter()
        .enumerate()
        .map(|(i, _)| {
            let x = i % width / 32;
            let y = i / width / 32;

            let megatile = &megatiles[x + y * dimensions.width];
            let megatile_id = assets.cv5s[megatile.group_index()][megatile.subtile_index()];

            let x2 = i % width % 32 / 8;
            let y2 = i / width % 32 / 8;
            let minitile = &assets.vx4s[megatile_id][x2 + y2 * 4];
            let wpe_ref = &assets.vr4s[minitile.index()];

            let x3 = i % width % 32 % 8;
            let y3 = i / width % 32 % 8;
            let color = if minitile.is_horizontally_flipped() {
                &assets.wpes[wpe_ref[(7 - x3) + y3 * 8]]
            } else {
                &assets.wpes[wpe_ref[x3 + y3 * 8]]
            };

            color
        })
        .map(|x| x.0)
        .collect::<Vec<_>>())
}
