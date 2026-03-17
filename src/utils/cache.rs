use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Cache directory and file
pub const CACHE_DIR: &str = "build/cache";
pub const CACHE_FILE: &str = "build_cache.csv";

fn cache_path() -> PathBuf {
    PathBuf::from(CACHE_DIR).join(CACHE_FILE)
}

/// Compute a simple hash of file contents using DefaultHasher (no external deps)
pub fn compute_file_hash(path: &Path) -> io::Result<String> {
    let bytes = fs::read(path)?;
    let mut hasher = DefaultHasher::new();
    bytes.hash(&mut hasher);
    Ok(format!("{:x}", hasher.finish()))
}

/// Load cache from CSV (filename,hash per line). Returns empty map if missing.
pub fn load_cache() -> io::Result<HashMap<String, String>> {
    let mut map = HashMap::new();
    let p = cache_path();
    if !p.exists() {
        return Ok(map);
    }
    let f = File::open(&p)?;
    let reader = BufReader::new(f);
    for line in reader.lines() {
        let line = line?;
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        if let Some(idx) = s.rfind(',') {
            let name = s[..idx].trim().to_string();
            let hash = s[idx+1..].trim().to_string();
            if !name.is_empty() && !hash.is_empty() {
                map.insert(name, hash);
            }
        }
    }
    Ok(map)
}

/// Save cache to CSV with atomic replace (write temp then rename)
pub fn save_cache(map: &HashMap<String, String>) -> io::Result<()> {
    let p = cache_path();
    if let Some(dir) = p.parent() {
        fs::create_dir_all(dir)?;
    }
    let mut tmp = p.clone();
    tmp.set_extension("tmp");
    {
        let mut f = File::create(&tmp)?;
        for (k, v) in map.iter() {
            writeln!(f, "{},{}", k, v)?;
        }
    }
    fs::rename(tmp, p)?;
    Ok(())
}
