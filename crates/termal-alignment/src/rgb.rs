#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {

    // The simplest case is when R, G, and B are known; if the colour is passed as a hex integer,
    // then the following constructor can be used instead.
    pub const fn from_u32(h: u32) -> Self {
        // Ignore bits > 16
        Self {
            r: ((h >> 16) & 0xFF) as u8,
            g: ((h >> 8)  & 0xFF) as u8,
            b: ( h        & 0xFF) as u8,
        }
    }

    // The simplest case is when R, G, and B are known; if the colour is passed as a hex string,
    // then the following constructor can be used instead.
    pub fn from_hex(s: &str) -> Result<Self, &'static str> {
        let s = s.trim();

        // Strip common prefixes
        let s = s.strip_prefix("0x")
            .or_else(|| s.strip_prefix("#"))
            .unwrap_or(s);

        match s.len() {
            6 => {
                // RRGGBB
                let value = u32::from_str_radix(s, 16)
                    .map_err(|_| "invalid hex color")?;

                Ok(Self {
                    r: ((value >> 16) & 0xFF) as u8,
                    g: ((value >> 8)  & 0xFF) as u8,
                    b: ( value        & 0xFF) as u8,
                })
            }
            8 => {
                // Assume AARRGGBB, ignore alpha
                let value = u32::from_str_radix(s, 16)
                    .map_err(|_| "invalid hex color")?;

                Ok(Self {
                    r: ((value >> 16) & 0xFF) as u8,
                    g: ((value >> 8)  & 0xFF) as u8,
                    b: ( value        & 0xFF) as u8,
                })
            }
            _ => Err("hex color must have 6 or 8 digits"),
        }
    }
}

const RGB_RED: Rgb = Rgb{r: 255, g: 0, b: 0};
const RGB_GRAY: Rgb = Rgb{r: 127, g: 127, b: 127};
const RGB_WHITE: Rgb = Rgb{r: 255, g: 255, b: 255};

// In-house colors
pub const ORANGE: Rgb = Rgb{r: 255, g: 165, b: 0};
pub const SALMON: Rgb = Rgb{r: 250, g: 128, b: 114};

// ClustalX colors (source:
// https://www.cgl.ucsf.edu/chimera/1.2065/docs/ContributedSoftware/multalignviewer/colprot.par)
pub const CLUSTALX_RED: Rgb = Rgb{r: 229, g: 51, b: 25};
pub const CLUSTALX_BLUE: Rgb = Rgb{r: 25, g: 127, b: 229};
pub const CLUSTALX_GREEN: Rgb = Rgb{r: 25, g: 204, b: 25};
pub const CLUSTALX_CYAN: Rgb = Rgb{r: 25, g: 178, b: 178};
pub const CLUSTALX_PINK: Rgb = Rgb{r: 229, g: 127, b: 127};
pub const CLUSTALX_MAGENTA: Rgb = Rgb{r: 204, g: 76, b: 204};
pub const CLUSTALX_YELLOW: Rgb = Rgb{r: 204, g: 204, b: 0};
pub const CLUSTALX_ORANGE: Rgb = Rgb{r: 229, g: 153, b: 76};

// JalView Nucleotide Colors

// Contrary to the Clustal colors, I found these as hex.
pub const JALVIEW_NUCLEOTIDE_A: Rgb = Rgb::from_u32(0x0064F73F);
pub const JALVIEW_NUCLEOTIDE_C: Rgb = Rgb::from_u32(0x00FFB340);
pub const JALVIEW_NUCLEOTIDE_G: Rgb = Rgb::from_u32(0x00EB413C);
pub const JALVIEW_NUCLEOTIDE_T: Rgb = Rgb::from_u32(0x003C88EE);
pub const JALVIEW_NUCLEOTIDE_U: Rgb = Rgb::from_u32(0x003C88EE);
pub const JALVIEW_NUCLEOTIDE_I: Rgb = Rgb::from_u32(0x00ffffff);
pub const JALVIEW_NUCLEOTIDE_X: Rgb = Rgb::from_u32(0x004f6f6f);
pub const JALVIEW_NUCLEOTIDE_R: Rgb = Rgb::from_u32(0x00CD5C5C);
pub const JALVIEW_NUCLEOTIDE_Y: Rgb = Rgb::from_u32(0x00008000);
pub const JALVIEW_NUCLEOTIDE_W: Rgb = Rgb::from_u32(0x004682B4);
pub const JALVIEW_NUCLEOTIDE_S: Rgb = Rgb::from_u32(0x00FF8C00);
pub const JALVIEW_NUCLEOTIDE_M: Rgb = Rgb::from_u32(0x009ACD32);
pub const JALVIEW_NUCLEOTIDE_K: Rgb = Rgb::from_u32(0x009932CC);
pub const JALVIEW_NUCLEOTIDE_B: Rgb = Rgb::from_u32(0x008b4513);
pub const JALVIEW_NUCLEOTIDE_H: Rgb = Rgb::from_u32(0x00808080);
pub const JALVIEW_NUCLEOTIDE_D: Rgb = Rgb::from_u32(0x00483D8B);
pub const JALVIEW_NUCLEOTIDE_V: Rgb = Rgb::from_u32(0x00b8860b);
pub const JALVIEW_NUCLEOTIDE_N: Rgb = Rgb::from_u32(0x002f4f4f);


pub struct ResidueColorMap {
    table: [Rgb; 256],
}

impl ResidueColorMap {
    #[inline]
    pub fn rgb(&self, b: u8) -> Rgb {
        self.table[b as usize]
    }

    fn with_default(default: Rgb) -> Self {
        Self { table: [default; 256] }
    }

    fn set(&mut self, b: u8, color: Rgb) {
        self.table[b as usize] = color;
    }

    fn set_pair(&mut self, b: u8, color: Rgb) {
        self.set(b, color);
        self.set(b.to_ascii_lowercase(), color);
    }

    pub fn dna_basic() -> Self {
        let mut map = Self::with_default(Rgb { r: 160, g: 160, b: 160 });
        map.set_pair(b'A', Rgb { r: 0, g: 200, b: 0 });
        map.set_pair(b'T', Rgb { r: 200, g: 0, b: 0 });
        map.set_pair(b'G', Rgb { r: 255, g: 165, b: 0 });
        map.set_pair(b'C', Rgb { r: 0, g: 0, b: 200 });
        map.set(b'-', Rgb { r: 0, g: 0, b: 0 });
        map
    }

    pub fn dna_clustalx() -> Self {
        let mut map = Self::with_default(RGB_WHITE);
        map
    }
}

#[cfg(test)]
mod test {

    use super::{
        Rgb,
        RGB_RED,
        ResidueColorMap,
    };

    #[test]
    fn test_default_colormap() {
        let cmap = ResidueColorMap::with_default(RGB_RED);
        assert_eq!(RGB_RED, cmap.rgb(b'a'));
    }

    #[test]
    fn test_simple_colormap() {
        let cmap = ResidueColorMap::dna_basic();
        assert_eq!(Rgb{r:160, g:160, b:160}, cmap.rgb(b'%'));
        assert_eq!(Rgb{r:0, g:200, b:0}, cmap.rgb(b'A'));
    }

    #[test]
    fn test_dna_clustalx() {
        let cmap = ResidueColorMap::dna_clustalx();
        assert_eq!(CLUSTALX_MAGENTA, cmap.rgb(b'D'));
    }
}
