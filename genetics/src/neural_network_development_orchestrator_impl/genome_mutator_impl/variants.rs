pub enum MutationVariant {
    HoxMutationVariant(HoxMutationVariant),
    ClusterMutationVariant(ClusterMutationVariant),
}

pub enum HoxMutationVariant {
    CopySelf,
}

pub enum ClusterUnitMutationVariant {}

pub enum ClusterMutationVariant {
    ShiftWeight,
    Bridge,
}
