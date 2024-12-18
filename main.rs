use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::Write;

// Define a struct to map the CSV file's format
#[derive(Debug, Deserialize)]
struct Song {
    danceability: f32,
    acousticness: f32,
    energy: f32,
    valence: f32,
    tempo: f32,
    decade: String,
}

// Define a structure for PCA results and clustering
#[derive(Debug)]
struct ClusterStats {
    cluster: usize,
    danceability: f32,
    acousticness: f32,
    energy: f32,
    valence: f32,
    tempo: f32,
    count: usize,
}

impl ClusterStats {
    fn new(cluster: usize) -> Self {
        ClusterStats {
            cluster,
            danceability: 0.0,
            acousticness: 0.0,
            energy: 0.0,
            valence: 0.0,
            tempo: 0.0,
            count: 0,
        }
    }

    fn update(&mut self, song: &Song) {
        self.danceability += song.danceability;
        self.acousticness += song.acousticness;
        self.energy += song.energy;
        self.valence += song.valence;
        self.tempo += song.tempo;
        self.count += 1;
    }

    fn averages(&self) -> (f32, f32, f32, f32, f32) {
        let count = self.count as f32;
        if count == 0.0 {
            return (0.0, 0.0, 0.0, 0.0, 0.0);
        }
        (
            self.danceability / count,
            self.acousticness / count,
            self.energy / count,
            self.valence / count,
            self.tempo / count,
        )
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "top1000_songs.csv";
    let mut reader = csv::Reader::from_path(file_path)?;

    // Parse songs and store in a vector
    let mut songs: Vec<Song> = Vec::new();
    for result in reader.deserialize() {
        let song: Song = result?;
        songs.push(song);
    }

    // Normalize features for PCA
    let mut normalized_songs = normalize_features(&songs);

    // Perform PCA (dummy implementation for demonstration)
    let reduced_features = pca(&mut normalized_songs, 2);

    // Cluster the songs (dummy k-means for simplicity)
    let clusters = k_means(&reduced_features, 3);

    // Compute cluster statistics
    let mut cluster_stats: HashMap<usize, ClusterStats> = HashMap::new();
    for (idx, song) in songs.iter().enumerate() {
        let cluster_id = clusters[idx];
        let stats = cluster_stats
            .entry(cluster_id)
            .or_insert_with(|| ClusterStats::new(cluster_id));
        stats.update(song);
    }

    // Write results to a file
    let output_file = "music_clusters.csv";
    let mut writer = File::create(output_file)?;
    writeln!(
        writer,
        "Cluster,Danceability,Acousticness,Energy,Valence,Tempo,Count"
    )?;
    for stats in cluster_stats.values() {
        let (dance_avg, acoustic_avg, energy_avg, valence_avg, tempo_avg) = stats.averages();
        writeln!(
            writer,
            "{},{:.2},{:.2},{:.2},{:.2},{:.2},{}",
            stats.cluster, dance_avg, acoustic_avg, energy_avg, valence_avg, tempo_avg, stats.count
        )?;
    }

    println!("Cluster analysis written to {}", output_file);
    Ok(())
}

// Normalize features for PCA
fn normalize_features(songs: &[Song]) -> Vec<Vec<f32>> {
    let mut normalized = vec![];
    for song in songs {
        normalized.push(vec![
            song.danceability,
            song.acousticness,
            song.energy,
            song.valence,
            song.tempo,
        ]);
    }
    normalized
}

// Dummy PCA function (reduce dimensions)
fn pca(data: &mut [Vec<f32>], target_dims: usize) -> Vec<Vec<f32>> {
    data.iter()
        .map(|row| row.iter().take(target_dims).cloned().collect())
        .collect()
}

// Dummy k-means clustering
fn k_means(data: &[Vec<f32>], k: usize) -> Vec<usize> {
    let mut clusters = vec![0; data.len()];
    for (i, row) in data.iter().enumerate() {
        clusters[i] = (row.iter().sum::<f32>() as usize) % k;
    }
    clusters
}
