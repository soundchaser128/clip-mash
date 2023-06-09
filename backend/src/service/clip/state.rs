use std::collections::hash_map::Entry;
use std::collections::HashMap;

use clip_mash_types::MarkerId;
use float_cmp::approx_eq;
use rand::rngs::StdRng;
use rand::seq::IteratorRandom;

use crate::service::Marker;

#[derive(Debug)]
pub struct MarkerStart {
    pub start_time: f64,
    pub end_time: f64,
    pub index: usize,
}

#[derive(Debug)]
pub struct MarkerState {
    pub data: HashMap<i64, MarkerStart>,
    pub markers: Vec<Marker>,
}

impl MarkerState {
    pub fn new(data: Vec<Marker>) -> Self {
        Self {
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
        }
    }

    pub fn get(&self, id: &MarkerId) -> Option<&MarkerStart> {
        self.data.get(&id.inner())
    }

    pub fn update(&mut self, id: &MarkerId, start_time: f64, index: usize) {
        let entry = self.data.entry(id.inner()).and_modify(|e| {
            e.start_time = start_time;
            e.index = index;
        });

        if let Entry::Occupied(e) = entry {
            if approx_eq!(f64, e.get().end_time, start_time, epsilon = 0.001) {
                e.remove();
                let index = self.markers.iter().position(|m| m.id == *id).unwrap();
                self.markers.remove(index);
            }
        }
    }

    pub fn find_marker_by_index(&self, index: usize) -> Option<Marker> {
        if self.markers.is_empty() {
            None
        } else {
            self.markers.get(index % self.markers.len()).cloned()
        }
    }

    pub fn find_marker_by_title(&self, title: &str, rng: &mut StdRng) -> Option<Marker> {
        self.markers
            .iter()
            .filter(|m| &m.title == title)
            .choose(rng)
            .cloned()
    }
}
