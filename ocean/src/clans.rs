use std::collections::HashMap;

#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
	pub clans: HashMap<String, Vec<String>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem { clans: HashMap::new() }
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
		if !self.clans.get(clan_id).is_none() {
			return self.clans.get(clan_id).unwrap().to_vec();
		}
		return Vec::new();
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
//        if let Some(clan) = self.clans.get(clan_id) {
//			clan.len()
//		} else {
//			0
//		}
		match self.clans.get(clan_id) {
			None => 0,
			Some(clan) => clan.len(),
		}
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        let mut largest_id: Option<String> = None;
		let mut largest_number = 0;
		for (clan_id, clan) in &self.clans {
			if clan.len() > largest_number {
				largest_id = Some(clan_id.clone());
				largest_number = clan.len();
			}
		}
		largest_id
    }
	
	pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
		// here we do not want return any thing so match will be wordy
		if let Some(clan) = self.clans.get_mut(clan_id) {
			clan.push(String::from(crab_name));
		} else {
			self.clans.insert(String::from(clan_id).clone(), vec![String::from(crab_name)]);
		}
	}
}
