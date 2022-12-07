use bitpack::bitpack::{gets, getu, news, newu};
use csc411_arith::{chroma_of_index, index_of_chroma};
use crate::structs::{Block, CVC};

/// Converts a vector of word bytes into a vector of u32's
/// Returns a vector of u32's
/// # Arguments:
/// * 'vec' Vector of word bytes
pub fn extract_words(vec: Vec<[u8; 4]>) -> Vec<u32> {
    let mut words = Vec::new();

    for byte in vec.iter() {
        let word = u32::from_be_bytes(*byte);
        words.push(word);
    }
    words
}

/// Converts vector of words (u32's) into Blocks of CVC pixels
/// Returns a vector of Blocks
/// # Arguments:
/// * 'words' Vector of u32 words
pub fn unpacking(words: Vec<u32>) -> Vec<Block> {
    let mut vec = Vec::new();

    for word in words {
        let a = getu(word as u64, 9, 23);
        let b = gets(word as u64, 5, 18);
        let c = gets(word as u64, 5, 13);
        let d = gets(word as u64, 5, 8);
        let pb = getu(word as u64, 4, 4);
        let pr = getu(word as u64, 4, 0);

        let a = (a as f64 / 511.0).clamp(0.0,1.0);
        let b = (b as f64 / 50.0).clamp(-0.3,0.3);
        let c = (c as f64 / 50.0).clamp(-0.3,0.3);
        let d = (d as f64 / 50.0).clamp(-0.3,0.3);

        let y1 = a - b - c + d;
        let y2 = a - b + c - d;
        let y3 = a + b - c - d;
        let y4 = a + b + c + d;

        let pb = chroma_of_index(pb as usize);
        let pr = chroma_of_index(pr as usize);

        let block = Block {
            pix_1: make_cvc(y1, pb, pr),
            pix_2: make_cvc(y2, pb, pr),
            pix_3: make_cvc(y3, pb, pr),
            pix_4: make_cvc(y4, pb, pr),
        };
        vec.push(block);
    }
    vec
}

/// Returns a CVC pixel
/// # Arguments:
/// * 'y' y value
/// * 'pb' pb value
/// * 'pr' pr value
///
/// # Example:
/// ```
/// use rpeg::structs::CVC;
/// use rpeg::words::make_cvc;
/// let cvc_pixel = make_cvc(0.5, 0.25, 0.15);
/// let cvc_pixel_2 = CVC {
///     y: 0.5,
///     pb: 0.25,
///     pr: 0.15
/// };
/// assert_eq!(cvc_pixel, cvc_pixel_2);
/// ```
pub fn make_cvc(y: f64, pb: f32, pr: f32) -> CVC {
    let cvc = CVC {
        y: y as f64,
        pb: pb as f64,
        pr: pr as f64
    };
    cvc
}

/// Converts a vector of blocks into vector of word bytes
/// Writes to standard output
/// # Arguments:
/// * 'blocks' Vector of Blocks
/// * 'width' Width of trimmed image
/// * 'height' Height of trimmed image
pub fn packing(blocks: Vec<Block>, width: u32, height: u32) {
    let mut vec = Vec::new();
    for block in blocks {
        let chroma = chroma(block.clone());

        let a = (block.pix_4.y + block.pix_3.y + block.pix_2.y + block.pix_1.y) / 4.0;
        let b = (block.pix_4.y + block.pix_3.y - block.pix_2.y - block.pix_1.y) / 4.0;
        let c = (block.pix_4.y - block.pix_3.y + block.pix_2.y - block.pix_1.y) / 4.0;
        let d = (block.pix_4.y - block.pix_3.y - block.pix_2.y + block.pix_1.y) / 4.0;

        let a = (a * 511.0).round() as u64;
        let b = (b.clamp(-0.3,0.3) * 50.0).round() as i64;
        let c = (c.clamp(-0.3,0.3) * 50.0).round() as i64;
        let d = (d.clamp(-0.3,0.3) * 50.0).round() as i64;

        let mut word = 0_u64;
        word = newu(word, 9, 23, a).unwrap();
        word = news(word, 5, 18, b).unwrap();
        word = news(word, 5, 13, c).unwrap();
        word = news(word, 5, 8, d).unwrap();
        word = newu(word, 4, 4, chroma.0 as u64).unwrap();
        word = newu(word, 4, 0, chroma.1 as u64).unwrap();
        let raw_data =  (word as u32).to_be_bytes();
        vec.push(raw_data);
    }
    csc411_rpegio::output_rpeg_data(&vec , width, height);
}

/// Returns of the chroma values of pb and pr given a block
/// # Arguments:
/// * 'block' Block of CVC pixels
///
/// # Example:
/// ```
/// use rpeg::structs::{CVC, Block};
/// use rpeg::words::{chroma, make_cvc};
/// let block = Block {
///     pix_1: make_cvc(0.5, 0.25, 0.15),
///     pix_2: make_cvc(0.5, 0.25, 0.15),
///     pix_3: make_cvc(0.5, 0.25, 0.15),
///     pix_4: make_cvc(0.5, 0.25, 0.15),
/// };
/// chroma(block);
/// ```
pub fn chroma(block: Block) -> (usize, usize) {
    let mut average:(f64, f64) = (0.0, 0.0);
    average.0 = (block.pix_1.pb + block.pix_2.pb + block.pix_3.pb + block.pix_4.pb)/4.0;
    average.1 = (block.pix_1.pr + block.pix_2.pr + block.pix_3.pr + block.pix_4.pr)/4.0;
    return (index_of_chroma(average.0 as f32), index_of_chroma(average.1 as f32));
}
