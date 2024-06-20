pub type NumberTypeUsed = f32;

// Used when number type isnt a float
// #[cfg_attr(rustfmt, rustfmt_skip)]
// const NUMBER_LOOKUP: [NumberTypeUsed; 256] = [
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,-1, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
// ];

#[derive(Debug, Clone)]
pub struct WeatherInfo {
    pub sum: NumberTypeUsed,
    pub min: NumberTypeUsed,
    pub max: NumberTypeUsed,
    pub count: usize,
}

impl WeatherInfo {
    pub fn new(bytes: &[u8]) -> Self {
        // let value: NumberTypeUsed = bytes_to_number_used(bytes);
        let value = fast_float::parse::<f32, _>(bytes).unwrap();
        WeatherInfo {
            sum: value,
            min: value,
            max: value,
            count: 1,
        }
    }

    #[allow(unused)]
    fn update(&mut self, bytes: &[u8]) {
        self.count += 1;
        // let value = bytes_to_number_used(bytes);
        let value = fast_float::parse::<f32, _>(bytes).unwrap();
        self.min = NumberTypeUsed::min(self.min, value);
        self.max = NumberTypeUsed::max(self.max, value);
        self.sum += value;
    }
}

impl Default for WeatherInfo {
    fn default() -> Self {
        WeatherInfo {
            sum: 0.0,
            min: 99999999.0,
            max: -99999999.0,
            count: 0,
        }
    }
}

impl std::ops::AddAssign for WeatherInfo {
    fn add_assign(&mut self, rhs: Self) {
        self.min = NumberTypeUsed::min(self.min, rhs.min);
        self.max = NumberTypeUsed::max(self.max, rhs.max);
        self.count += rhs.count;
        self.sum += rhs.sum;
    }
}

impl std::fmt::Display for WeatherInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{'min': {:.1}, 'mean': {:.1}, 'max': {:.1} }},",
            self.min,
            self.sum / self.count as NumberTypeUsed,
            self.max
        )
    }
}

// #[inline]
// fn bytes_to_number_used(bytes: &[u8]) -> NumberTypeUsed {
//     let sign: NumberTypeUsed = if bytes[0] == 0x2D { -1 } else { 1 };
//     let number: NumberTypeUsed = bytes
//         .iter()
//         .rev()
//         .filter(|&e| e != &0x2E && e != &0x2D)
//         .enumerate()
//         .map(|(i, &e)| {
//             let pos = NumberTypeUsed::from(10).pow(u32::try_from(i).unwrap());
//             NUMBER_LOOKUP[e as usize] * pos
//         })
//         .reduce(|a, b| a + b)
//         .unwrap();
//     number * sign
// }

/// some garbage to quickly convert to &str
/// seems like its much faster
/// :)
#[repr(C)]
#[derive(Eq)]
pub union StringUnion<'a> {
    string: &'a str,
    bytes: &'a [u8],
}

#[allow(unused)]
impl<'a> StringUnion<'a> {
    pub fn as_str(&self) -> &'a str {
        unsafe { self.string }
    }

    fn test() {
        //          67    104   101   110   110   97    105
        let value = StringUnion {
            bytes: &[0x43, 0x68, 0x65, 0x6E, 0x6E, 0x61, 0x69],
        };
        println!("as bytes {:?}", unsafe { value.bytes });
        println!("as str {:?}", unsafe { value.string });
    }
}

impl<'a> PartialEq for StringUnion<'a> {
    fn eq(&self, other: &Self) -> bool {
        let a = unsafe { self.bytes };
        let b = unsafe { other.bytes };
        a.iter().zip(b).all(|(x, y)| x == y)
    }
}
impl<'a> PartialOrd for StringUnion<'a> {
    #[allow(unused)]
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}
impl<'a> Ord for StringUnion<'a> {
    #[allow(unused)]
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}