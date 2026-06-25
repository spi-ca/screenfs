//! Perf-counter feature tests grouped by perf area.

#[path = "perf/data_path.rs"]
mod data_path;
#[path = "perf/helpers.rs"]
mod helpers;
#[path = "perf/invalidation.rs"]
mod invalidation;
#[path = "perf/lookup_stat_readlink.rs"]
mod lookup_stat_readlink;
#[path = "perf/open_like.rs"]
mod open_like;
#[path = "perf/readdir.rs"]
mod readdir;
#[path = "perf/resolved_path.rs"]
mod resolved_path;
#[path = "perf/sync.rs"]
mod sync;
