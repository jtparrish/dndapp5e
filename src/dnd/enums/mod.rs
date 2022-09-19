#[macro_use]
mod macros {
    macro_rules! match_pred {
        ($id:ident) => {
            stringify!($id)
        };
    
        ($id:ident, $str_rep:literal) => {
            $str_rep
        };
    }
    
    macro_rules! mk_enum {
        { pub $enum_nm: ident{ $( $id:ident $( = $str_rep:literal )? ),+, }} => {
            #[derive(Debug, PartialEq)]
            pub enum $enum_nm {
                $( $id ),+,
                Unknown(String),
            }
        
            impl std::str::FromStr for $enum_nm {
                type Err = ();
            
                fn from_str(s: &str) -> Result<Self, ()> {
                    Ok(
                        match s {
                            $( match_pred!($id $(, $str_rep )?) => $enum_nm::$id),+,
                            _ => $enum_nm::Unknown(s.to_owned()),
                        }
                    )
                }
            }
        
            impl std::fmt::Display for $enum_nm {
            
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $( $enum_nm::$id => write!(f, match_pred!($id $(, $str_rep )?)) ),+,
                        $enum_nm::Unknown(s) => write!(f, "{}", &s),
                    }
                }
            }

            impl From<&str> for $enum_nm {
                fn from(s: &str) -> Self {
                    use std::str::FromStr;

                    $enum_nm::from_str(s).expect("any variant using this macro should return Unknown(_) rather than error")
                }
            }
        };

        {$enum_nm: ident{ $( $id:ident $( = $str_rep:literal )? ),+, }} => {
            #[derive(Debug, PartialEq)]
            pub enum $enum_nm {
                $( $id ),+,
                Unknown(String),
            }
        
            impl std::str::FromStr for $enum_nm {
                type Err = ();
            
                fn from_str(s: &str) -> Result<Self, ()> {
                    Ok(
                        match s {
                            $( match_pred!($id $(, $str_rep )?) => $enum_nm::$id),+,
                            _ => $enum_nm::Unknown(s.to_owned()),
                        }
                    )
                }
            }

            impl From<&str> for $enum_nm {
                fn from(s: &str) -> Self {
                    use std::str::FromStr;

                    $enum_nm::from_str(s).expect("any variant using this macro should return Unknown(_) rather than error")
                }
            }
        
            impl std::fmt::Display for $enum_nm {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

                    match self {
                        $( $enum_nm::$id => write!(f, match_pred!($id $(, $str_rep )?)) ),+,
                        $enum_nm::Unknown(s) => write!(f, "{}", &s),
                    }
                }
            }
        };
    }
    
    
    #[cfg(test)]
    mod tests {
        use std::str::FromStr;
        use std::fmt::Display;
        use std::fmt::Formatter;

        #[test]
        fn simple_test() {
            mk_enum!{ 
                TestEnum {
                    Item1 = "Item One",
                    Item2,
                    Item3,
                }
            }
            assert_eq!(TestEnum::from_str("Item1").unwrap(), TestEnum::Unknown("Item1".to_owned()));
            assert_eq!(TestEnum::from_str("Item One").unwrap(), TestEnum::Item1);
            assert_eq!(TestEnum::from_str("Item2").unwrap(), TestEnum::Item2);
            assert_eq!(TestEnum::from_str("Item3").unwrap(), TestEnum::Item3);
        }
    }
}

// pub enum Size {
//     Tiny,
//     Small,
//     Medium,
//     Large,
//     Gargantuan,
//     Unknown(String),
// }

mk_enum! {
    pub Size {
        Tiny,
        Small,
        Medium,
        Large,
        Gargantuan,
    }
}

pub mod alignment {
    // enum Order {
    //     Lawful,
    //     Neutral,
    //     Chaotic,
    //     Unknown(String),
    // }

    mk_enum! {
        pub Order {
            Lawful,
            Neutral,
            Chaotic,
        }
    }

    // enum Moral {
    //     Good,
    //     Neutral,
    //     Evil,
    //     Unknown(String),
    // }

    mk_enum! {
        pub Moral {
            Good,
            Neutral,
            Evil,
        }
    }

    pub struct Alignment(Order, Moral);
}

pub mod r#type {
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
        Undead(Option<UndeadSubtype>),
        Unknown(String, Option<String>),
    }

    impl From<(&str, Option<&str>)> for Type {
        fn from(t: (&str, Option<&str>)) -> Self {
            use Type::*;
            match t {
                ("abberation", s) => {
                    assert!(s.is_none());
                    Abberation
                },

                ("beast", s)=> {
                    assert!(s.is_none());
                    Beast
                },

                ("celestial", s)=> {
                    assert!(s.is_none());
                    Celestial
                },

                ("construct", s)=> {
                    assert!(s.is_none());
                    Construct
                },

                ("dragon", s)=> {
                    assert!(s.is_none());
                    Dragon
                },

                ("elemental", s)=> {
                    assert!(s.is_none());
                    Elemental
                },

                ("fey", s)=> {
                    assert!(s.is_none());
                    Fey
                },

                ("fiend", s)=> Fiend( s.map(|x| x.into()) ),

                ("giant", s)=> {
                    assert!(s.is_none());
                    Giant
                },

                ("humanoid", s)=>  Humanoid( s.map(|x| x.into()) ),

                ("monstrosity", s)=>  Monstrosity( s.map(|x| x.into()) ),

                ("ooze", s)=> {
                    assert!(s.is_none());
                    Ooze
                },

                ("plant", s)=> {
                    assert!(s.is_none());
                    Plant
                },

                ("swarm of tiny beasts", s)=> {
                    assert!(s.is_none());
                    SwarmOfTinyBeasts
                },

                ("Undead", s)=>  Undead( s.map(|x| x.into()) ),

                (p, s) => Unknown(p.to_owned(), s.map(|x| x.to_owned())),
            }
        }
    }

    impl std::fmt::Display for Type {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            use Type::*;
    
            match self {
                Abberation => write!(f, "abberation"),
                Beast => write!(f, "beast"),
                Celestial => write!(f, "celestial"),
                Construct => write!(f, "construct"),
                Dragon => write!(f, "dragon"),
                Elemental => write!(f, "elemental"),
                Fey => write!(f, "fey"),
                Fiend(s) => match s {
                    Some(st) => write!(f, "fiend ({})", st.to_string()),
                    None => write!(f, "fiend (none)"),
                },
                Giant => write!(f, "giant"),
                Humanoid(s) => match s {
                    Some(st) => write!(f, "humanoid ({})", st.to_string()),
                    None => write!(f, "humanoid (none)"),
                },
                Monstrosity(s) => match s {
                    Some(st) => write!(f, "monstrosity ({})", st.to_string()),
                    None => write!(f, "monstrosity (none)"),
                },
                Ooze => write!(f, "ooze"),
                Plant => write!(f, "plant"),
                SwarmOfTinyBeasts => write!(f, "swarm of tiny beasts"),
                Undead(s) => match s {
                    Some(st) => write!(f, "undead ({})", st.to_string()),
                    None => write!(f, "undead (none)"),
                },
                Unknown(t, s) => write!(f, "unknown ({}:{})", t,  match s {
                    Some(inner) => &inner,
                    None => "none",
                }),
            }
        }
    }

    // pub enum FiendSubtype {
    //     Demon,
    //     Devil,
    //     Shapechanger,
    //     Unknown(String),
    // }
    mk_enum! {
        FiendSubtype {
            Demon = "demon",
            Devil = "devil",
            Shapechanger = "shapechanger",
        }
    }

    // // NOTE: shapechanger not considered subtype
    // pub enum HumanoidSubtype {
    //     AnyRace,
    //     Dwarf,
    //     Elf,
    //     Gnoll,
    //     Gnome,
    //     Goblinoid,
    //     Grimlock,
    //     Human,
    //     Kobold,
    //     Lizardfolk,
    //     Merfolk,
    //     Orc,
    //     Sahuagin,
    //     Unknown(String),
    // }

    // NOTE: shapechanger not considered subtype
    mk_enum! {
        HumanoidSubtype {
            AnyRace = "any race",
            Dwarf = "dwarf",
            Elf = "elf",
            Gnoll = "gnoll",
            Gnome = "gnome",
            Goblinoid = "goblinoid",
            Grimlock = "grimlock",
            Human = "human",
            Kobold = "kobold",
            Lizardfolk = "lizardfolk",
            Merfolk = "merfolk",
            Orc = "orc",
            Sahuagin = "sahuagin",
        }
    }

    // pub enum MonstrositySubtype {
    //     Shapechanger,
    //     Titan,
    //     Unknown(String),
    // }

    mk_enum! {
        MonstrositySubtype {
            Shapechanger = "shapechanger",
            Titan = "titan",
        }
    }

    // pub enum UndeadSubtype {
    //     Shapechanger,
    //     Unknown(String),
    // }

    mk_enum! {
        UndeadSubtype {
            Shapechanger = "shapechanger",
        }
    }
}

// pub enum DamageTypes {
//     Acid,
//     Bludgeoning,
//     Cold,
//     Fire,
//     Force,
//     Lightning,
//     Necrotic,
//     Piercing,
//     Poison,
//     Psychic,
//     Radiant,
//     Slashing,
//     Thunder,
//     Unknown(String),
// }

mk_enum!{
    pub DamageTypes {
        Acid = "acid",
        Bludgeoning = "bludgeoning",
        Cold = "cold",
        Fire = "fire",
        Force = "force",
        Lightning = "lightning",
        Necrotic = "necrotic",
        Piercing = "piercing",
        Poison = "poison",
        Psychic = "psychic",
        Radiant = "radiant",
        Slashing = "slashing",
        Thunder = "thunder",
    }
}

// pub enum Languages {
//     Common,
//     Dwarvish,
//     Elvish,
//     Giant,
//     Gnomish,
//     Goblin,
//     Halfling,
//     Orc,
//     Abyssal,
//     Celestial,
//     Draconic,
//     DeepSpeech,
//     Infernal,
//     Primordial,
//     Sylvan,
//     Undercommon,
//     Unknown(String),
// }

mk_enum!{
    pub Languages {
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
        DeepSpeech = "Deep Speech",
        Infernal,
        Primordial,
        Sylvan,
        Undercommon,
    }
}

#[cfg(test)]
    mod tests {
        use std::str::FromStr;

        #[test]
        fn simple_test() {
            use super::Languages;

            assert_eq!(Languages::from_str("Common").unwrap(), Languages::Common);
            assert_eq!(Languages::from_str("Dwarvish").unwrap(), Languages::Dwarvish);
            assert_eq!(Languages::from_str("Elvish").unwrap(), Languages::Elvish);
            assert_eq!(Languages::from_str("Giant").unwrap(), Languages::Giant);
            assert_eq!(Languages::from_str("Gnomish").unwrap(), Languages::Gnomish);
            assert_eq!(Languages::from_str("Goblin").unwrap(), Languages::Goblin);
            assert_eq!(Languages::from_str("Halfling").unwrap(), Languages::Halfling);
            assert_eq!(Languages::from_str("Orc").unwrap(), Languages::Orc);
            assert_eq!(Languages::from_str("Abyssal").unwrap(), Languages::Abyssal);
            assert_eq!(Languages::from_str("Celestial").unwrap(), Languages::Celestial);
            assert_eq!(Languages::from_str("Draconic").unwrap(), Languages::Draconic);
            assert_eq!(Languages::from_str("Deep Speech").unwrap(), Languages::DeepSpeech);
            assert_eq!(Languages::from_str("Infernal").unwrap(), Languages::Infernal);
            assert_eq!(Languages::from_str("Primordial").unwrap(), Languages::Primordial);
            assert_eq!(Languages::from_str("Sylvan").unwrap(), Languages::Sylvan);
            assert_eq!(Languages::from_str("Undercommon").unwrap(), Languages::Undercommon);
            assert_eq!(Languages::from_str("DeepSpeech").unwrap(), Languages::Unknown("DeepSpeech".to_owned()));
            assert_eq!(Languages::from_str("English").unwrap(), Languages::Unknown("English".to_owned()));

        }
    }