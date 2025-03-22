use crate::files::MissionFile;
use hemtt_sqm::Value;

/// Mission metadata extraction trait
pub(crate) trait MetadataExtractor {
    /// Get mission name
    fn name(&self) -> Option<String>;
    /// Get mission author
    fn author(&self) -> Option<String>;
}

impl MetadataExtractor for MissionFile {
    fn name(&self) -> Option<String> {
        if let Some(sqm_data) = self.sqm_data() {
            if let Some(mission_classes) = sqm_data.classes.get("Mission") {
                for mission_class in mission_classes {
                    if let Some(intel_classes) = mission_class.classes.get("Intel") {
                        for intel_class in intel_classes {
                            if let Some(Value::String(name)) = intel_class.properties.get("briefingName") {
                                return Some(name.clone());
                            }
                        }
                    }
                }
            }
        }
        None
    }

    fn author(&self) -> Option<String> {
        if let Some(sqm_data) = self.sqm_data() {
            if let Some(scenario_data) = sqm_data.classes.get("ScenarioData") {
                for properties in scenario_data {
                    if let Some(Value::String(author)) = properties.properties.get("author") {
                        return Some(author.clone());
                    }
                }
            }
        }
        None
    }
} 