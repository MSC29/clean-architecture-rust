pub trait ApiMapper<Entity, Presenter, Payload> {
    // Map an Entity to a Presenter
    fn to_api(entity: Entity) -> Presenter;

    // Map a Payload to an Entity
    fn to_entity(payload: Payload) -> Entity;
}
