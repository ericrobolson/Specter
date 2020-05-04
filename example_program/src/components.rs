pub struct position { pub value: vec2, }
impl Component for position { type Storage = VecStorage<Self>;}

pub struct velocity { pub value: vec2, }
impl Component for velocity { type Storage = VecStorage<Self>;}

pub struct hitpoints { pub value: number,pub max: number, }
impl Component for hitpoints { type Storage = VecStorage<Self>;}

