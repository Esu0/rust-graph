#![allow(unused)]

use std::collections::BinaryHeap;
pub trait Heap: Default {
    type Elem;

    fn push(&mut self, elem: Self::Elem);
    fn pop(&mut self) -> Option<Self::Elem>;
}

pub trait Graph<V: Copy, H: Heap<Elem = V>> {
    fn adj_v(&self, vertex: V, heap: &mut H);
    fn dfs(&self, root: V) -> IterDfs<'_, V, H, Self> {
        IterDfs {graph: self, current: root, heap: H::default()}
    }
}

pub struct IterDfs<'a, V, H, G: ?Sized> {
    graph: &'a G,
    current: V,
    heap: H,
}

impl<'a, V: Copy, H: Heap<Elem = V>, G: ?Sized + Graph<V, H>> Iterator for IterDfs<'a, V, H, G> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> {
        // implements
        None
    }
}