mod mr_subgraph;
mod sp_subgraph;

use crate::provide::{Edges, Graph, Neighbors, Vertices};
pub use mr_subgraph::MultiRootSubgraph;
pub use sp_subgraph::ShortestPathSubgraph;

use crate::graph::Edge;
use std::marker::PhantomData;

use super::EdgeDir;

pub trait AsSubgraph<W, E: Edge<W>>: Neighbors + Vertices + Edges<W, E> {}

pub trait AsMutSubgraph<W, E: Edge<W>>: AsSubgraph<W, E> {
    fn remove_edge(&mut self, src_id: usize, dst_id: usize, edge_id: usize);

    fn remove_vertex(&mut self, vertex_id: usize);
}

pub struct Subgraph<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> {
    #[allow(dead_code)]
    graph: &'a G,

    edges: Vec<(usize, usize, &'a E)>,
    vertices: Vec<usize>,

    phantom_w: PhantomData<W>,
    phantom_ty: PhantomData<Ty>,
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> Subgraph<'a, W, E, Ty, G> {
    pub fn init(graph: &'a G, edges: Vec<(usize, usize, &'a E)>, vertices: Vec<usize>) -> Self {
        Subgraph {
            graph,
            edges,
            vertices,

            phantom_w: PhantomData,
            phantom_ty: PhantomData,
        }
    }
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> AsMutSubgraph<W, E>
    for Subgraph<'a, W, E, Ty, G>
{
    fn remove_edge(&mut self, _: usize, _: usize, edge_id: usize) {
        self.edges.retain(|(_, _, edge)| edge.get_id() != edge_id);
    }

    fn remove_vertex(&mut self, vertex_id: usize) {
        self.edges
            .retain(|(src_id, dst_id, _)| *src_id != vertex_id && *dst_id != vertex_id);

        self.vertices.retain(|v_id| *v_id != vertex_id);
    }
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> Neighbors for Subgraph<'a, W, E, Ty, G> {
    fn neighbors(&self, src_id: usize) -> Vec<usize> {
        self.edges
            .iter()
            .filter(|(s_id, _, _)| *s_id == src_id)
            .map(|(_, dst_id, _)| *dst_id)
            .collect()
    }
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> Vertices for Subgraph<'a, W, E, Ty, G> {
    fn vertices(&self) -> Vec<usize> {
        self.vertices.iter().copied().collect()
    }
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> Edges<W, E>
    for Subgraph<'a, W, E, Ty, G>
{
    fn edges_from(&self, src_id: usize) -> Vec<(usize, &E)> {
        self.edges
            .iter()
            .filter(|(s_id, _, _)| *s_id == src_id)
            .map(|(_, dst_id, edge)| (*dst_id, *edge))
            .collect()
    }

    fn edges_between(&self, src_id: usize, dst_id: usize) -> Vec<&E> {
        self.edges
            .iter()
            .filter(|(s_id, d_id, _)| *s_id == src_id && *d_id == dst_id)
            .map(|(_, _, edge)| *edge)
            .collect()
    }

    fn edge_between(&self, src_id: usize, dst_id: usize, edge_id: usize) -> Option<&E> {
        self.edges_between(src_id, dst_id)
            .into_iter()
            .find(|edge| edge.get_id() == edge_id)
    }

    fn edge(&self, edge_id: usize) -> Option<&E> {
        self.edges
            .iter()
            .find(|(_, _, edge)| edge.get_id() == edge_id)
            .and_then(|(_, _, edge)| Some(*edge))
    }

    fn as_directed_edges(&self) -> Vec<(usize, usize, &E)> {
        self.edges
            .iter()
            .flat_map(|(src_id, dst_id, edge)| {
                vec![(*src_id, *dst_id, *edge), (*dst_id, *src_id, *edge)]
            })
            .collect()
    }

    fn has_any_edge(&self, src_id: usize, dst_id: usize) -> bool {
        !self.edges_between(src_id, dst_id).is_empty()
    }

    fn edges(&self) -> Vec<(usize, usize, &E)> {
        self.edges.iter().copied().collect()
    }

    fn edges_count(&self) -> usize {
        self.edges.len()
    }
}

impl<'a, W, E: Edge<W>, Ty: EdgeDir, G: Graph<W, E, Ty>> AsSubgraph<W, E>
    for Subgraph<'a, W, E, Ty, G>
{
}
