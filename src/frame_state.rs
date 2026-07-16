#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BeingId {
    Hueman,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FrameId {
    Hueman,
    Gremlin,
    Goblin,
    Ghoul,
    Troll,
    Ork,
    Ogre,
    Troglodyte,
    Pixy,
    Sprite,
    Faerie,
    Nymph,
    Siren,
    Muse,
    Werewolf,
    Gargoyle,
    Merman,
    Chimera,
    Manticore,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FlowId {
    TinkerGrip,
    Stonefold,
    PressureRelocation,
    PackRelay,
    Moonrush,
    MeteorDrop,
    RiptideSwim,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GlowId {
    Confusion,
    Projection,
    Recognition,
    SpriteCall,
    FaerieVeil,
    MuseChorus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrentPrism {
    body: u16,
    spirit: u16,
    mind: u16,
    soul_interior: u16,
    soul_exterior: u16,
}

impl CurrentPrism {
    pub const fn new(
        body: u16,
        spirit: u16,
        mind: u16,
        soul_interior: u16,
        soul_exterior: u16,
    ) -> Self {
        Self {
            body,
            spirit,
            mind,
            soul_interior,
            soul_exterior,
        }
    }

    pub const fn origin() -> Self {
        Self::new(1, 1, 1, 1, 1)
    }

    pub const fn body(&self) -> u16 {
        self.body
    }

    pub const fn spirit(&self) -> u16 {
        self.spirit
    }

    pub const fn mind(&self) -> u16 {
        self.mind
    }

    pub const fn soul_interior(&self) -> u16 {
        self.soul_interior
    }

    pub const fn soul_exterior(&self) -> u16 {
        self.soul_exterior
    }
}

impl Default for CurrentPrism {
    fn default() -> Self {
        Self::origin()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FrameState {
    frame: FrameId,
    prism: CurrentPrism,
    flow_learnset: Vec<FlowId>,
    glow_learnset: Vec<GlowId>,
}

impl FrameState {
    pub fn new(
        frame: FrameId,
        prism: CurrentPrism,
        flow_learnset: Vec<FlowId>,
        glow_learnset: Vec<GlowId>,
    ) -> Self {
        Self {
            frame,
            prism,
            flow_learnset,
            glow_learnset,
        }
    }

    pub fn origin() -> Self {
        Self::new(
            FrameId::Hueman,
            CurrentPrism::origin(),
            Vec::new(),
            Vec::new(),
        )
    }

    pub const fn frame(&self) -> FrameId {
        self.frame
    }

    pub const fn being(&self) -> BeingId {
        BeingId::Hueman
    }

    pub const fn prism(&self) -> &CurrentPrism {
        &self.prism
    }

    pub fn flow_learnset(&self) -> &[FlowId] {
        &self.flow_learnset
    }

    pub fn glow_learnset(&self) -> &[GlowId] {
        &self.glow_learnset
    }

    pub(crate) fn set_frame(&mut self, frame: FrameId) {
        self.frame = frame;
    }

    pub(crate) fn set_prism(&mut self, prism: CurrentPrism) {
        self.prism = prism;
    }

    pub(crate) fn learn_flow(&mut self, flow_id: FlowId) {
        if !self.flow_learnset.contains(&flow_id) {
            self.flow_learnset.push(flow_id);
        }
    }

    pub(crate) fn learn_glow(&mut self, glow_id: GlowId) {
        if !self.glow_learnset.contains(&glow_id) {
            self.glow_learnset.push(glow_id);
        }
    }
}

impl Default for FrameState {
    fn default() -> Self {
        Self::origin()
    }
}

#[cfg(test)]
mod tests {
    use super::{BeingId, CurrentPrism, FrameId, FrameState};

    #[test]
    fn origin_frame_state_matches_the_first_fixture() {
        let state = FrameState::origin();

        assert_eq!(state.being(), BeingId::Hueman);
        assert_eq!(state.frame(), FrameId::Hueman);
        assert_eq!(state.prism(), &CurrentPrism::origin());
        assert!(state.flow_learnset().is_empty());
        assert!(state.glow_learnset().is_empty());
    }

    #[test]
    fn transformed_frame_states_preserve_the_hueman_being_identity() {
        let gremlin = FrameState::new(
            FrameId::Gremlin,
            CurrentPrism::new(3, 1, 1, 1, 1),
            Vec::new(),
            Vec::new(),
        );
        let pixy = FrameState::new(
            FrameId::Pixy,
            CurrentPrism::new(1, 1, 3, 1, 1),
            Vec::new(),
            Vec::new(),
        );

        assert_eq!(gremlin.being(), BeingId::Hueman);
        assert_eq!(pixy.being(), BeingId::Hueman);
    }
}
