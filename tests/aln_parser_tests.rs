use aln_system_update_orchestrator::aln::AlnUpdatePlan;

#[test]
fn parses_update_system_file() {
    let plan = AlnUpdatePlan::from_file("aln/system_update_integration_v1.7.aln")
        .expect("failed to parse ALN file");
    assert_eq!(plan.version, "1.0.1.7");
    assert!(!plan.components.renderers.is_empty());
}
