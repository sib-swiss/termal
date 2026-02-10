#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

const RGB_RED: Rgb = Rgb{r: 255, g: 0, b: 0};

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

}
