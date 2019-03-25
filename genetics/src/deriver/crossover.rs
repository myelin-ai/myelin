use super::GenomeDeriver;
use crate::constant::CROSSOVER_EXTRA_GENE_SELECTION_PROBABILITY;
use crate::genome::Genome;
use itertools::Itertools;
use myelin_random::RandomChanceChecker;

/// Implementation of chromosomal crossover
#[derive(Debug)]
pub struct ChromosomalCrossover {
    random_chance_checker: Box<dyn RandomChanceChecker>,
}

impl ChromosomalCrossover {
    /// Creates a new instance of [`ChromosomalCrossover`].
    pub fn new(random_chance_checker: Box<dyn RandomChanceChecker>) -> Self {
        Self {
            random_chance_checker,
        }
    }

    fn crossover_genes<T>(&mut self, genome_one: Vec<T>, genome_two: Vec<T>) -> Vec<T> {
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

    fn pick_one<T>(&mut self, gene_one: T, gene_two: T) -> T {
        if self.random_chance_checker.flip_coin() {
            gene_one
        } else {
            gene_two
        }
    }

    fn pick_extra_gene<T>(&mut self, gene: T) -> Option<T> {
        if self
            .random_chance_checker
            .flip_coin_with_probability(CROSSOVER_EXTRA_GENE_SELECTION_PROBABILITY)
        {
            Some(gene)
        } else {
            None
        }
    }
}

impl GenomeDeriver for ChromosomalCrossover {
    fn derive_genome_from_parents(&mut self, parent_genomes: (Genome, Genome)) -> Genome {
        let Genome {
            hox_genes: hox_genes_one,
            cluster_genes: cluster_genes_one,
        } = parent_genomes.0;

        let Genome {
            hox_genes: hox_genes_two,
            cluster_genes: cluster_genes_two,
        } = parent_genomes.1;

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
    use myelin_random::RandomChanceCheckerMock;

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
    fn derive_genome_from_parents_with_same_length() {
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

        let mut random_chance_checker = RandomChanceCheckerMock::default();
        random_chance_checker.expect_flip_coin().returns(true);
        random_chance_checker.expect_flip_coin().returns(false);
        random_chance_checker.expect_flip_coin().returns(false);
        random_chance_checker.expect_flip_coin().returns(true);
        random_chance_checker.expect_flip_coin().returns(false);

        let mut deriver = ChromosomalCrossover::new(box random_chance_checker);

        let actual_genome = deriver.derive_genome_from_parents((genome_one, genome_two));

        assert_eq!(expected_genome, actual_genome);
    }

    #[test]
    fn derive_genome_from_parents_with_left_being_longer() {
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

        let mut random_chance_checker = RandomChanceCheckerMock::default();
        random_chance_checker.expect_flip_coin().returns(true);
        random_chance_checker.expect_flip_coin().returns(false);
        random_chance_checker.expect_flip_coin().returns(false);
        random_chance_checker
            .expect_flip_coin_with_probability(any())
            .returns(false);
        random_chance_checker
            .expect_flip_coin_with_probability(any())
            .returns(true);

        let mut deriver = ChromosomalCrossover::new(box random_chance_checker);

        let actual_genome = deriver.derive_genome_from_parents((genome_one, genome_two));

        assert_eq!(expected_genome, actual_genome);
    }

    #[test]
    fn derive_genome_from_parents_with_right_being_longer() {
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

        let mut random_chance_checker = RandomChanceCheckerMock::default();
        random_chance_checker.expect_flip_coin().returns(true);
        random_chance_checker.expect_flip_coin().returns(false);
        random_chance_checker.expect_flip_coin().returns(false);
        random_chance_checker
            .expect_flip_coin_with_probability(any())
            .returns(true);
        random_chance_checker
            .expect_flip_coin_with_probability(any())
            .returns(false);

        let mut deriver = ChromosomalCrossover::new(box random_chance_checker);

        let actual_genome = deriver.derive_genome_from_parents((genome_one, genome_two));

        assert_eq!(expected_genome, actual_genome);
    }
}
