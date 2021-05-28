use rand::{thread_rng, Rng};
use std::cmp::{Ord, Ordering};

use super::FitnessFunction;
use std::fmt::{Display, Formatter, Debug};
use std::fmt;

#[derive(Copy, Clone)]
pub struct Individual {
    x: f32,
    y: f32,
    f: FitnessFunction
}

type Genotype = u32;

impl Individual {
    pub(super) fn random(min_bound: f32, max_bound: f32, f: FitnessFunction) -> Individual {
        let mut rng = thread_rng();
        Self::new(rng.gen_range(min_bound..=max_bound), f)
    }

    pub(super) fn new(x: f32, f: FitnessFunction) -> Individual {
        let y = f(x);
        Self{x, y, f}
    }

    pub(super) fn give_birth(genotype: Genotype, f: FitnessFunction) -> Individual {
        let x = <f32>::from_bits(genotype);
        Self::new(x, f)
    }

    pub(super) fn mate(&self, that: Individual, mutation_probability: f32) -> [Individual; 2] {
        let mut rng = thread_rng();

        let genes1 = self.genotype();
        let genes2 = that.genotype();

        let parts1 = genes1.to_be_bytes();
        let parts2 = genes2.to_be_bytes();

        let new_parts = [
            [parts1[0], parts1[1], parts2[2], parts2[3]],
            [parts2[0], parts2[1], parts1[2], parts1[3]]
        ];

        let mut descendants = [Individual::default(); 2];

        for i in 0..descendants.len() {
            let selected_parts = new_parts[i];
            let mut genes = u32::from_be_bytes(selected_parts);
            let mutate_with_probability = rng.gen_range(0.0..=1.0);

            if mutate_with_probability <= mutation_probability {
                let mask = 1u32.rotate_left(rng.gen_range(0..32));
                genes ^= mask;
            }

            descendants[i] = Self::give_birth(genes, self.f);
        }

        descendants
    }

    pub(super) fn invalid(&self) -> bool {
        self.x.is_nan() || self.x.is_infinite()
    }

    pub(super) fn lefter_then(&self, other: &Individual) -> bool {
        self.x < other.x
    }

    pub(super) fn righter_then(&self, other: &Individual) -> bool {
        self.x > other.x
    }

    pub(super) fn getX(&self) -> f32 {
        self.x.clone()
    }

    fn genotype(&self) -> Genotype {
        self.x.to_bits()
    }
}

impl Display for Individual {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Debug for Individual {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}", self))
    }
}

impl Default for Individual {
    fn default() -> Self {
        Self{x: 0.0, y: 0.0, f: |_| { 0.0 }}
    }
}


impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Individual {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y < other.y {
            Ordering::Less
        } else if self.y > other.y {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.y == other.y
    }
}

impl Eq for Individual {

}

#[cfg(test)]
mod individual_test {
    use super::Individual;

    fn add1(x: f32) -> f32 {
        x + 1f32
    }

    #[test]
    fn test_give_birth() {
        let genes = 0b0u32;
        let i = Individual::give_birth(genes, add1);
        assert_eq!(i.x, 0f32);
        assert_eq!(i.y, 1f32);

        let genes = 0b111111100000000000000000000000;
        let i = Individual::give_birth(genes, add1);
        assert_eq!(i.x, 1f32);
        assert_eq!(i.y, 2f32);
    }

    #[test]
    fn test_genotype() {
        let i = Individual::new(0f32, add1);
        let genes = i.genotype();
        assert_eq!(genes, 0b0);

        let i = Individual::new(1f32, add1);
        let genes = i.genotype();
        assert_eq!(genes, 0b111111100000000000000000000000)
    }

    #[test]
    fn test_mate_without_mutation() {
        let ancestor1 = Individual::new(2.0 / 5.0, add1);
        let ancestor2 = Individual::new(1.0 / 3.0 as f32, add1);
        let descendants = ancestor1.mate(ancestor2, 0.0);

        let opt1 = 0b111110110011001010101010101011u32;
        let opt2 = 0b111110101010101100110011001101u32;


        let fact1 = descendants[0].genotype();
        let fact2 = descendants[1].genotype();

        assert_eq!(fact1, opt1);
        assert_eq!(fact2, opt2);
    }

    #[test]
    fn test_mate_with_mutation() {
        let ancestor1 = Individual::new(2.0 / 5.0, add1);
        let ancestor2 = Individual::new(1.0 / 3.0, add1);
        let descendants = ancestor1.mate(ancestor2, 0.0);

        let get_positive = |x: &Individual| { x.genotype().count_ones() as i32 };

        let positive_genes_by_default: Vec<i32> = descendants.iter().map(get_positive).collect();

        let descendants_with_mutation = ancestor1.mate(ancestor2, 1.0);
        let positive_genes_with_mutation: Vec<i32> = descendants_with_mutation.iter().map(get_positive).collect();

        assert_eq!((positive_genes_by_default[0] - positive_genes_with_mutation[0]).abs(), 1);
        assert_eq!((positive_genes_by_default[1] - positive_genes_with_mutation[1]).abs(), 1);
    }

    #[test]
    fn test_mate_with_mutation_probability() {
        let ancestor1 = Individual::new(2.0 / 5.0, add1);
        let ancestor2 = Individual::new(1.0 / 3.0 as f32, add1);
        let descendants = ancestor1.mate(ancestor2, 0.0);

        let get_positive = |x: &Individual| { x.genotype().count_ones() as i32 };

        let positive_genes_by_default: Vec<i32> = descendants.iter().map(get_positive).collect();

        let mut mutations: f32 = 0.0;

        for _ in 0..=100000 {
            let descendants_with_mutation = ancestor1.mate(ancestor2, 0.5);
            let positive_genes_with_mutation: Vec<i32> = descendants_with_mutation.iter().map(get_positive).collect();

            for i in 0..=1 {
                if (positive_genes_by_default[i] - positive_genes_with_mutation[i]).abs() > 0 {
                    mutations += 1.0;
                }
            }
        }



        assert_eq!((mutations / 1000.0).round(), 100.0)
    }

    #[test]
    fn test_dispaly() {
        let i = Individual::new(-10.5, add1);
        assert_eq!(format!("{}", i), "(-10.5, -9.5)");

        let i = Individual::new(0.0, add1);
        assert_eq!(format!("{}", i), "(0, 1)");

        let i = Individual::new(10.5, add1);
        assert_eq!(format!("{}", i), "(10.5, 11.5)");
    }

    #[test]
    fn test_debug() {
        let i = Individual::new(-10.5, add1);
        assert_eq!(format!("{:?}", i), "(-10.5, -9.5)");

        let i = Individual::new(0.0, add1);
        assert_eq!(format!("{:?}", i), "(0, 1)");

        let i = Individual::new(10.5, add1);
        assert_eq!(format!("{:?}", i), "(10.5, 11.5)");
    }

    #[test]
    fn test_comparation() {
        let a = Individual::new(1.0, add1);
        let b = Individual::new(2.0, add1);
        let c = a.clone();

        assert!(!(a > c));

        assert!(a < b);
        assert!(b > a);
    }
}
