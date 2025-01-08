use std::{fmt::{Debug, Display}, path::PathBuf};

use convert_case::{Case, Casing};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use strum::{EnumCount, EnumIter, IntoEnumIterator, IntoStaticStr};

#[derive(Copy, Clone, Debug, Default, Hash, PartialEq, Eq, EnumIter, IntoStaticStr, EnumCount)]
pub enum Biome {
    #[default]
    NoBiome,
    BloodKelp,
    BulbZone,
    CragField,
    CrashZone,
    CraterEdge,
    DeepGrandReef,
    DeepSparseReef,
    Dunes,
    GrandReef,
    GrassyPlateaus,
    InactiveLavaZone,
    JellyshroomCaves,
    KelpForest,
    LavaLakes,
    LostRiver,
    Mountains,
    MushroomForest,
    SafeShallows,
    SeaTreadersPath,
    SparseReef,
    UnderwaterIslands,
}

impl Biome {
    pub fn associated_path(&self) -> PathBuf {
        let mut cd = std::env::current_dir().expect("Couldn't find current working directory");

        cd.push("assets");
        cd.push("png-backgrounds");

        match self {
            Self::NoBiome => cd.push("placeholder.png"),
            biome => cd.push(format!("{}.png", biome.to_string().to_case(Case::Kebab))),
        };

        cd
    }

    pub fn into_par_iter() -> impl ParallelIterator<Item = Biome> {
        //TODO: Put more effort in:
        //TODO: Don't use vec as intermediate
        //TODO: Directly onstruct parallel iterator
        Biome::iter()
            .collect::<Vec<Biome>>()
            .into_par_iter()
    }

    pub fn to_label_string(&self) -> String {
        self
            .to_string()
            .chars()
            .fold((String::new(), false), |mut state, c| {
                if c.is_uppercase() && state.1 {
                    state.0.push(' ');
                }

                state.0.push(c);
                state.1 = c.is_lowercase();
                state
            })
            .0
    }
}

impl Display for Biome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.into())
    }
}
