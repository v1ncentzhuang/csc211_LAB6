use crate::structs::{Block, CVC};

/// Converts CVC pixels into 2x2 blocks of respective pixels (2x2 iterator)
/// Returns a vector of Blocks (4 CVC pixels per block)
/// # Arguments:
/// * 'cvc_pixels' Vector of CVC pixels
/// * 'width' Width of the trimmed image
/// * 'height' Height of the trimmed image
pub fn cvc_to_blocks(cvc_pixels: Vec<CVC>, width: usize, height: usize) -> Vec<Block> {
    let mut blocks:Vec<Block> = Vec::new();

    //(i,j)
    for i in (0..height).step_by(2) {
        for j in (0..width).step_by(2) {
            let temp = Block {
                pix_1: get(i, j, width.clone(), &cvc_pixels),
                pix_2: get(i, j + 1, width.clone(), &cvc_pixels),
                pix_3: get(i + 1, j, width.clone(), &cvc_pixels),
                pix_4: get(i + 1, j + 1, width.clone(), &cvc_pixels),
            };
            blocks.push(temp);
        }
    }
    blocks
}

/// Returns the value at given index using row major order
/// # Arguments:
/// * 'i' row index
/// * 'j' column index
/// * 'width' width of the trimmed image
fn get(i: usize, j: usize, width: usize, cvc_pixels: &Vec<CVC>) -> CVC {
    cvc_pixels[(i * (width)) + j].clone()
}

/// Converts Blocks into vector of CVC pixels
/// Returns a vector of CVC pixels
/// # Arguments:
/// * 'blocks' Vector of Blocks (4 CVC pixels)
/// * 'width' Width of the trimmed image
/// * 'height' Height of the trimmed image
pub fn blocks_to_cvc(blocks: Vec<Block>, width: usize, height: usize) -> Vec<CVC> {
    let mut vec = Vec::new();
    for block in blocks {
        vec.push(block.pix_1);
        vec.push(block.pix_2);
        vec.push(block.pix_3);
        vec.push(block.pix_4);
    };
    let mut cvc_pixels  = vec.clone();

    let mut counter = 0;

    for i in (0..height).step_by(2) {
        for j in (0..width).step_by(2) {
            cvc_pixels[set(i, j, width)] = vec[counter].clone();
            counter += 1;
            cvc_pixels[set(i,j + 1, width)] = vec[counter].clone();
            counter += 1;
            cvc_pixels[set(i + 1, j, width)] = vec[counter].clone();
            counter += 1;
            cvc_pixels[set(i + 1, j + 1,width)] = vec[counter].clone();
            counter += 1;
        }
    }
    cvc_pixels
}

/// Returns the index given coordinates
/// # Arguments:
/// * 'i' row index
/// * 'j' column index
/// * 'width' width of the trimmed image
fn set(i: usize, j: usize, width: usize) -> usize {
    (i * (width)) + j
}
