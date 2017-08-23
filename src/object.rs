use coord::Coord;
use game::Game;

/// A data structure for things like doors and traps which
/// can be interacted with. For more about the differences
/// between objects and actors, see actor.rs.
pub struct Object {
    class: Class,
    material: Material,
    active: bool,

    /// The coordinate of the object.
    xy: Option<Coord>,
}

impl Object {
    pub fn new(game: &Game, name: &str, active: bool) -> Object {
        let object_database = game.database.get("Object").get(name);

        Object {
            class: Class::string_to_class(object_database.get("class").get_str()),
            material: Material::string_to_material(object_database.get("material").get_str()),
            active: active,

            xy: None,
        }
    }
}

/// Enum listing possible object classes.
pub enum Class {
    Door,
    Trap,
}

impl Class {
    /// Converts `string` to a Class enum.
    ///
    /// # Panics
    /// Panics if `string` does not correspond to a Class value.
    pub fn string_to_class(string: &str) -> Class {
        match string {
            "Door" => Class::Door,
            "Trap" => Class::Trap,
            _ => {
                panic!(
                    "Class::string_to_class failed: invalid input \"{}\"",
                    string
                )
            }
        }
    }
}

/// Enum listing possible object materials.
pub enum Material {
    Wood,
    Iron,
}

impl Material {
    /// Converts `string` to a `Material` enum.
    ///
    /// # Panics
    /// Panics if `string` does not correspond to a Material value.
    pub fn string_to_material(string: &str) -> Material {
        match string {
            "Wood" => Material::Wood,
            "Iron" => Material::Iron,
            _ => {
                panic!(
                    "Material::string_to_material failed: invalid input \"{}\"",
                    string
                )
            }
        }
    }
}
