//! # plato-engine
//!
//! High-performance PLATO tile engine — Rust replacement for the Python PLATO server core.
//!
//! Handles tile lifecycle (submit → gate → store → query), room management,
//! and inter-room navigation via tag-based adjacency graphs.

/// A PLATO tile with metadata and content.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tile {
    /// Unique tile identifier.
    pub id: u64,
    /// Room this tile belongs to.
    pub room: String,
    /// Tags for adjacency graph navigation.
    pub tags: Vec<String>,
    /// Quality score from gating pipeline.
    pub quality_score: f64,
    /// Tile content as opaque bytes.
    pub content: Vec<u8>,
}

/// Quality gate result for a submitted tile.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GateResult {
    pub passed: bool,
    pub score: f64,
    pub reason: Option<String>,
}

/// Default quality gate: passes tiles with score >= 0.5.
pub fn default_gate(tile: &Tile) -> GateResult {
    let passed = tile.quality_score >= 0.5;
    GateResult {
        passed,
        score: tile.quality_score,
        reason: if passed { None } else { Some("below threshold".into()) },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_gate_passes() {
        let tile = Tile {
            id: 1, room: "default".into(), tags: vec!["test".into()],
            quality_score: 0.8, content: vec![],
        };
        let result = default_gate(&tile);
        assert!(result.passed);
        assert_eq!(result.score, 0.8);
    }

    #[test]
    fn test_default_gate_rejects() {
        let tile = Tile {
            id: 2, room: "default".into(), tags: vec![],
            quality_score: 0.2, content: vec![],
        };
        let result = default_gate(&tile);
        assert!(!result.passed);
        assert!(result.reason.is_some());
    }

    #[test]
    fn test_tile_serde_roundtrip() {
        let tile = Tile {
            id: 42, room: "music".into(), tags: vec!["gagaku".into(), "constraint".into()],
            quality_score: 0.95, content: vec![1, 2, 3],
        };
        let json = serde_json::to_string(&tile).unwrap();
        let back: Tile = serde_json::from_str(&json).unwrap();
        assert_eq!(back.id, 42);
        assert_eq!(back.tags.len(), 2);
    }
}
