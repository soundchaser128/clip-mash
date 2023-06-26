use std::collections::hash_map::Entry;
use std::collections::HashMap;

use clip_mash_types::MarkerId;
use float_cmp::approx_eq;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;
use tracing::{debug, info};

use crate::service::Marker;

#[derive(Debug)]
pub struct MarkerStart {
    pub start_time: f64,
    pub end_time: f64,
    pub index: usize,
}

pub struct MarkerStateInfo {
    pub marker: Marker,
    pub start: f64,
    pub end: f64,
    pub skipped_duration: f64,
}

#[derive(Debug)]
pub struct MarkerState {
    data: HashMap<i64, Vec<MarkerStart>>,
    durations: Vec<f64>,
    markers: Vec<Marker>,
    total_duration: f64,
    length: f64,
}

impl MarkerState {
    pub fn new(data: Vec<Marker>, mut durations: Vec<f64>, length: f64) -> Self {
        durations.reverse();
        Self {
            durations,
            data: data
                .iter()
                .map(|m| {
                    (
                        m.id.inner(),
                        MarkerStart {
                            start_time: m.start_time,
                            end_time: m.end_time,
                            index: 0,
                        },
                    )
                })
                .collect(),
            markers: data,
            total_duration: 0.0,
            length,
        }
    }

    pub fn get(&self, id: &MarkerId) -> Option<&MarkerStart> {
        info!("getting marker {:?}", id);
        self.data.get(&id.inner())
    }

    pub fn update(
        &mut self,
        id: &MarkerId,
        start_time: f64,
        duration: f64,
        remaining_duration: f64,
    ) {
        self.durations.pop();
        if remaining_duration > 0.0 {
            self.durations.push(remaining_duration);
        }
        self.total_duration += duration;
        let entry = self.data.entry(id.inner()).and_modify(|e| {
            e.start_time = start_time;
            e.index += 1;
        });

        if let Entry::Occupied(e) = entry {
            if approx_eq!(f64, e.get().end_time, start_time, epsilon = 0.001) {
                e.remove();
                let index = self.markers.iter().position(|m| m.id == *id).unwrap();
                self.markers.remove(index);
            }
        }
    }

    pub fn find_marker_by_index(&self, index: usize) -> Option<MarkerStateInfo> {
        let index = index % self.markers.len();
        let next_duration = self.durations.last().copied();
        if let Some(duration) = next_duration {
            self.markers.get(index).and_then(|marker| {
                let state = self.get(&marker.id)?;
                let next_end_time = state.start_time + duration;
                let skipped_duration = if next_end_time > marker.end_time {
                    info!("next_end_time: {}, marker end time: {} for marker {}", next_end_time, marker.end_time, marker.id);
                    next_end_time - marker.end_time
                } else {
                    0.0
                };
                debug!(
                    "found marker: {}: {} - {} (skipped: {})",
                    marker.title, state.start_time, next_end_time, skipped_duration,
                );
                Some(MarkerStateInfo {
                    marker: marker.clone(),
                    start: state.start_time,
                    end: next_end_time.min(marker.end_time),
                    skipped_duration,
                })
            })
        } else {
            None
        }
    }

    pub fn find_marker_by_title(&self, title: &str, rng: &mut StdRng) -> Option<MarkerStateInfo> {
        let next_duration = self.durations.last().copied();
        if let Some(duration) = next_duration {
            self.markers
                .iter()
                .filter_map(|marker| {
                    if marker.title != title {
                        debug!("marker titles don't match: {} != {}", title, marker.title);
                        return None;
                    }
                    let state = self.get(&marker.id).unwrap();
                    let next_end_time = state.start_time + duration;
                    let skipped_duration = if next_end_time > marker.end_time {
                        next_end_time - marker.end_time
                    } else {
                        0.0
                    };
                    debug!(
                        "found marker: {}: {} - {} (skipped: {} - {} = {})",
                        marker.title,
                        state.start_time,
                        next_end_time,
                        next_end_time,
                        marker.end_time,
                        skipped_duration,
                    );
                    Some(MarkerStateInfo {
                        marker: marker.clone(),
                        start: state.start_time,
                        end: next_end_time.min(marker.end_time),
                        skipped_duration,
                    })
                })
                .choose(rng)
        } else {
            None
        }
    }

    pub fn finished(&self) -> bool {
        self.markers.is_empty() || self.total_duration >= self.length || self.durations.is_empty()
    }
}
