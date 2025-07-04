use std::ptr::NonNull;
use std::{ffi::CString, ptr};

use kahypar_sys::{
    kahypar_configure_context_from_file, kahypar_configure_context_from_string,
    kahypar_context_free, kahypar_context_new, kahypar_context_t, kahypar_create_hypergraph,
    kahypar_hypergraph_free, kahypar_hypergraph_t, kahypar_partition, kahypar_partition_hypergraph,
    kahypar_set_seed,
};

/// Wrapper for KaHyPar hypergraph context object.
pub struct KaHyParContext {
    context: NonNull<kahypar_context_t>,
}

/// Wrapper for KaHyPar hypergraph object.
pub struct KaHyParHyperGraph {
    hypergraph: NonNull<kahypar_hypergraph_t>,
}

impl KaHyParContext {
    /// Creates a new blank context.
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            Self {
                context: NonNull::new(kahypar_context_new()).unwrap(),
            }
        }
    }

    /// Configures KaHyParContext object via input file.
    pub fn configure_from_file(&mut self, config_file: CString) {
        unsafe { kahypar_configure_context_from_file(self.context.as_ptr(), config_file.as_ptr()) }
    }

    /// Configures KaHyParContext object via input string.
    pub fn configure_from_str(&mut self, config_string: CString) {
        unsafe {
            kahypar_configure_context_from_string(self.context.as_ptr(), config_string.as_ptr())
        }
    }

    /// Set seed for non-deterministic partitioning.
    pub fn set_seed(&mut self, seed: i32) {
        unsafe {
            kahypar_set_seed(self.context.as_ptr(), seed);
        }
    }
}

impl Default for KaHyParContext {
    fn default() -> Self {
        KaHyParContext::new()
    }
}

impl Drop for KaHyParContext {
    fn drop(&mut self) {
        unsafe {
            kahypar_context_free(self.context.as_ptr());
        }
    }
}

impl KaHyParHyperGraph {
    /// Creates a new KaHyPar HyperGraph object.
    ///
    /// # Arguments
    ///
    /// * `num_blocks` - Number of nodes in hypergraph
    /// * `num_vertices` - Number of vertices in hypergraph
    /// * `num_hyperedges` - Number of hyperedges in hypergraph
    /// * `hyperedge_indices` - A Vector of integers, used to index `hyperedges` to determine number of nodes for each hyperedge
    /// * `hyperedges` - A Vector of integers, indexed by `hyperedge_indices`
    /// * `hyperedge_weights` - A Vector of integers of `len(hyperedge_indices)-1`, provides integer weight to each hyperedge
    /// * `vertex_weights`- A Vector of integers of `len(num_vertices)`, provides integer weight to each node
    #[must_use]
    pub fn new(
        num_blocks: i32,
        num_vertices: u32,
        num_hyperedges: u32,
        hyperedge_indices: &[usize],
        hyperedges: &[u32],
        hyperedge_weights: Option<Vec<i32>>,
        vertex_weights: Option<Vec<i32>>,
    ) -> Self {
        unsafe {
            let hyperedge_weights =
                hyperedge_weights.unwrap_or_else(|| vec![0; num_hyperedges as usize]);
            let vertex_weights = vertex_weights.unwrap_or_else(|| vec![0; num_hyperedges as usize]);
            Self {
                hypergraph: NonNull::new(kahypar_create_hypergraph(
                    num_blocks,
                    num_vertices,
                    num_hyperedges,
                    hyperedge_indices.as_ptr(),
                    hyperedges.as_ptr(),
                    hyperedge_weights.as_ptr(),
                    vertex_weights.as_ptr(),
                ))
                .unwrap(),
            }
        }
    }

    /// Partitions the hypergraph.
    ///
    /// # Arguments
    ///
    /// * `num_blocks` - Number of partitions to create
    /// * `epsilon` - Imbalance parameter (= tolerance for size differences between partitions)
    /// * `objective` - the objective to use
    /// * `kahypar_context` - the context to use
    /// * `partition` - The output vector, matching in length with the total number of elements
    pub fn partition(
        &mut self,
        num_blocks: i32,
        epsilon: f64,
        objective: &mut i32,
        kahypar_context: &mut KaHyParContext,
        partition: &mut [i32],
    ) {
        unsafe {
            kahypar_partition_hypergraph(
                self.hypergraph.as_ptr(),
                num_blocks,
                epsilon,
                objective,
                kahypar_context.context.as_ptr(),
                partition.as_mut_ptr(),
            )
        }
    }
}

impl Drop for KaHyParHyperGraph {
    fn drop(&mut self) {
        unsafe {
            kahypar_hypergraph_free(self.hypergraph.as_ptr());
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub fn partition(
    num_vertices: u32,
    num_hyperedges: u32,
    imbalance: f64,
    k: i32,
    vertex_weights: Option<Vec<i32>>,
    hyperedge_weights: Option<Vec<i32>>,
    hyperedge_indices: &[usize],
    hyperedges: &[u32],
    objective: &mut i32,
    context: &mut KaHyParContext,
    partition: &mut [i32],
) {
    let vertex_weights = vertex_weights.unwrap_or_default();
    let hyperedge_weights = hyperedge_weights.unwrap_or_default();
    let vweights = if !vertex_weights.is_empty() {
        vertex_weights.as_ptr()
    } else {
        ptr::null()
    };
    let hweights = if !hyperedge_weights.is_empty() {
        hyperedge_weights.as_ptr()
    } else {
        ptr::null()
    };

    unsafe {
        kahypar_partition(
            num_vertices,
            num_hyperedges,
            imbalance,
            k,
            vweights,
            hweights,
            hyperedge_indices.as_ptr(),
            hyperedges.as_ptr(),
            objective,
            context.context.as_ptr(),
            partition.as_mut_ptr(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_init() {
        let _context = KaHyParContext::new();
    }

    #[test]
    fn test_set_seed() {
        let mut context = KaHyParContext::new();
        context.set_seed(43);
    }

    #[test]
    fn test_context_init_file() {
        let mut context = KaHyParContext::new();
        context.configure_from_file(
            CString::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/km1_kKaHyPar_sea20.ini"
            ))
            .unwrap(),
        );
    }

    #[test]
    fn test_context_init_str() {
        let mut context = KaHyParContext::new();
        context.configure_from_str(
            CString::new(include_str!("../tests/km1_kKaHyPar_sea20.ini")).unwrap(),
        );
    }

    #[test]
    fn test_hypergraph_init() {
        let num_blocks = 1;
        let num_vertices = 7;
        let num_hyperedges = 4;
        let hyperedge_indices = &[0, 2, 6, 9, 12];
        let hyperedges = &[0, 2, 0, 1, 3, 4, 3, 4, 6, 2, 5, 6];
        let vertex_weights = vec![1, 2, 3, 4, 5, 6, 7];
        let hyperedge_weights = vec![11, 22, 33, 44];
        let _ = KaHyParHyperGraph::new(
            num_blocks,
            num_vertices,
            num_hyperedges,
            hyperedge_indices,
            hyperedges,
            Some(hyperedge_weights),
            Some(vertex_weights),
        );
    }

    #[test]
    fn test_partition() {
        let mut context = KaHyParContext::new();
        context.configure_from_file(
            CString::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/km1_kKaHyPar_sea20.ini"
            ))
            .unwrap(),
        );
        let num_vertices = 7;
        let num_hyperedges = 4;

        let hyperedge_indices = [0, 2, 6, 9, 12];
        let hyperedges = [0, 2, 0, 1, 3, 4, 3, 4, 6, 2, 5, 6];

        let node_weights = vec![1, 2, 3, 4, 5, 6, 7];
        let hyperedge_weights = vec![1, 1000, 1, 1000];

        let imbalance = 0.03;

        let k = 2;

        let mut objective = 0;
        let mut partitioning = [-1, -1, -1, -1, -1];

        partition(
            num_vertices,
            num_hyperedges,
            imbalance,
            k,
            Some(node_weights),
            Some(hyperedge_weights),
            &hyperedge_indices,
            &hyperedges,
            &mut objective,
            &mut context,
            &mut partitioning,
        );

        assert_eq!(partitioning, [1, 1, 0, 0, 1]);
    }
}
