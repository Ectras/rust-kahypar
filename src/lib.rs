use std::ptr::NonNull;
#[allow(non_camel_case_types)]
mod kahypar {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

use crate::kahypar::{
    kahypar_configure_context_from_file, kahypar_context_free, kahypar_context_new,
    kahypar_context_t, kahypar_create_hypergraph, kahypar_hypergraph_free, kahypar_hypergraph_t,
    kahypar_partition, kahypar_partition_hypergraph, kahypar_set_seed,
};

/// Wrapper for KaHyPar hypegraph context object
pub struct KaHyParContext {
    context: NonNull<kahypar_context_t>,
}

/// Wrapper for KaHyPar hypegraph object
pub struct KaHyParHyperGraph {
    hypergraph: NonNull<kahypar_hypergraph_t>,
}

impl KaHyParContext {
    /// Creates a new blank context.
    /// # Examples
    ///
    /// ```
    /// # use kahypar_sys::KaHyParContext;
    /// let context = KaHyParContext::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            Self {
                context: NonNull::new(kahypar_context_new()).unwrap(),
            }
        }
    }

    /// Configures KaHyParContext object via input file
    pub fn configure(&mut self, config_file: String) {
        unsafe {
            kahypar_configure_context_from_file(
                self.context.as_ptr(),
                config_file.as_ptr() as *mut i8,
            )
        }
    }

    /// Set seed for non-deterministic partitioning.
    ///
    ///# Examples
    /// ```
    /// # use kahypar_sys::KaHyParContext;
    /// let mut context = KaHyParContext::new();
    /// context.set_seed(43);
    /// ```
    pub fn set_seed(&mut self, seed: i32) {
        unsafe {
            kahypar_set_seed(self.context.as_mut(), seed);
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
    /// Set seed for non-deterministic partitioning.
    ///
    ///# Examples
    /// ```
    /// # use kahypar_sys::KaHyParContext;
    /// # use kahypar_sys::KaHyParHyperGraph;
    /// let mut context = KaHyParContext::new();
    /// let num_blocks = 1;
    /// let num_vertices = 7; let num_hyperedges = 4;
    /// let hyperedge_indices = &[0,2,6,9,12];
    /// let hyperedges = &[0,2,0,1,3,4,3,4,6,2,5,6];
    /// let vertex_weights = &[1,2,3,4,5,6,7];
    /// let hyperedge_weights = &[11,22,33,44];
    /// let hypergraph = KaHyParHyperGraph::new(num_blocks,
    /// num_vertices,
    /// num_hyperedges,
    /// hyperedge_indices,
    /// hyperedges,
    /// hyperedge_weights,
    /// vertex_weights,);
    /// ```
    #[must_use]
    pub fn new(
        num_blocks: i32,
        num_vertices: u32,
        num_hyperedges: u32,
        hyperedge_indices: &[usize],
        hyperedges: &[u32],
        hyperedge_weights: &[i32],
        vertex_weights: &[i32],
    ) -> Self {
        unsafe {
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
                self.hypergraph.as_mut(),
                num_blocks,
                epsilon,
                objective,
                kahypar_context.context.as_mut(),
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
    let vertex_weights = if let Some(weights) = vertex_weights {
        weights
    } else {
        Vec::with_capacity(0)
    };
    let hyperedge_weights = if let Some(weights) = hyperedge_weights {
        weights
    } else {
        Vec::with_capacity(0)
    };

    unsafe {
        kahypar_partition(
            num_vertices,
            num_hyperedges,
            imbalance,
            k,
            vertex_weights.as_ptr(),
            hyperedge_weights.as_ptr(),
            hyperedge_indices.as_ptr(),
            hyperedges.as_ptr(),
            objective,
            context.context.as_mut(),
            partition.as_mut_ptr(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partition() {
        let mut context = KaHyParContext::new();
        context.configure(std::string::String::from(
            "src/tests/km1_kKaHyPar_sea20.ini",
        ));
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
    }
}
