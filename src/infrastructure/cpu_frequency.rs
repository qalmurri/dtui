use std::{fs, path::Path};

const CPUFREQ_ROOT: &str = "/sys/devices/system/cpu/cpufreq";

/// Return the logical-CPU-weighted current frequency in GHz when Linux exposes it.
pub fn average_ghz() -> Option<f64> {
    let root = Path::new(CPUFREQ_ROOT);
    let mut weighted_frequency = 0.0;
    let mut cpu_count = 0_usize;

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            if !name.to_string_lossy().starts_with("policy") {
                continue;
            }

            let directory = entry.path();
            let Some(frequency_khz) = read_frequency(&directory) else {
                continue;
            };
            let affected = read_cpu_count(&directory.join("affected_cpus"))
                .or_else(|| read_cpu_count(&directory.join("related_cpus")))
                .unwrap_or(1);

            weighted_frequency += frequency_khz as f64 * affected as f64;
            cpu_count += affected;
        }
    }

    if cpu_count > 0 {
        return Some(weighted_frequency / cpu_count as f64 / 1_000_000.0);
    }

    proc_cpuinfo_average_ghz()
}

fn read_frequency(policy: &Path) -> Option<u64> {
    ["cpuinfo_avg_freq", "cpuinfo_cur_freq", "scaling_cur_freq"]
        .iter()
        .find_map(|name| {
            fs::read_to_string(policy.join(name))
                .ok()?
                .trim()
                .parse()
                .ok()
        })
}

fn read_cpu_count(path: &Path) -> Option<usize> {
    let value = fs::read_to_string(path).ok()?;
    let count = value
        .trim()
        .split(',')
        .map(|part| {
            let mut bounds = part.split('-');
            let start = bounds.next()?.trim().parse::<usize>().ok()?;
            let end = bounds
                .next()
                .map_or(Some(start), |end| end.trim().parse().ok())?;
            (end >= start).then_some(end - start + 1)
        })
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .sum();
    (count > 0).then_some(count)
}

fn proc_cpuinfo_average_ghz() -> Option<f64> {
    let contents = fs::read_to_string("/proc/cpuinfo").ok()?;
    let (sum, count) = contents
        .lines()
        .filter_map(|line| {
            let (name, value) = line.split_once(':')?;
            (name.trim() == "cpu MHz")
                .then(|| value.trim().parse::<f64>().ok())
                .flatten()
        })
        .fold((0.0, 0_usize), |(sum, count), mhz| (sum + mhz, count + 1));

    (count > 0).then_some(sum / count as f64 / 1000.0)
}
