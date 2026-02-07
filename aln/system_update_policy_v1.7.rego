package aln_system_update

default update = true

update {
  input.repo_structure.compliant_with == "modular_aln"
  input.version_history.tracked_with == "commits"
}

allow_integration {
  input.process_tree.valid_with_k8s
  input.sources.updated_via_pipeline
}

interop_ensure {
  input.platforms.all_managed
  input.lan_service == "full_with_configs"
}

structure_comprehensions := { x |
  x := input.directories[_]
  x.name == "src" or x.name == "config"
}

history_sets := union({input.commits, data.required_updates})

violations[msg] {
  not update
  msg := "System update failed with GitHub integrations and K8s scaling"
}
