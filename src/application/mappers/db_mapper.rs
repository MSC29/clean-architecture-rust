pub trait DbMapper<Entity, DbModel> {
    // Map an Entity to a DbModel
    fn to_db(entity: Entity) -> DbModel;

    // Map a DbModel to an Entity
    fn to_entity(model: DbModel) -> Entity;
}
