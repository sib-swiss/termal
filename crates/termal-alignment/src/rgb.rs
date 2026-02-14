#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {

    // The simplest case is when R, G, and B are known; if the colour is passed as a hex integer,
    // then the following constructor can be used instead. This function is marked 'const' so it
    // can be used to initialize maps at compile time.
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

    /// Returns a CSS hex color, e.g. "#00ff7f".
    pub fn to_hex(self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}

pub const RGB_RED: Rgb = Rgb{r: 255, g: 0, b: 0};
pub const RGB_GRAY: Rgb = Rgb{r: 127, g: 127, b: 127};
pub const RGB_WHITE: Rgb = Rgb{r: 255, g: 255, b: 255};

pub const GAP_COLOR: Rgb = RGB_GRAY;

// In-house colors
pub const TERMAL_ORANGE: Rgb  = Rgb {r: 255, g: 165, b: 0};
pub const TERMAL_SALMON: Rgb  = Rgb {r: 250, g: 128, b: 114};

// ASCII 8-Color palette colors
pub const ASCII_8_COLOR_GREEN: Rgb   = Rgb { r: 0,   g: 128, b: 0   };
pub const ASCII_8_COLOR_MAGENTA: Rgb = Rgb { r: 128, g: 0,   b: 128 };
pub const ASCII_8_COLOR_RED: Rgb     = Rgb { r: 128, g: 0,   b: 0   };
pub const ASCII_8_COLOR_BLUE: Rgb    = Rgb { r: 0,   g: 0,   b: 128 };

// Lesk aa colors (source?)
pub const LESK_ORANGE: Rgb = TERMAL_ORANGE;
pub const LESK_GREEN: Rgb = ASCII_8_COLOR_GREEN;
pub const LESK_BLUE: Rgb = ASCII_8_COLOR_BLUE;
pub const LESK_MAGENTA: Rgb = ASCII_8_COLOR_MAGENTA;
pub const LESK_RED: Rgb = ASCII_8_COLOR_RED;

// ClustalX aa colors (source:
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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorMapName {
    AALesk,
    AAClustalX,
    DNAJalView,
    Monochrome,
}

#[derive(Clone, Debug)]
pub struct ResidueColorMap {
    table: [Rgb; 256],
}

impl ResidueColorMap {

    pub fn with_default(gap_color: Rgb, default: Rgb) -> Self {
        let mut tbl = [default; 256];
        tbl[b'-' as usize] = gap_color;
        tbl[b'.' as usize] = gap_color;
        Self { table: tbl }
    }

    pub fn by_name(name: ColorMapName) -> Self {
        match name {
            ColorMapName::AALesk     => Self::aa_lesk(),
            ColorMapName::AAClustalX => Self::aa_clustalx(),
            ColorMapName::DNAJalView => Self::dna_jalview(),
            ColorMapName::Monochrome => Self::monochrome(),
        }
    }

    #[inline]
    pub fn rgb(&self, b: u8) -> Rgb {
        self.table[b as usize]
    }

    fn set(&mut self, b: u8, color: Rgb) {
        self.table[b as usize] = color;
    }

    pub fn set_pair(&mut self, b: u8, color: Rgb) {
        self.set(b.to_ascii_lowercase(), color);
        self.set(b.to_ascii_uppercase(), color);
    }


    pub fn monochrome() -> Self {
        Self::with_default(GAP_COLOR, RGB_WHITE)
    }

    pub fn dna_basic() -> Self {
        let mut map = Self::with_default(GAP_COLOR, Rgb { r: 160, g: 160, b: 160 });
        map.set_pair(b'A', Rgb { r: 0, g: 200, b: 0 });
        map.set_pair(b'T', Rgb { r: 200, g: 0, b: 0 });
        map.set_pair(b'G', Rgb { r: 255, g: 165, b: 0 });
        map.set_pair(b'C', Rgb { r: 0, g: 0, b: 200 });
        map
    }

    pub fn dna_jalview() -> Self {
        let mut map = Self::with_default(GAP_COLOR, RGB_WHITE);
        map.set_pair(b'A', JALVIEW_NUCLEOTIDE_A);
        map.set_pair(b'C', JALVIEW_NUCLEOTIDE_C);
        map.set_pair(b'G', JALVIEW_NUCLEOTIDE_G);
        map.set_pair(b'T', JALVIEW_NUCLEOTIDE_T);
        map.set_pair(b'U', JALVIEW_NUCLEOTIDE_U);
        map.set_pair(b'I', JALVIEW_NUCLEOTIDE_I);
        map.set_pair(b'X', JALVIEW_NUCLEOTIDE_X);
        map.set_pair(b'R', JALVIEW_NUCLEOTIDE_R);
        map.set_pair(b'Y', JALVIEW_NUCLEOTIDE_Y);
        map.set_pair(b'W', JALVIEW_NUCLEOTIDE_W);
        map.set_pair(b'S', JALVIEW_NUCLEOTIDE_S);
        map.set_pair(b'M', JALVIEW_NUCLEOTIDE_M);
        map.set_pair(b'K', JALVIEW_NUCLEOTIDE_K);
        map.set_pair(b'B', JALVIEW_NUCLEOTIDE_B);
        map.set_pair(b'H', JALVIEW_NUCLEOTIDE_H);
        map.set_pair(b'D', JALVIEW_NUCLEOTIDE_D);
        map.set_pair(b'V', JALVIEW_NUCLEOTIDE_V);
        map.set_pair(b'N', JALVIEW_NUCLEOTIDE_N);
        map
    }

    pub fn aa_clustalx() -> Self {
        let mut map = Self::with_default(GAP_COLOR, RGB_WHITE);
        map.set_pair(b'G', CLUSTALX_ORANGE);
        map.set_pair(b'A', CLUSTALX_BLUE);
        map.set_pair(b'S', CLUSTALX_GREEN);
        map.set_pair(b'T', CLUSTALX_GREEN);
        map.set_pair(b'C', CLUSTALX_PINK);
        map.set_pair(b'V', CLUSTALX_BLUE);
        map.set_pair(b'I', CLUSTALX_BLUE);
        map.set_pair(b'L', CLUSTALX_BLUE);
        map.set_pair(b'P', CLUSTALX_YELLOW);
        map.set_pair(b'F', CLUSTALX_BLUE);
        map.set_pair(b'Y', CLUSTALX_CYAN);
        map.set_pair(b'M', CLUSTALX_BLUE);
        map.set_pair(b'W', CLUSTALX_BLUE);
        map.set_pair(b'N', CLUSTALX_GREEN);
        map.set_pair(b'Q', CLUSTALX_GREEN);
        map.set_pair(b'H', CLUSTALX_CYAN);
        map.set_pair(b'D', CLUSTALX_MAGENTA);
        map.set_pair(b'E', CLUSTALX_MAGENTA);
        map.set_pair(b'K', CLUSTALX_RED);
        map.set_pair(b'R', CLUSTALX_RED);
        map
    }

    pub fn aa_lesk() -> Self {
        let mut map = Self::with_default(GAP_COLOR, RGB_WHITE);
        map.set_pair(b'G', TERMAL_ORANGE);
        map.set_pair(b'A', TERMAL_ORANGE);
        map.set_pair(b'S', TERMAL_ORANGE);
        map.set_pair(b'T', TERMAL_ORANGE);
        map.set_pair(b'C', ASCII_8_COLOR_GREEN);
        map.set_pair(b'V', ASCII_8_COLOR_GREEN);
        map.set_pair(b'I', ASCII_8_COLOR_GREEN);
        map.set_pair(b'L', ASCII_8_COLOR_GREEN);
        map.set_pair(b'P', ASCII_8_COLOR_GREEN);
        map.set_pair(b'F', ASCII_8_COLOR_GREEN);
        map.set_pair(b'Y', ASCII_8_COLOR_GREEN);
        map.set_pair(b'M', ASCII_8_COLOR_GREEN);
        map.set_pair(b'W', ASCII_8_COLOR_GREEN);
        map.set_pair(b'N', ASCII_8_COLOR_MAGENTA);
        map.set_pair(b'Q', ASCII_8_COLOR_MAGENTA);
        map.set_pair(b'H', ASCII_8_COLOR_MAGENTA);
        map.set_pair(b'D', ASCII_8_COLOR_RED);
        map.set_pair(b'E', ASCII_8_COLOR_RED);
        map.set_pair(b'K', ASCII_8_COLOR_BLUE);
        map.set_pair(b'R', ASCII_8_COLOR_BLUE);
        map.set_pair(b'X', RGB_WHITE);
        map
    }

}

#[cfg(test)]
mod test {

    use super::{
        CLUSTALX_MAGENTA,
        ColorMapName,
        GAP_COLOR,
        RGB_RED,
        ResidueColorMap,
        Rgb,
    };

    #[test]
    fn test_default_colormap() {
        let cmap = ResidueColorMap::with_default(GAP_COLOR, RGB_RED);
        assert_eq!(RGB_RED, cmap.rgb(b'a'));
    }

    #[test]
    fn test_simple_colormap() {
        let cmap = ResidueColorMap::dna_basic();
        assert_eq!(Rgb{r:160, g:160, b:160}, cmap.rgb(b'%'));
        assert_eq!(Rgb{r:0, g:200, b:0}, cmap.rgb(b'A'));
    }

    #[test]
    fn test_aa_clustalx() {
        let cmap = ResidueColorMap::aa_clustalx();
        assert_eq!(CLUSTALX_MAGENTA, cmap.rgb(b'D'));
        assert_eq!(CLUSTALX_MAGENTA, cmap.rgb(b'd'));
        assert_eq!(GAP_COLOR, cmap.rgb(b'-'));
        assert_eq!(GAP_COLOR, cmap.rgb(b'.'));
    }

    #[test]
    fn test_from_u32() {
        let rgb = Rgb::from_u32(0xFF7700);
        assert_eq!(255, rgb.r);
        assert_eq!(119, rgb.g);
        assert_eq!(0, rgb.b);
    }

    #[test]
    fn test_colormap_name() {
        let cmap = ResidueColorMap::by_name(ColorMapName::AAClustalX);
        assert_eq!(CLUSTALX_MAGENTA, cmap.rgb(b'D'));
        assert_eq!(CLUSTALX_MAGENTA, cmap.rgb(b'd'));
        assert_eq!(GAP_COLOR, cmap.rgb(b'-'));
        assert_eq!(GAP_COLOR, cmap.rgb(b'.'));
    }

}
