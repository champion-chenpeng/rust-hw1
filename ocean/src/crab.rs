use crate::color::Color;
use crate::cookbook::{Cookbook, Recipe};
use crate::diet::Diet;
use crate::prey::Prey;
use crate::reef::Reef;
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
        Crab { name, speed, color, diet, reefs: Vec::new() }
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

    // PART 2 BELOW
    // ------------

    /**
     * Have this crab discover a new reef, adding it to its list of reefs.
     */
    pub fn discover_reef(&mut self, reef: Rc<RefCell<Reef>>) {
        self.reefs.push(reef);
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
        let mut prey_index: Option<(Box<dyn Prey>, usize)> = None;
		for index in 0..self.reefs.len() {
			let mut reef = self.reefs[index].borrow_mut();
			let prey = reef.take_prey();
			if !prey.is_none() {
				prey_index = Some((prey?, index));
				break
			}
		}
		prey_index
	}
    /**
     * Releases the given prey back into the reef at the given index.
     */
    fn release_prey(&mut self, prey: Box<dyn Prey>, reef_index: usize) {
        self.reefs[reef_index].borrow_mut().add_prey(prey);
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
		let mut result = false;
        let mut escaped_preys = Vec::new();
		let mut prey_index_option = self.catch_prey();
		while !prey_index_option.is_none() {
			let mut prey_index = prey_index_option.unwrap();
			if prey_index.0.try_escape(self) || prey_index.0.diet() != self.diet() {
				escaped_preys.push(prey_index);
			} else {
				result = true;
			}
			prey_index_option = self.catch_prey();
		}
		for escaped_prey in escaped_preys {
			self.release_prey(escaped_prey.0, escaped_prey.1);
		}
		result
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
		let mut r: Option<&Recipe> = None;
		for recipe in cookbook.recipes() {
			if recipe.diet() == self.diet() {
				r = Some(&recipe);
			}
		}
		r
    }
}
