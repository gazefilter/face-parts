pub extern crate pair_utils;

use pair_utils::Side;

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Face {
    Eye(Eye),
    Mouth(Mouth),
    Nose(Nose),
    Jaw(Jaw),
    Eyebrow(Eyebrow),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Eyebrow {
    Edge(Side, Joint),
    Arch(Side),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Nose {
    Apex,
    Root,
    Septum,
    Nostril(Side),
    Sulcus(Side),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Jaw {
    Chin,
    Cheekbone(Side),
    Temple(Side),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Mouth {
    Philtrum,
    Corner(Side),
    Aperture(Part),
    Edge(Side, Part),
    Tubercle(Side, Part),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Eye {
    Corner(Side, Joint),
    Lid(Side, Part, Joint),
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Joint {
    Inner,
    Outer,
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum Part {
    Upper,
    Lower,
}
