use game::CardDetails;
use std::cmp::min;

pub trait FieldCard {
    fn on_turn_over(&mut self);
    fn on_my_turn_over(&mut self);
}

#[derive(Clone)]
pub struct Creature<'a> {
    pub details: &'a CardDetails,
    pub on_flank: bool,
    pub damage: u32,
    pub armor: u32,
    pub stunned: bool,
    pub exhausted: bool,
}

impl<'a> Creature<'a> {
    pub fn new(details: &'a CardDetails) -> Self {
        Creature {
            details,
            on_flank: true,
            damage: 0,
            armor: details.armor,
            stunned: false,
            exhausted: true,
        }
    }

    pub fn max_damage(&self) -> u32 {
        self.details.power
    }

    pub fn max_armor(&self) -> u32 {
        self.details.armor
    }

    pub fn power(&self) -> u32 {
        self.details.power
    }

    pub fn do_damage(&mut self, mut damage: u32) {
        let armor_damage = min(self.armor, damage);
        self.armor -= armor_damage;
        damage -= armor_damage;
        self.damage += damage;
    }

    pub fn fight<'b>(&mut self, other: &'b mut Creature) {
        self.do_damage(other.power());
        other.do_damage(self.power());
    }

    pub fn is_alive(&self) -> bool {
        self.damage < self.power()
    }
}

impl<'a> FieldCard for Creature<'a> {
    fn on_turn_over(&mut self) {
        self.armor = self.max_armor();
    }

    fn on_my_turn_over(&mut self) {
        self.exhausted = false;
    }
}

pub struct Artifact<'a> {
    details: &'a CardDetails,
    exhausted: bool,
}

impl<'a> Artifact<'a> {
    pub fn new(details: &'a CardDetails) -> Self {
        Artifact {
            details,
            exhausted: true,
        }
    }
}

impl<'a> FieldCard for Artifact<'a> {
    fn on_turn_over(&mut self) {}

    fn on_my_turn_over(&mut self) {
        self.exhausted = false;
    }
}

#[cfg(test)]
mod test {
    use game::{CardDetails, House, Type, test::test_card};
    use field::{Creature, Artifact, FieldCard};

    struct TestFixture<'a> {
        pub creature: Creature<'a>
    }

    fn creature_fixture<F>(tf: F) where F: Fn(&mut TestFixture) {
        let card_details = test_card(
            House::Brobnar,
            Type::Creature,
            0, 2, 2,
        );
        let mut o = TestFixture { creature: Creature::new(&card_details) };
        tf(&mut o);
    }

    #[test]
    fn test_creature_reset() {
        creature_fixture(|f| {
            assert_eq!(f.creature.exhausted, true);
            assert_eq!(f.creature.armor, 2);
            f.creature.on_turn_over();
            assert_eq!(f.creature.exhausted, true);
            f.creature.on_my_turn_over();
            assert_eq!(f.creature.exhausted, false);
            assert_eq!(f.creature.max_armor(), 2);
            f.creature.armor = 0;
            f.creature.on_turn_over();
            assert_eq!(f.creature.armor, 2);
        });
    }

    #[test]
    fn test_creature_damage() {
        creature_fixture(|f| {
            assert_eq!(f.creature.armor, 2);
            assert_eq!(f.creature.damage, 0);
            f.creature.do_damage(3);
            assert_eq!(f.creature.armor, 0);
            assert_eq!(f.creature.damage, 1);
            assert!(f.creature.is_alive());
            f.creature.do_damage(2);
            assert_eq!(f.creature.damage, 3);
            assert!(!f.creature.is_alive());
        });
    }
}