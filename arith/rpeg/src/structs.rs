/// A struct type that represents a CVC pixel
/// # Attributes:
/// * 'y' f64
/// * 'pb' f64
/// * 'pr' f64
#[derive(Clone)]
pub struct CVC {
    pub y: f64,
    pub pb: f64,
    pub pr: f64
}

/// A struct type that represents a block of CVC pixels
/// # Attributes
/// * 'pix_1' struct CVC
/// * 'pix_2' struct CVC
/// * 'pix_3' struct CVC
/// * 'pix_4' struct CVC
#[derive(Clone)]
pub struct Block {
    pub pix_1: CVC,
    pub pix_2: CVC,
    pub pix_3: CVC,
    pub pix_4: CVC
}

/// A struct type that represents RGB values as floating point representations
/// # Attributes:
/// * 'red' f64
/// * 'blue' f64
/// * 'green' f64
#[derive(Clone)]
pub struct FloatRgb {
    pub red: f64,
    pub blue: f64,
    pub green: f64
}

pub struct UnpackedWord {
    pub a: u64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
    pub pb: usize,
    pub pr: usize,
}
