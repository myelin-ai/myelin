pub(crate) mod color {
    pub(crate) const ORANGE: &str = "orange";
    pub(crate) const BLUE: &str = "blue";
    pub(crate) const GREEN: &str = "green";
    pub(crate) const BROWN: &str = "brown";
    pub(crate) const BLACK: &str = "black";
}

pub(crate) mod offset {
    use myelin_geometry::Point;

    pub(crate) const NAME_OFFSET: Point = Point { x: 0.0, y: -10.0 };
}

pub(crate) mod alignment {
    pub(crate) const CENTER: &str = "center";
}
