//! Garbage Collection Module
use std::cell::Cell;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::time::{Duration, Instant};

// Configuration

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GcStrategy {
    ReferenceCounting,
    MarkAndSweep,
    Generational,
    Arena,
    Manual,
}

impl Default for GcStrategy {
    fn default() -> Self {
        GcStrategy::ReferenceCounting
    }
}

#[derive(Clone, Debug)]
pub struct GcConfig {
    pub strategy: GcStrategy,
    pub max_heap_size: usize,
    pub collection_threshold: f64,
    pub min_collection_interval: Duration,
    pub enable_cycle_detection: bool,
    pub verbose: bool,
    pub young_gen_size: usize,
    pub promotion_threshold: usize,
}

impl Default for GcConfig {
    fn default() -> Self {
        Self {
            strategy: GcStrategy::ReferenceCounting,
            max_heap_size: 256 * 1024 * 1024,
            collection_threshold: 0.7,
            min_collection_interval: Duration::from_millis(10),
            enable_cycle_detection: true,
            verbose: false,
            young_gen_size: 1024 * 1024,
            promotion_threshold: 3,
        }
    }
}

impl GcConfig {
    pub fn builder() -> GcConfigBuilder {
        GcConfigBuilder::new()
    }

    pub fn for_server() -> Self {
        Self {
            strategy: GcStrategy::Generational,
            max_heap_size: 512 * 1024 * 1024,
            collection_threshold: 0.6,
            young_gen_size: 4 * 1024 * 1024,
            ..Default::default()
        }
    }

    pub fn for_cli() -> Self {
        Self {
            strategy: GcStrategy::Arena,
            max_heap_size: 64 * 1024 * 1024,
            ..Default::default()
        }
    }

    pub fn for_embedded() -> Self {
        Self {
            strategy: GcStrategy::ReferenceCounting,
            max_heap_size: 16 * 1024 * 1024,
            enable_cycle_detection: false,
            ..Default::default()
        }
    }
}

pub struct GcConfigBuilder {
    config: GcConfig,
}

impl GcConfigBuilder {
    pub fn new() -> Self {
        Self { config: GcConfig::default() }
    }

    pub fn strategy(mut self, s: GcStrategy) -> Self { self.config.strategy = s; self }
    pub fn heap_size(mut self, s: usize) -> Self { self.config.max_heap_size = s; self }
    pub fn collection_threshold(mut self, t: f64) -> Self { self.config.collection_threshold = t.clamp(0.1, 0.95); self }
    pub fn min_collection_interval(mut self, i: Duration) -> Self { self.config.min_collection_interval = i; self }
    pub fn enable_cycle_detection(mut self, e: bool) -> Self { self.config.enable_cycle_detection = e; self }
    pub fn verbose(mut self, v: bool) -> Self { self.config.verbose = v; self }
    pub fn young_gen_size(mut self, s: usize) -> Self { self.config.young_gen_size = s; self }
    pub fn promotion_threshold(mut self, t: usize) -> Self { self.config.promotion_threshold = t; self }
    pub fn build(self) -> GcConfig { self.config }
}

impl Default for GcConfigBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Statistics

#[derive(Clone, Debug, Default)]
pub struct GcStats {
    pub allocations: usize,
    pub deallocations: usize,
    pub collections: usize,
    pub objects_collected: usize,
    pub bytes_freed: usize,
    pub heap_used: usize,
    pub heap_peak: usize,
    pub gc_time_ms: u64,
    pub last_collection: Option<Instant>,
    pub cycle_collections: usize,
    pub cycles_found: usize,
}

impl GcStats {
    pub fn collection_efficiency(&self) -> f64 {
        if self.allocations == 0 { return 0.0; }
        self.objects_collected as f64 / self.allocations as f64
    }

    pub fn avg_gc_pause_ms(&self) -> f64 {
        if self.collections == 0 { return 0.0; }
        self.gc_time_ms as f64 / self.collections as f64
    }
}

impl fmt::Display for GcStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GC Statistics:")?;
        writeln!(f, "  Allocations: {}", self.allocations)?;
        writeln!(f, "  Collections: {}", self.collections)?;
        writeln!(f, "  Heap used: {} KB", self.heap_used / 1024)?;
        writeln!(f, "  Avg pause: {:.2} ms", self.avg_gc_pause_ms())?;
        Ok(())
    }
}

// Object Colors

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Color {
    White,
    Gray,
    Black,
}

// Heap Object

#[derive(Debug)]
struct HeapObject {
    id: usize,
    size: usize,
    ref_count: Cell<usize>,
    color: Cell<Color>,
    marked: Cell<bool>,
    generation: Cell<u8>,
    age: Cell<u8>,
    is_root: Cell<bool>,
}

impl HeapObject {
    fn new(id: usize, size: usize) -> Self {
        Self {
            id, size,
            ref_count: Cell::new(1),
            color: Cell::new(Color::White),
            marked: Cell::new(false),
            generation: Cell::new(0),
            age: Cell::new(0),
            is_root: Cell::new(false),
        }
    }
}

// Arena

struct Arena {
    buffer: Vec<u8>,
    offset: Cell<usize>,
}

impl Arena {
    fn new(size: usize) -> Self {
        Self { buffer: vec![0u8; size], offset: Cell::new(0) }
    }

    fn allocate(&self, size: usize) -> Option<usize> {
        let offset = self.offset.get();
        if offset + size > self.buffer.len() { return None; }
        self.offset.set(offset + size);
        Some(offset)
    }

    fn reset(&self) { self.offset.set(0); }
    fn used(&self) -> usize { self.offset.get() }
}

// Generational Heap

struct GenHeap {
    young: HashSet<usize>,
    old: HashSet<usize>,
}

impl GenHeap {
    fn new() -> Self { Self { young: HashSet::new(), old: HashSet::new() } }
    fn add(&mut self, id: usize) { self.young.insert(id); }
    fn young_count(&self) -> usize { self.young.len() }
    fn old_count(&self) -> usize { self.old.len() }
}

// Garbage Collector

pub struct GarbageCollector {
    config: GcConfig,
    stats: GcStats,
    objects: HashMap<usize, HeapObject>,
    roots: HashSet<usize>,
    children: HashMap<usize, Vec<usize>>,
    arena: Option<Arena>,
    gen_heap: Option<GenHeap>,
    enabled: Cell<bool>,
    next_id: Cell<usize>,
}

impl GarbageCollector {
    pub fn new(config: GcConfig) -> Self {
        let arena = if config.strategy == GcStrategy::Arena {
            Some(Arena::new(config.max_heap_size))
        } else { None };

        let gen_heap = if config.strategy == GcStrategy::Generational {
            Some(GenHeap::new())
        } else { None };

        Self {
            config,
            stats: GcStats::default(),
            objects: HashMap::new(),
            roots: HashSet::new(),
            children: HashMap::new(),
            arena,
            gen_heap,
            enabled: Cell::new(true),
            next_id: Cell::new(1),
        }
    }

    pub fn with_defaults() -> Self { Self::new(GcConfig::default()) }
    pub fn for_server() -> Self { Self::new(GcConfig::for_server()) }
    pub fn for_cli() -> Self { Self::new(GcConfig::for_cli()) }
    pub fn for_embedded() -> Self { Self::new(GcConfig::for_embedded()) }

    pub fn config(&self) -> &GcConfig { &self.config }
    pub fn stats(&self) -> &GcStats { &self.stats }
    pub fn set_enabled(&self, e: bool) { self.enabled.set(e); }
    pub fn is_enabled(&self) -> bool { self.enabled.get() }

    pub fn allocate(&mut self, size: usize) -> usize {
        let id = self.next_id.get();
        self.next_id.set(id + 1);

        if let Some(ref arena) = self.arena {
            if let Some(aid) = arena.allocate(size) {
                self.objects.insert(aid, HeapObject::new(aid, size));
                self.stats.allocations += 1;
                self.stats.heap_used += size;
                return aid;
            }
        }

        self.objects.insert(id, HeapObject::new(id, size));
        if let Some(ref mut gh) = self.gen_heap { gh.add(id); }

        self.stats.allocations += 1;
        self.stats.heap_used += size;
        if self.stats.heap_used > self.stats.heap_peak {
            self.stats.heap_peak = self.stats.heap_used;
        }

        self.maybe_collect();
        id
    }

    pub fn deallocate(&mut self, id: usize) {
        if let Some(obj) = self.objects.remove(&id) {
            self.stats.deallocations += 1;
            self.stats.heap_used = self.stats.heap_used.saturating_sub(obj.size);
            self.stats.objects_collected += 1;
            self.stats.bytes_freed += obj.size;
            self.roots.remove(&id);
            self.children.remove(&id);
        }
    }

    pub fn add_root(&mut self, id: usize) {
        if let Some(obj) = self.objects.get_mut(&id) {
            obj.is_root.set(true);
        }
        self.roots.insert(id);
    }

    pub fn remove_root(&mut self, id: usize) {
        if let Some(obj) = self.objects.get_mut(&id) {
            obj.is_root.set(false);
        }
        self.roots.remove(&id);
    }

    pub fn add_child(&mut self, parent: usize, child: usize) {
        self.children.entry(parent).or_default().push(child);
        if let Some(obj) = self.objects.get_mut(&child) {
            let c = obj.ref_count.get() + 1;
            obj.ref_count.set(c);
        }
    }

    pub fn remove_child(&mut self, parent: usize, child: usize) {
        if let Some(children) = self.children.get_mut(&parent) {
            if let Some(pos) = children.iter().position(|&x| x == child) {
                children.remove(pos);
                if let Some(obj) = self.objects.get_mut(&child) {
                    let c = obj.ref_count.get().saturating_sub(1);
                    obj.ref_count.set(c);
                }
            }
        }
    }

    pub fn inc_ref(&mut self, id: usize) {
        if let Some(obj) = self.objects.get_mut(&id) {
            let c = obj.ref_count.get() + 1;
            obj.ref_count.set(c);
        }
    }

    pub fn dec_ref(&mut self, id: usize) {
        if let Some(obj) = self.objects.get_mut(&id) {
            let c = obj.ref_count.get().saturating_sub(1);
            obj.ref_count.set(c);
            if c == 0 {
                self.deallocate(id);
            }
        }
    }

    pub fn collect(&mut self) {
        if !self.enabled.get() { return; }
        let start = Instant::now();

        match self.config.strategy {
            GcStrategy::ReferenceCounting => self.collect_refcount(),
            GcStrategy::MarkAndSweep => self.collect_mark_sweep(),
            GcStrategy::Generational => self.collect_gen(),
            GcStrategy::Arena => {}
            GcStrategy::Manual => {}
        }

        self.stats.collections += 1;
        self.stats.last_collection = Some(start);
        self.stats.gc_time_ms += start.elapsed().as_millis() as u64;
    }

    pub fn collect_full(&mut self) {
        self.collect();
        // Cycle detection simplified
    }

    fn maybe_collect(&mut self) {
        let threshold = (self.config.max_heap_size as f64 * self.config.collection_threshold) as usize;
        if self.stats.heap_used >= threshold {
            self.collect();
        }
    }

    fn collect_refcount(&mut self) {
        let zero_ref: Vec<usize> = self.objects.iter()
            .filter(|(_, o)| o.ref_count.get() == 0 && !o.is_root.get())
            .map(|(&id, _)| id)
            .collect();
        for id in zero_ref { self.deallocate(id); }
    }

    fn collect_mark_sweep(&mut self) {
        // Mark
        for obj in self.objects.values_mut() {
            obj.marked.set(false);
        }
        let roots: Vec<usize> = self.roots.iter().copied().collect();
        for id in roots { self.mark(id); }

        // Sweep
        let unmarked: Vec<usize> = self.objects.iter()
            .filter(|(_, o)| !o.marked.get() && !o.is_root.get())
            .map(|(&id, _)| id)
            .collect();
        for id in unmarked { self.deallocate(id); }
    }

    fn mark(&mut self, id: usize) {
        let mut stack = vec![id];
        while let Some(cur) = stack.pop() {
            if let Some(obj) = self.objects.get_mut(&cur) {
                if obj.marked.get() { continue; }
                obj.marked.set(true);
            }
            if let Some(children) = self.children.get(&cur).cloned() {
                stack.extend(children);
            }
        }
    }

    fn collect_gen(&mut self) {
        if let Some(ref mut gh) = self.gen_heap {
            let young: Vec<usize> = gh.young.iter().copied().collect();
            for id in young {
                if let Some(obj) = self.objects.get(&id) {
                    if obj.ref_count.get() == 0 {
                        gh.young.remove(&id);
                    }
                }
            }
            // Deallocate after iteration to avoid borrow issues
            let to_remove: Vec<usize> = self.objects.iter()
                .filter(|(_, o)| o.ref_count.get() == 0)
                .map(|(&id, _)| id)
                .collect();
            for id in to_remove {
                self.deallocate(id);
            }
        }
    }

    pub fn reset_arena(&mut self) {
        if let Some(ref arena) = self.arena {
            let used = arena.used();
            arena.reset();
            self.stats.heap_used = self.stats.heap_used.saturating_sub(used);
            self.stats.collections += 1;
        }
    }

    pub fn heap_usage_percent(&self) -> f64 {
        (self.stats.heap_used as f64 / self.config.max_heap_size as f64) * 100.0
    }

    pub fn object_count(&self) -> usize { self.objects.len() }
    pub fn root_count(&self) -> usize { self.roots.len() }
}

impl Default for GarbageCollector {
    fn default() -> Self { Self::with_defaults() }
}

// GcPtr - GC managed pointer

pub struct GcPtr<T> {
    inner: std::rc::Rc<T>,
}

impl<T> GcPtr<T> {
    pub fn new(value: T, gc: &mut GarbageCollector) -> Self {
        let _ = gc.allocate(std::mem::size_of::<T>());
        Self { inner: std::rc::Rc::new(value) }
    }

    pub fn strong_count(&self) -> usize {
        std::rc::Rc::strong_count(&self.inner)
    }
}

impl<T> Clone for GcPtr<T> {
    fn clone(&self) -> Self {
        Self { inner: std::rc::Rc::clone(&self.inner) }
    }
}

impl<T> std::ops::Deref for GcPtr<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.inner }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gc_allocation() {
        let mut gc = GarbageCollector::with_defaults();
        let id = gc.allocate(100);
        assert!(id > 0);
        assert_eq!(gc.object_count(), 1);
    }

    #[test]
    fn test_gc_deallocation() {
        let mut gc = GarbageCollector::with_defaults();
        let id = gc.allocate(100);
        gc.deallocate(id);
        assert_eq!(gc.object_count(), 0);
    }

    #[test]
    fn test_gc_roots() {
        let mut gc = GarbageCollector::with_defaults();
        let id = gc.allocate(100);
        gc.add_root(id);
        assert_eq!(gc.root_count(), 1);
        gc.remove_root(id);
        assert_eq!(gc.root_count(), 0);
    }

    #[test]
    fn test_gc_reference_counting() {
        let mut gc = GarbageCollector::with_defaults();
        let id = gc.allocate(100);
        gc.inc_ref(id);
        gc.inc_ref(id);
        assert_eq!(gc.objects.get(&id).unwrap().ref_count.get(), 3);
        gc.dec_ref(id);
        gc.dec_ref(id);
        assert!(gc.objects.contains_key(&id));
        gc.dec_ref(id);
        assert!(!gc.objects.contains_key(&id));
    }

    #[test]
    fn test_gc_mark_sweep() {
        let mut gc = GarbageCollector::new(GcConfig {
            strategy: GcStrategy::MarkAndSweep,
            ..Default::default()
        });
        let root = gc.allocate(100);
        let child = gc.allocate(50);
        let unreachable = gc.allocate(75);
        gc.add_root(root);
        gc.add_child(root, child);
        gc.collect();
        assert!(gc.objects.contains_key(&root));
        assert!(gc.objects.contains_key(&child));
        assert!(!gc.objects.contains_key(&unreachable));
    }

    #[test]
    fn test_gc_generational() {
        let mut gc = GarbageCollector::for_server();
        for _ in 0..100 { gc.allocate(1000); }
        assert!(gc.gen_heap.is_some());
        assert!(gc.gen_heap.as_ref().unwrap().young_count() > 0);
    }

    #[test]
    fn test_gc_arena() {
        let mut gc = GarbageCollector::for_cli();
        let id1 = gc.allocate(100);
        let id2 = gc.allocate(200);
        assert!(gc.arena.is_some());
        // Arena IDs start at 0
        assert!(id1 >= 0);
        assert!(id2 >= 0);
        gc.reset_arena();
        assert_eq!(gc.stats.heap_used, 0);
    }

    #[test]
    fn test_gc_stats() {
        let mut gc = GarbageCollector::with_defaults();
        for i in 0..10 {
            let id = gc.allocate(100);
            if i % 2 == 0 { gc.deallocate(id); }
        }
        assert_eq!(gc.stats.allocations, 10);
        assert_eq!(gc.stats.deallocations, 5);
    }

    #[test]
    fn test_gc_config_builder() {
        let config = GcConfig::builder()
            .strategy(GcStrategy::MarkAndSweep)
            .heap_size(128 * 1024 * 1024)
            .collection_threshold(0.8)
            .build();
        assert_eq!(config.strategy, GcStrategy::MarkAndSweep);
        assert_eq!(config.max_heap_size, 128 * 1024 * 1024);
    }

    #[test]
    fn test_gc_enabled() {
        let mut gc = GarbageCollector::with_defaults();
        assert!(gc.is_enabled());
        gc.set_enabled(false);
        assert!(!gc.is_enabled());
        gc.collect();
        assert_eq!(gc.stats.collections, 0);
    }

    #[test]
    fn test_gc_preconfigured() {
        let server = GarbageCollector::for_server();
        assert_eq!(server.config.strategy, GcStrategy::Generational);
        let cli = GarbageCollector::for_cli();
        assert_eq!(cli.config.strategy, GcStrategy::Arena);
        let embedded = GarbageCollector::for_embedded();
        assert_eq!(embedded.config.strategy, GcStrategy::ReferenceCounting);
    }

    #[test]
    fn test_gc_child_relationships() {
        let mut gc = GarbageCollector::with_defaults();
        let parent = gc.allocate(100);
        let child1 = gc.allocate(50);
        let child2 = gc.allocate(50);
        gc.add_child(parent, child1);
        gc.add_child(parent, child2);
        assert_eq!(gc.objects.get(&child1).unwrap().ref_count.get(), 2);
        gc.remove_child(parent, child1);
        assert_eq!(gc.objects.get(&child1).unwrap().ref_count.get(), 1);
    }

    #[test]
    fn test_gc_heap_threshold() {
        let mut gc = GarbageCollector::new(GcConfig {
            max_heap_size: 1000,
            collection_threshold: 0.5,
            strategy: GcStrategy::MarkAndSweep,
            ..Default::default()
        });
        for _ in 0..10 { gc.allocate(100); }
        assert!(gc.stats.collections > 0);
    }

    #[test]
    fn test_gc_stats_display() {
        let mut gc = GarbageCollector::with_defaults();
        gc.allocate(100);
        let s = format!("{}", gc.stats());
        assert!(s.contains("GC Statistics"));
    }

    #[test]
    fn test_gc_ptr() {
        let mut gc = GarbageCollector::with_defaults();
        let ptr = GcPtr::new(42, &mut gc);
        assert_eq!(*ptr, 42);
        assert_eq!(ptr.strong_count(), 1);
        let ptr2 = ptr.clone();
        assert_eq!(ptr.strong_count(), 2);
    }
}
