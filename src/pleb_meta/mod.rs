mod bond;
mod grammar;
mod mode;
mod operator;
mod route;
mod routing_pass;
mod sequence;

pub use bond::PlebMetaBond;
pub use grammar::{PlebMetaGrammar, normal_response};
pub use mode::Mode;
pub use operator::Operator;
pub use route::{
    ExteriorShape, ExteriorState, InteriorState, PlebMetaInput, PlebMetaRouting, StrandState,
};
pub use routing_pass::RoutingPass;
pub use sequence::Sequence;

#[cfg(test)]
mod tests {
    use super::{
        ExteriorShape, Mode, Operator, PlebMetaGrammar, PlebMetaInput, Sequence, normal_response,
    };

    #[test]
    fn sequence_complements_are_symmetric() {
        assert_eq!(Sequence::Pleb.complement(), Sequence::Blep);
        assert_eq!(Sequence::Blep.complement(), Sequence::Pleb);
        assert_eq!(Sequence::Meta.complement(), Sequence::Atem);
        assert_eq!(Sequence::Atem.complement(), Sequence::Meta);
    }

    #[test]
    fn mode_complements_are_symmetric() {
        assert_eq!(Mode::Pathos.complement(), Mode::Bathos);
        assert_eq!(Mode::Bathos.complement(), Mode::Pathos);
        assert_eq!(Mode::Logos.complement(), Mode::Ethos);
        assert_eq!(Mode::Ethos.complement(), Mode::Logos);
    }

    #[test]
    fn operators_handle_only_their_assigned_sequences() {
        assert!(Operator::Clouseau.handles(Sequence::Pleb));
        assert!(!Operator::Clouseau.handles(Sequence::Meta));
        assert!(!Operator::Clouseau.handles(Sequence::Blep));
        assert!(!Operator::Clouseau.handles(Sequence::Atem));

        assert!(Operator::Hal.handles(Sequence::Meta));
        assert!(!Operator::Hal.handles(Sequence::Pleb));
        assert!(!Operator::Hal.handles(Sequence::Blep));
        assert!(!Operator::Hal.handles(Sequence::Atem));

        assert!(Operator::Cleopatra.handles(Sequence::Blep));
        assert!(Operator::Cleopatra.handles(Sequence::Atem));
        assert!(!Operator::Cleopatra.handles(Sequence::Pleb));
        assert!(!Operator::Cleopatra.handles(Sequence::Meta));
    }

    #[test]
    fn straight_routing_witnesses_all_four_strands() {
        let routing = PlebMetaGrammar::route(PlebMetaInput {
            exterior_shape: ExteriorShape::Straight,
            pleb_mode: Mode::Pathos,
            meta_mode: Mode::Logos,
        });

        assert_eq!(routing.pleb().sequence(), Sequence::Pleb);
        assert_eq!(routing.pleb().mode(), Mode::Pathos);
        assert_eq!(routing.blep().sequence(), Sequence::Blep);
        assert_eq!(routing.blep().mode(), Mode::Bathos);
        assert_eq!(routing.meta().sequence(), Sequence::Meta);
        assert_eq!(routing.meta().mode(), Mode::Logos);
        assert_eq!(routing.atem().sequence(), Sequence::Atem);
        assert_eq!(routing.atem().mode(), Mode::Ethos);

        assert_eq!(routing.exterior().foreground_sequence(), Sequence::Pleb);
        assert_eq!(routing.exterior().operator(), Operator::Clouseau);
        assert_eq!(routing.interior().sequence(), Sequence::Blep);
        assert_eq!(routing.interior().operator(), Operator::Cleopatra);
        assert_eq!(routing.bond().pleb_mode(), Mode::Pathos);
        assert_eq!(routing.bond().meta_mode(), Mode::Logos);
    }

    #[test]
    fn curved_routing_witnesses_all_four_strands() {
        let routing = PlebMetaGrammar::route(PlebMetaInput {
            exterior_shape: ExteriorShape::Curved,
            pleb_mode: Mode::Pathos,
            meta_mode: Mode::Logos,
        });

        assert_eq!(routing.pleb().sequence(), Sequence::Pleb);
        assert_eq!(routing.pleb().mode(), Mode::Pathos);
        assert_eq!(routing.blep().sequence(), Sequence::Blep);
        assert_eq!(routing.blep().mode(), Mode::Bathos);
        assert_eq!(routing.meta().sequence(), Sequence::Meta);
        assert_eq!(routing.meta().mode(), Mode::Logos);
        assert_eq!(routing.atem().sequence(), Sequence::Atem);
        assert_eq!(routing.atem().mode(), Mode::Ethos);

        assert_eq!(routing.exterior().foreground_sequence(), Sequence::Meta);
        assert_eq!(routing.exterior().operator(), Operator::Hal);
        assert_eq!(routing.interior().sequence(), Sequence::Atem);
        assert_eq!(routing.interior().operator(), Operator::Cleopatra);
        assert_eq!(routing.bond().pleb_mode(), Mode::Pathos);
        assert_eq!(routing.bond().meta_mode(), Mode::Logos);
    }

    #[test]
    fn normal_response_returns_the_complementary_mode() {
        assert_eq!(normal_response(Mode::Pathos), Mode::Bathos);
        assert_eq!(normal_response(Mode::Bathos), Mode::Pathos);
        assert_eq!(normal_response(Mode::Logos), Mode::Ethos);
        assert_eq!(normal_response(Mode::Ethos), Mode::Logos);
    }

    #[test]
    fn complete_routing_witness_always_contains_every_required_part() {
        let routing = PlebMetaGrammar::route(PlebMetaInput::default());

        assert_eq!(routing.pleb().sequence(), Sequence::Pleb);
        assert_eq!(routing.blep().sequence(), Sequence::Blep);
        assert_eq!(routing.meta().sequence(), Sequence::Meta);
        assert_eq!(routing.atem().sequence(), Sequence::Atem);
        assert_eq!(routing.blep().mode(), routing.pleb().mode().complement());
        assert_eq!(routing.atem().mode(), routing.meta().mode().complement());
        assert_eq!(
            routing.interior().sequence(),
            routing.exterior().foreground_sequence().complement()
        );
    }
}
