pub trait ApiMapper<Entity, Presenter, Payload> {
    // Map an Entity to an HttpObj
    fn to_api(entity: Entity) -> Presenter;

    //Map an HttpObj to an Entity
    fn to_entity(payload: Payload) -> Entity;
}
