use std::collections::HashMap;

include!(concat!(env!("OUT_DIR"), "/codegen_dictionaries.rs"));

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
#[cfg_attr(feature = "ser", derive(Serialize))]
pub enum DictionaryType {
    #[default]
    Passwords,
    English,
    FemaleNames,
    MaleNames,
    Surnames,
    UsTvAndFilm,
    UserInputs,
}

#[derive(Clone, Debug)]
pub enum RankedDictionary<'a> {
    Bultin(&'a phf::Map<&'static str, usize>),
    Dynamic(HashMap<&'a str, usize>),
}

impl RankedDictionary<'_> {
    pub fn get(&self, key: &str) -> Option<usize> {
        match self {
            RankedDictionary::Bultin(dict) => dict.get(key).copied(),
            RankedDictionary::Dynamic(dict) => dict.get(key).copied(),
        }
    }
}

impl<'a, K> FromIterator<(K, usize)> for RankedDictionary<'a>
where
    K: AsRef<str> + 'a,
    HashMap<&'a str, usize>: FromIterator<(K, usize)>,
{
    fn from_iter<T: IntoIterator<Item = (K, usize)>>(iter: T) -> Self {
        RankedDictionary::Dynamic(iter.into_iter().collect())
    }
}

lazy_static! {
    pub(crate) static ref RANKED_DICTIONARIES: HashMap<DictionaryType, RankedDictionary<'static>> = {
        let mut dicts = HashMap::with_capacity(6);
        dicts.insert(
            DictionaryType::Passwords,
            RankedDictionary::Bultin(&PASSWORDS),
        );
        dicts.insert(
            DictionaryType::English,
            RankedDictionary::Bultin(&ENGLISH_WIKI),
        );
        dicts.insert(
            DictionaryType::FemaleNames,
            RankedDictionary::Bultin(&FEMALE_NAMES),
        );
        dicts.insert(
            DictionaryType::MaleNames,
            RankedDictionary::Bultin(&MALE_NAMES),
        );
        dicts.insert(
            DictionaryType::Surnames,
            RankedDictionary::Bultin(&SURNAMES),
        );
        dicts.insert(
            DictionaryType::UsTvAndFilm,
            RankedDictionary::Bultin(&US_TV_AND_FILM),
        );
        dicts
    };
}
