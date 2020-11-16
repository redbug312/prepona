use magnitude::Magnitude;

use crate::graph::edge::Edge;

/// Represents an edge with a weight.
///
/// # Generic Parameters:
/// * `W`: Weight of the edge.
pub struct DefaultEdge<W> {
    src_id: usize,
    dst_id: usize,
    weight: Magnitude<W>,
}

impl<W> Edge<W> for DefaultEdge<W> {
    /// Initializes a default edge with the given `weight`.
    ///
    /// # Arguments:
    /// * `weight`: Weight of the edge.
    ///
    /// # Returns:
    /// * Initialized edge.
    fn init(src_id: usize, dst_id: usize, weight: Magnitude<W>) -> Self {
        DefaultEdge {
            src_id,
            dst_id,
            weight,
        }
    }

    /// # Returns:
    /// Weight of the edge.
    fn get_weight(&self) -> &Magnitude<W> {
        &self.weight
    }

    /// # Updates weight of the edge.
    ///
    /// # Arguments:
    /// * `weight`: New weight.
    fn set_weight(&mut self, weight: Magnitude<W>) {
        self.weight = weight
    }

    fn get_src_id(&self) -> usize {
        self.src_id
    }

    fn get_dst_id(&self) -> usize {
        self.dst_id
    }
}

use std::any::Any;
use std::convert::From;
/// Construct a default edge from any value.
impl<W: Any> From<(usize, usize, W)> for DefaultEdge<W> {
    /// # Arguments:
    /// * `weight`: Weight of the edge.
    ///
    /// # Returns:
    /// An initialized default edge with the specified `weight`.
    fn from((src_id, dst_id, weight): (usize, usize, W)) -> Self {
        DefaultEdge::init(src_id, dst_id, weight.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let edge = DefaultEdge::init(0, 1, 2.into());

        assert_eq!(edge.get_src_id(), 0);
        assert_eq!(edge.get_dst_id(), 1);
        assert_eq!(edge.get_weight(), &2.into());
    }

    #[test]
    fn set_weight() {
        let mut edge = DefaultEdge::init(0, 1, 2.into());

        edge.set_weight(3.into());

        assert_eq!(edge.get_weight(), &3.into());
    }

    #[test]
    fn from_triplet() {
        let edge: DefaultEdge<usize> = (0, 1, 2).into();

        assert_eq!(edge.get_src_id(), 0);
        assert_eq!(edge.get_dst_id(), 1);
        assert_eq!(edge.get_weight(), &2.into());
    }
}
