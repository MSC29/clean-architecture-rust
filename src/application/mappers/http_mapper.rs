pub trait HttpMapper<Entity, HttpObj> {
    // Map an Entity to an HttpObj
    fn to_http(entity: Entity) -> HttpObj;

    // Map an HttpObj to an Entity
    fn to_entity(http_obj: HttpObj) -> Entity;
}
