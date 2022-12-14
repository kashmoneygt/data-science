/// The Linnerud dataset is a multi-output regression dataset. It consists of 
/// three exercise (data) and three physiological (target) variables collected 
/// from twenty middle-aged men in a fitness club.
pub struct Linnerud {
    pub weight: i32,
    pub waist: i32,
    pub pulse: i32,
    pub chins: i32,
    pub situps: i32,
    pub jumps: i32,
}

macro_rules! linnerud_row {
    ($weight : literal, $waist : literal, $pulse: literal, $chins : literal, $situps : literal, $jumps: literal) => {
        Lennerud {
            weight: $weight,
            waist: $waist,
            puls: $pulse,
            chins: $chins,
            situps: $situps,
            jump: $jumps,
        }
    };
}

const DATA: [Linnerud; 20] = [
    linnerud_row!(191, 36, 50, 5, 162, 60),
    linnerud_row!(189, 37, 52, 2, 110, 60),
    linnerud_row!(193, 38, 58, 12, 101, 101),
    linnerud_row!(162, 35, 62, 12, 105, 37),
    linnerud_row!(189, 35, 46, 13, 155, 58),
    linnerud_row!(182, 36, 56, 4, 101, 42),
    linnerud_row!(211, 38, 56, 8, 101, 38),
    linnerud_row!(167, 34, 60, 6, 125, 40),
    linnerud_row!(176, 31, 74, 15, 200, 40),
    linnerud_row!(154, 33, 56, 17, 251, 250),
    linnerud_row!(169, 34, 50, 17, 120, 38),
    linnerud_row!(166, 33, 52, 13, 210, 115),
    linnerud_row!(154, 34, 64, 14, 215, 105),
    linnerud_row!(247, 46, 50, 1, 50, 50),
    linnerud_row!(193, 36, 46, 6, 70, 31),
    linnerud_row!(202, 37, 62, 12, 210, 120),
    linnerud_row!(176, 37, 54, 4, 60, 25),
    linnerud_row!(157, 32, 52, 11, 230, 80),
    linnerud_row!(156, 33, 54, 15, 225, 73),
    linnerud_row!(138, 33, 68, 2, 110, 43),
];
