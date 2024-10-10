use {bevy::prelude::*, std::marker::PhantomData};

#[derive(Resource)]
pub struct AssetOwner<T, A: Asset>(Handle<A>, PhantomData<T>);

impl<T: Component, A: Asset> AssetOwner<T, A> {
    pub fn new(handle: Handle<A>) -> Self {
        Self(handle, PhantomData)
    }

	pub fn handle(&self) -> Handle<A> {
		self.0.clone_weak()
	}
}