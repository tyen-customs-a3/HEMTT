use hemtt_workspace::WorkspacePath;

/// Types of files found in a mission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    /// mission.sqm file
    Sqm,
    /// Configuration file (description.ext, etc.)
    Config(ConfigFileType),
    /// SQF script file
    Script,
    /// Stringtable file
    StringTable,
    /// Other file type
    Other(OtherFileType),
}

impl FileType {
    /// Get all possible file types
    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![
            Self::Sqm,
            Self::Config(ConfigFileType::Ext),
            Self::Config(ConfigFileType::Code),
            Self::Config(ConfigFileType::Other),
            Self::Script,
            Self::StringTable,
            Self::Other(OtherFileType::Image),
            Self::Other(OtherFileType::Sound),
            Self::Other(OtherFileType::Video),
            Self::Other(OtherFileType::Text),
            Self::Other(OtherFileType::Binary),
        ]
    }

    /// Determine the file type from a path
    #[must_use]
    pub fn from_path(path: &WorkspacePath) -> Option<Self> {
        // Skip directories
        if path.is_dir().unwrap_or(false) {
            return None;
        }
        
        let filename = path.filename().to_lowercase();
        let extension = path.extension();
        
        tracing::debug!("Checking file type for {}: filename={}, extension={:?}", path, filename, extension);
        
        // Check for mission.sqm
        if filename == "mission.sqm" {
            tracing::debug!("Found mission.sqm");
            return Some(Self::Sqm);
        }
        
        // Check for SQF scripts - do this before config files since .sqf files are not configs
        if extension.as_ref().map_or(false, |ext| ext.to_lowercase() == "sqf") {
            tracing::debug!("Found SQF script");
            return Some(Self::Script);
        }
        
        // Check for config files
        if let Some(config_type) = ConfigFileType::from_filename(&filename) {
            tracing::debug!("Found config file: {:?}", config_type);
            return Some(Self::Config(config_type));
        }
        
        // Check for stringtable files
        if filename.contains("stringtable") && extension.as_ref().map_or(false, |ext| ext.to_lowercase() == "xml") {
            tracing::debug!("Found stringtable");
            return Some(Self::StringTable);
        }
        
        // Other files
        if let Some(file_type) = OtherFileType::from_path(path) {
            tracing::debug!("Found other file: {:?}", file_type);
            Some(Self::Other(file_type))
        } else {
            tracing::debug!("Defaulting to binary file");
            Some(Self::Other(OtherFileType::Binary))
        }
    }

    /// Check if the file type is text-based
    #[must_use]
    pub fn is_text(&self) -> bool {
        matches!(
            self,
            Self::Sqm
                | Self::Config(_)
                | Self::Script
                | Self::StringTable
                | Self::Other(OtherFileType::Text)
        )
    }

    /// Get the error identifier for this file type
    #[must_use]
    pub fn error_ident(&self) -> &'static str {
        match self {
            Self::Sqm => "MISSION_SQM_PARSE_ERROR",
            Self::Config(_) => "MISSION_CONFIG_PARSE_ERROR",
            Self::Script => "MISSION_SCRIPT_PARSE_ERROR",
            Self::StringTable => "MISSION_STRINGTABLE_PARSE_ERROR",
            Self::Other(_) => "MISSION_FILE_ERROR",
        }
    }

    /// Convert file type to string representation
    #[must_use]
    pub fn to_string(&self) -> String {
        match self {
            Self::Sqm => "mission.sqm".to_string(),
            Self::Config(config_type) => match config_type {
                ConfigFileType::Ext => "description.ext".to_string(),
                ConfigFileType::Code => "functions config".to_string(),
                ConfigFileType::Other => "other config".to_string(),
            },
            Self::Script => "script".to_string(),
            Self::StringTable => "stringtable".to_string(),
            Self::Other(other_type) => match other_type {
                OtherFileType::Image => "image".to_string(),
                OtherFileType::Sound => "sound".to_string(),
                OtherFileType::Video => "video".to_string(),
                OtherFileType::Text => "text".to_string(),
                OtherFileType::Binary => "binary".to_string(),
            },
        }
    }
}

/// Types of configuration files in a mission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConfigFileType {
    /// description.ext file
    Ext,
    /// CfgFunctions.hpp, CfgVehicles.hpp, etc.
    Code,
    /// Other header files
    Other,
}

impl ConfigFileType {
    /// Determine the config file type from a filename
    #[must_use]
    pub fn from_filename(filename: &str) -> Option<Self> {
        match filename.to_lowercase().as_str() {
            "description.ext" => Some(Self::Ext),
            _ if filename.ends_with(".hpp") || filename.ends_with(".h") => Some(Self::Code), 
            _ => Some(Self::Other),
        }
    }
}

/// Types of other files in a mission
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OtherFileType {
    /// Image file
    Image,
    /// Sound file
    Sound,
    /// Video file
    Video,
    /// Text file
    Text,
    /// Binary file
    Binary,
}

impl OtherFileType {
    /// Determine the other file type from a path
    #[must_use]
    pub fn from_path(path: &WorkspacePath) -> Option<Self> {
        path.extension()
            .map(|ext| {
                let ext = ext.to_lowercase();
                match ext.as_str() {
                    // Image files
                    "jpg" | "jpeg" | "png" | "paa" | "tga" | "bmp" => Self::Image,
                    
                    // Sound files
                    "ogg" | "wav" | "wss" => Self::Sound,
                    
                    // Video files
                    "ogv" | "mp4" => Self::Video,
                    
                    // Text files
                    "txt" | "md" | "html" | "htm" | "json" | "xml" => Self::Text,
                    
                    // Binary files
                    _ => Self::Binary,
                }
            })
    }
} 