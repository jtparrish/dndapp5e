enum Size {
    Tiny,
    Small,
    Medium,
    Large,
    Gargantuan,
    Unknown(String),
}

mod alignment {
    enum Order {
        Lawful,
        Neutral,
        Chaotic,
        Unknown(String),
    }

    enum Moral {
        Good,
        Neutral,
        Evil,
        Unknown(String),
    }

    pub struct Alignment(Order, Moral);
}

mod r#type {
    pub enum Type {
        Abberation,
        Beast,
        Celestial,
        Construct,
        Dragon,
        Elemental,
        Fey,
        Fiend(Option<FiendSubtype>),
        Giant,
        Humanoid(Option<HumanoidSubtype>),
        Monstrosity(Option<MonstrositySubtype>),
        Ooze,
        Plant,
        SwarmOfTinyBeasts,
        Undead(UndeadSubtype),
        Unknown(String),
    }

    pub enum FiendSubtype {
        Demon,
        Devil,
        Shapechanger,
        Unknown(String),
    }

    // NOTE: shapechanger not considered subtype
    pub enum HumanoidSubtype {
        AnyRace,
        Dwarf,
        Elf,
        Gnoll,
        Gnome,
        Goblinoid,
        Grimlock,
        Human,
        Kobold,
        Lizardfolk,
        Merfolk,
        Orc,
        Sahuagin,
        Unknown(String),
    }

    pub enum MonstrositySubtype {
        None,
        Shapechanger,
        Titan,
        Unknown(String),
    }

    pub enum UndeadSubtype {
        None,
        Shapechanger,
        Unknown(String),
    }
}

enum DamageTypes {
    Acid,
    Bludgeoning,
    Cold,
    Fire,
    Force,
    Lightning,
    Necrotic,
    Piercing,
    Poison,
    Psychic,
    Radiant,
    Slashing,
    Thunder,
    Unknown(String),
}

enum Languages {
    Common,
    Dwarvish,
    Elvish,
    Giant,
    Gnomish,
    Goblin,
    Halfling,
    Orc,
    Abyssal,
    Celestial,
    Draconic,
    DeepSpeech,
    Infernal,
    Primordial,
    Sylvan,
    Undercommon,
    Unknown(String),
}

