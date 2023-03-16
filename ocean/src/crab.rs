use crate::color::Color;
use crate::cookbook::{Cookbook, Recipe};
use crate::diet::Diet;
use crate::prey::{self, Prey};
use crate::reef::Reef;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Crab {
    // TODO: Add fields here (some in part 1, some in part 2)
    pub name: String,
    pub speed: u32,
    pub color: Color,
    pub diet: Diet,
    pub reefs: Vec<Rc<RefCell<Reef>>>,
}

// Do NOT implement Copy for Crab.
impl Crab {
    pub fn new(name: String, speed: u32, color: Color, diet: Diet) -> Crab {
        Crab {
            name,
            speed,
            color,
            diet,
            reefs: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn color(&self) -> &Color {
        &self.color
    }

    pub fn diet(&self) -> Diet {
        self.diet
    }

    pub fn breed(&self, other: &Crab, name: String) -> Crab {
        let diet = Diet::random_diet();
        let color = Color::cross(&self.color, &other.color);
        Crab {
            name,
            speed: 1,
            color,
            diet,
            reefs: Vec::new(),
        }
    }

    // PART 2 BELOW
    // ------------

    /**
     * Have this crab discover a new reef, adding it to its list of reefs.
     */
    pub fn discover_reef(&mut self, reef: Rc<RefCell<Reef>>) {
        self.reefs.push(reef)
    }

    /**
     * Returns Some prey from one of the reefs this crab feeds from,
     * and the index of that reef in self.reefs if able to find Some prey
     * using the `take_prey` method of Reef.
     *
     * If `take_prey` returns None, try the next reef. Try each reef only once.
     *
     * If all reefs are empty, or this crab has no reefs, return None.
     */
    fn catch_prey(&mut self) -> Option<(Box<dyn Prey>, usize)> {
        for (i, reef) in self.reefs.iter().enumerate() {
            let mut reef = reef.borrow_mut();
            if let Some(prey) = reef.take_prey() {
                return Some((prey, i));
            }
        }
        None
    }

    /**
     * Releases the given prey back into the reef at the given index.
     */
    fn release_prey(&mut self, prey: Box<dyn Prey>, reef_index: usize) {
        if let Some(reef) = self.reefs.get(reef_index) {
            if let Ok(mut reef) = reef.try_borrow_mut() {
                reef.add_prey(prey)
            }
        }
    }

    /**
     * Have this crab go hunting.
     *
     * A crab will keep trying to catch prey until it succeeds,
     * or runs out of remaining prey to try to catch.
     *
     * You should keep track of all escaped prey in a local.
     *
     * Once you have finished hunting, release all escaped prey back into
     * the reefs from whence they came _before_ you return if prey was caught.
     *
     * Your algorithm might look something like this pseudocode. The challenge
     * in this task is not intended to be in figuring out the algorithm, but
     * in figuring out _how to write it in Rust_ using what you've learned so far.
     *
     * ```text
     *     there are no escaped prey yet
     *     prey has not been caught
     *     while prey can be caught
     *       if prey escapes
     *         mark the prey as escaped
     *         try again
     *     
     *       if prey is not edible by this crab
     *         mark the prey as escaped
     *         try again
     *       
     *       prey has been caught
     *       stop trying
     *     
     *     release each escaped prey back to its reef
     *     was prey caught?
     * ```
     *
     * Note: this pseudocode reads like a terrible poem.
     */
    pub fn hunt(&mut self) -> bool {
        let mut escaped_prey = Vec::new();
        let mut prey_caught = false;

        for (i, reef_rc) in self.reefs.iter().enumerate() {
            let mut reef = reef_rc.borrow_mut();

            while let Some(mut prey) = reef.take_prey() {
                if prey.diet() == self.diet {
                    if prey.try_escape(self) {
                        escaped_prey.push((prey, i));
                    } else {
                        prey_caught = true;
                        break;
                    }
                } else {
                    escaped_prey.push((prey, i));
                }
            }

            if prey_caught {
                break;
            }
        }

        for (prey, i) in escaped_prey {
            let mut reef = self.reefs[i].borrow_mut();
            reef.add_prey(prey);
        }

        prey_caught
    }

    /**
     * Returns Some of any recipe from the given cookbook that matches the crab's diet
     * preferences, or None if no such recipe exists.
     *
     * IMPORTANT: you will need to add lifetime parameters to this function. It is
     * up to you to figure out which ones and where. Do not make any other changes
     * to the signature.
     */
    pub fn choose_recipe<'a>(&'a self, cookbook: &'a Cookbook) -> Option<&Recipe> {
        cookbook.recipes().find(|recipe| recipe.diet() == self.diet)
    }
}
