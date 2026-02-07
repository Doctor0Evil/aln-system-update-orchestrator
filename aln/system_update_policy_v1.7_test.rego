package aln_system_update_test

import data.aln_system_update

test_update_allows_valid_input {
  aln_system_update.update with input as {
    "repo_structure": { "compliant_with": "modular_aln" },
    "version_history": { "tracked_with": "commits" },
    "process_tree": { "valid_with_k8s": true },
    "sources": { "updated_via_pipeline": true },
    "platforms": { "all_managed": true },
    "lan_service": "full_with_configs",
    "directories": [ { "name": "src" }, { "name": "config" } ],
    "commits": []
  }
}
