use super::UniqueId;

#[derive(Debug)]
pub struct Node<N>
where
    N: Sized,
{
    id: UniqueId,
    data: N,
}

impl<N> Node<N> {
    pub fn new(id: UniqueId, data: N) -> Self {
        Node { id: id, data: data }
    }

    pub fn id(&self) -> &UniqueId {
        &self.id
    }

    pub fn data(&self) -> &N {
        &self.data
    }
}
