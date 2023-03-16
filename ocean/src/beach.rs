use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    crabs: Vec<Crab>,
}

impl Beach {
    pub fn new() -> Beach {
        Beach { crabs: Vec::new() }
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab)
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        if self.crabs.is_empty() {
            return None;
        }
        let fastest_crab = self.crabs.iter().max_by_key(|c| c.speed);
        fastest_crab
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        self.crabs.iter().filter(|crab| crab.name == name).collect()
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        let crab1 = self.get_crab(i);
        let crab2 = self.get_crab(j);
        let baby_crab = crab1.breed(crab2, name);
        self.add_crab(baby_crab);
    }
}
