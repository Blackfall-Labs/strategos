# Strategos: Astromind Integration TODO

**Created:** 2026-01-08
**Context:** Ternary Thermogram Neural Architecture Plan

This file tracks required changes for Strategos to support the Astromind cognitive architecture.

## 1. Engram Format Fixes

### Issue: manifest.json Location
The current packing behavior may place `manifest.json` alongside the archive rather than inside it.

**Required Behavior:**
- `manifest.json` MUST be packed INSIDE the `.eng` archive
- Archive readers expect `ArchiveReader::read_file("manifest.json")` to work
- Current fix in Astromind uses `open_and_init()` instead of `open()` to initialize entry index

**Test Case:**
```bash
# After packing, verify manifest is inside:
strategos list archive.eng | grep manifest.json
# Should show: manifest.json (not empty)
```

### Issue: Entry Index Initialization ✅ FIXED (2026-01-11)
`ArchiveReader::open()` doesn't initialize the entry index, causing "file not found" errors.

**Solution:**
- Use `ArchiveReader::open_and_init()` OR
- Call `reader.initialize()` after `open()`
- Document this requirement clearly in API docs

**Status:** Fixed in `src/formats/engram.rs` - now uses `open_and_init()`.
All command files (`extract.rs`, `info.rs`, `list.rs`, `search.rs`, `sign.rs`, `verify.rs`) already called `initialize()` after `open()`.

## 2. Thermogram Support

### Background
Astromind is migrating from safetensors to thermograms for neural mesh weights.
Thermograms provide:
- Ternary weights (2 bits per weight, 16x compression)
- Runtime plasticity (STDP, consolidation)
- Colony growth for new knowledge regions

### Required Features

#### 2.1 Pack `.thermo` Files
Strategos should recognize and properly handle `.thermo` files when packing:

```bash
strategos pack mesh_colony/ -o brain.eng
# Should include:
#   meshes/dialogue.thermo
#   meshes/reasoning.thermo
#   meshes/planning.thermo
#   colonies/verbal/root.thermo
```

#### 2.2 Thermogram Manifest Entries
Thermogram files should have special manifest entries:

```json
{
  "entries": {
    "meshes/dialogue.thermo": {
      "type": "thermogram",
      "format_version": "1.0",
      "weight_type": "ternary",
      "neuron_count": 1024,
      "compression": "packed_ternary"
    }
  }
}
```

#### 2.3 Colony Directory Structure
Support for thermogram colonies (hierarchical mesh organization):

```
brain.eng/
  colonies/
    verbal/
      root.thermo      # Root thermogram
      dialogue.thermo  # Sub-colony
      voice.thermo     # Sub-colony
    spatial/
      root.thermo
      voxel.thermo
      planning.thermo
```

#### 2.4 Incremental Updates
Future: Support delta packing for thermogram updates:

```bash
# Pack only changed thermograms
strategos pack-delta old_brain.eng new_meshes/ -o brain_update.eng
```

## 3. Priority

| Item | Priority | Blocking? | Status |
|------|----------|-----------|--------|
| manifest.json inside archive | HIGH | Yes | ✅ Verified working |
| Entry index initialization | MEDIUM | No | ✅ Fixed |
| .thermo file support | HIGH | Yes (for Tier 1 training) | Pending |
| Colony directory support | MEDIUM | No | Pending |
| Incremental updates | LOW | No | Pending |

## 4. Related Files

- `engram-rs/src/reader.rs` - ArchiveReader implementation
- `strategos/src/pack.rs` - Packing logic
- `thermogram-rs/src/export.rs` - Thermogram export format
- `astromind/engineering/20260108-*.md` - Architecture decision context

## 5. Notes

The Astromind project is implementing a three-tier training architecture:

1. **Tier 1 (Lobe)**: Train individual meshes → export to `.thermo`
2. **Tier 2 (Coordination)**: Train cross-mesh routing
3. **Tier 3 (Adaptive)**: Runtime plasticity (no Strategos involvement)

Strategos involvement is primarily in Tier 1 export and potentially Tier 2 scenario packing.
