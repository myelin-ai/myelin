use super::GenomeDeriver;
use crate::constant::CROSSOVER_EXTRA_GENE_SELECTION_PROBABILITY;
use crate::genome::Genome;
use itertools::Itertools;
use myelin_random::Random;

/// Implementation of chromosomal crossover
#[derive(Debug, Clone)]
pub struct ChromosomalCrossoverGenomeDeriver {
    random: Box<dyn Random>,
}

impl ChromosomalCrossoverGenomeDeriver {
    /// Creates a new instance of [`ChromosomalCrossoverGenomeDeriver`].
    pub fn new(random: Box<dyn Random>) -> Self {
        Self { random }
    }

    fn crossover_genes<T>(&self, genome_one: Vec<T>, genome_two: Vec<T>) -> Vec<T> {
        use itertools::EitherOrBoth::*;

        genome_one
            .into_iter()
            .zip_longest(genome_two)
            .filter_map(|genes| match genes {
                Both(gene_one, gene_two) => Some(self.pick_one(gene_one, gene_two)),
                Left(gene) | Right(gene) => self.pick_extra_gene(gene),
            })
            .collect()
    }

    fn pick_one<T>(&self, gene_one: T, gene_two: T) -> T {
        if self.random.flip_coin() {
            gene_one
        } else {
            gene_two
        }
    }

    fn pick_extra_gene<T>(&self, gene: T) -> Option<T> {
        if self
            .random
            .flip_coin_with_probability(CROSSOVER_EXTRA_GENE_SELECTION_PROBABILITY)
        {
            Some(gene)
        } else {
            None
        }
    }
}

impl GenomeDeriver for ChromosomalCrossoverGenomeDeriver {
    fn derive_genome_from_parents(&self, parent_genomes: (Genome, Genome)) -> Genome {
        let (
            Genome {
                hox_genes: hox_genes_one,
                cluster_genes: cluster_genes_one,
            },
            Genome {
                hox_genes: hox_genes_two,
                cluster_genes: cluster_genes_two,
            },
        ) = parent_genomes;

        Genome {
            hox_genes: self.crossover_genes(hox_genes_one, hox_genes_two),
            cluster_genes: self.crossover_genes(cluster_genes_one, cluster_genes_two),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::genome::*;
    use mockiato::any;
    use myelin_random::RandomMock;

    fn hox_gene(cluster_index: usize) -> HoxGene {
        HoxGene {
            placement: HoxPlacement::Standalone,
            cluster_index: ClusterGeneIndex(cluster_index),
            disabled_connections: Vec::new(),
        }
    }

    fn cluster_gene(placement_neuron: usize) -> ClusterGene {
        ClusterGene {
            neurons: Vec::new(),
            connections: Vec::new(),
            placement_neuron: NeuronClusterLocalIndex(placement_neuron),
        }
    }

    #[test]
    fn derive_genome_from_parents_with_same_length_results_in_genome_with_same_length() {
        let genome_one = Genome {
            hox_genes: vec![hox_gene(0), hox_gene(1)],
            cluster_genes: vec![cluster_gene(2), cluster_gene(3), cluster_gene(4)],
        };

        let genome_two = Genome {
            hox_genes: vec![hox_gene(10), hox_gene(11)],
            cluster_genes: vec![cluster_gene(12), cluster_gene(13), cluster_gene(14)],
        };

        let expected_genome = Genome {
            hox_genes: vec![hox_gene(0), hox_gene(11)],
            cluster_genes: vec![cluster_gene(12), cluster_gene(3), cluster_gene(14)],
        };

        let mut random = RandomMock::new();
        random.expect_flip_coin_calls_in_order();
        random.expect_flip_coin().returns(true);
        random.expect_flip_coin().returns(false);
        random.expect_flip_coin().returns(false);
        random.expect_flip_coin().returns(true);
        random.expect_flip_coin().returns(false);

        let deriver = ChromosomalCrossoverGenomeDeriver::new(box random);

        let actual_genome = deriver.derive_genome_from_parents((genome_one, genome_two));

        assert_eq!(expected_genome, actual_genome);
    }

    #[test]
    fn derive_genome_from_parents_with_left_being_longer_takes_genes_from_longer_genome() {
        let genome_one = Genome {
            hox_genes: vec![hox_gene(0), hox_gene(1)],
            cluster_genes: vec![cluster_gene(2), cluster_gene(3), cluster_gene(4)],
        };

        let genome_two = Genome {
            hox_genes: vec![hox_gene(10), hox_gene(11)],
            cluster_genes: vec![cluster_gene(12)],
        };

        let expected_genome = Genome {
            hox_genes: vec![hox_gene(0), hox_gene(11)],
            cluster_genes: vec![cluster_gene(12), cluster_gene(4)],
        };

        let mut random = RandomMock::new();
        random.expect_flip_coin_calls_in_order();
        random.expect_flip_coin_with_probability_calls_in_order();
        random.expect_flip_coin().returns(true);
        random.expect_flip_coin().returns(false);
        random.expect_flip_coin().returns(false);
        random
            .expect_flip_coin_with_probability(any())
            .returns(false);
        random
            .expect_flip_coin_with_probability(any())
            .returns(true);

        let deriver = ChromosomalCrossoverGenomeDeriver::new(box random);

        let actual_genome = deriver.derive_genome_from_parents((genome_one, genome_two));

        assert_eq!(expected_genome, actual_genome);
    }

    #[test]
    fn derive_genome_from_parents_with_right_being_longer_takes_genes_from_longer_genome() {
        let genome_one = Genome {
            hox_genes: vec![hox_gene(0), hox_gene(1)],
            cluster_genes: vec![cluster_gene(2)],
        };

        let genome_two = Genome {
            hox_genes: vec![hox_gene(10), hox_gene(11)],
            cluster_genes: vec![cluster_gene(12), cluster_gene(13), cluster_gene(14)],
        };

        let expected_genome = Genome {
            hox_genes: vec![hox_gene(0), hox_gene(11)],
            cluster_genes: vec![cluster_gene(12), cluster_gene(13)],
        };

        let mut random = RandomMock::new();
        random.expect_flip_coin_calls_in_order();
        random.expect_flip_coin_with_probability_calls_in_order();
        random.expect_flip_coin().returns(true);
        random.expect_flip_coin().returns(false);
        random.expect_flip_coin().returns(false);
        random
            .expect_flip_coin_with_probability(any())
            .returns(true);
        random
            .expect_flip_coin_with_probability(any())
            .returns(false);

        let deriver = ChromosomalCrossoverGenomeDeriver::new(box random);

        let actual_genome = deriver.derive_genome_from_parents((genome_one, genome_two));

        assert_eq!(expected_genome, actual_genome);
    }
}
