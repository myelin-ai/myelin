pub(crate) mod color {
    pub(crate) const ORGANISM: &str = "orange";
    pub(crate) const WATER: &str = "blue";
    pub(crate) const PLANT: &str = "green";
    pub(crate) const TERRAIN: &str = "brown";
    pub(crate) const LABEL: &str = "black";
}

pub(crate) mod offset {
    use myelin_engine::geometry::Point;

    pub(crate) const NAME_OFFSET: Point = Point { x: 0.0, y: -10.0 };
}

pub(crate) mod alignment {
    pub(crate) const CENTER: &str = "center";
}
