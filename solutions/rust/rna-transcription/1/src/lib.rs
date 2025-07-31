pub trait Nucleotide
where
    Self: std::marker::Sized,
{
    fn from_char(c: char) -> Option<Self>;
}

#[derive(Debug, PartialEq)]
pub enum DNANucleotide {
    A,
    C,
    G,
    T,
}

impl Nucleotide for DNANucleotide {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'A' => Some(DNANucleotide::A),
            'C' => Some(DNANucleotide::C),
            'G' => Some(DNANucleotide::G),
            'T' => Some(DNANucleotide::T),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RNANucleotide {
    U,
    G,
    C,
    A,
}

impl Nucleotide for RNANucleotide {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'U' => Some(RNANucleotide::U),
            'G' => Some(RNANucleotide::G),
            'C' => Some(RNANucleotide::C),
            'A' => Some(RNANucleotide::A),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Strand<T> {
    content: Vec<T>,
}

impl<T> Strand<T>
where
    T: Nucleotide,
{
    pub fn new(input: &str) -> Result<Self, &'static str> {
        match input
            .chars()
            .map(|c| T::from_char(c))
            .collect::<Option<Vec<T>>>()
        {
            Some(nucleotides) => Ok(Self {
                content: nucleotides,
            }),
            None => Err("invalid nucleotide"),
        }
    }
}

pub type DNA = Strand<DNANucleotide>;
pub type RNA = Strand<RNANucleotide>;

impl DNA {
    pub fn to_rna(&self) -> RNA {
        RNA {
            content: self.content
                .iter()
                .map(|n| match n {
                    &DNANucleotide::A => RNANucleotide::U,
                    &DNANucleotide::C => RNANucleotide::G,
                    &DNANucleotide::G => RNANucleotide::C,
                    &DNANucleotide::T => RNANucleotide::A,
                })
                .collect(),
        }
    }
}