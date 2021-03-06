use std::collections::BTreeMap;
use std::fmt;
use zone::Zone;
extern crate serde;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Cosmogony {
    pub zones: Vec<Zone>,
    pub meta: CosmogonyMetadata,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CosmogonyMetadata {
    pub osm_filename: String,
    pub stats: CosmogonyStats,
    // errors:
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CosmogonyStats {
    pub level_counts: BTreeMap<u32, u64>,
    pub zone_type_counts: BTreeMap<String, u64>,
    pub wikidata_counts: BTreeMap<u32, u64>,
    pub zone_with_unkwown_country_rules: BTreeMap<String, usize>,
    pub unhandled_admin_level: BTreeMap<String, BTreeMap<u32, usize>>,
    pub zone_without_country: usize,
}

impl CosmogonyStats {
    pub fn compute(&mut self, zones: &Vec<Zone>) {
        for zone in zones {
            let type_ = zone.zone_type
                .map(|t| format!("{:?}", t))
                .unwrap_or("None".into());
            let count = self.zone_type_counts.entry(type_).or_insert(0);
            *count += 1;
            let level = zone.admin_level.unwrap_or(0);
            let count = self.level_counts.entry(level).or_insert(0);
            *count += 1;
            if zone.wikidata.is_some() {
                let wd_count = self.wikidata_counts.entry(level).or_insert(0);
                *wd_count += 1;
            };
        }
    }
}

impl fmt::Display for CosmogonyStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (level, count) in &self.level_counts {
            let wd = self.wikidata_counts.get(level).unwrap_or(&0u64);
            write!(f, "Admin level {}: {} element(s)\n", level, count)?;
            write!(f, "    {} with wikidata id\n", wd)?;
        }
        for (zone_type, count) in &self.zone_type_counts {
            write!(f, "{:?}: {} element(s)\n", zone_type, count)?;
        }

        Ok(())
    }
}
