import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


class StarterRepoTests(unittest.TestCase):
    def test_workspace_files_exist(self) -> None:
        required_paths = [
            ROOT / "Cargo.toml",
            ROOT / "README.md",
            ROOT / "AGENTS.md",
            ROOT / "common" / "core" / "Cargo.toml",
            ROOT / "common" / "core" / "src" / "lib.rs",
            ROOT / "common" / "adapters" / "Cargo.toml",
            ROOT / "common" / "adapters" / "src" / "lib.rs",
            ROOT / "common" / "cli" / "Cargo.toml",
            ROOT / "common" / "cli" / "src" / "lib.rs",
            ROOT / "common" / "cli" / "src" / "main.rs",
            ROOT / "docs" / "index.md",
            ROOT / "demo" / "storyboard.md",
            ROOT / "examples" / "README.md",
            ROOT / "apps" / "README.md",
            ROOT / "site" / "index.html",
            ROOT / "site" / "demo.html",
            ROOT / "site" / "404.html",
            ROOT / "site" / "css" / "style.css",
            ROOT / "site" / "js" / "main.js",
            ROOT / "site" / "assets" / "runtime-contract.json",
            ROOT / "site" / "docs" / "index.html",
            ROOT / "scripts" / "build-release.sh",
            ROOT / "scripts" / "release-contract.sh",
            ROOT / "scripts" / "install.sh",
            ROOT / "scripts" / "update-check.sh",
            ROOT / "scripts" / "validate-release.sh",
            ROOT / ".github" / "workflows" / "deploy-pages.yml",
            ROOT / ".github" / "workflows" / "release-artifacts.yml",
            ROOT / "tests" / "common_side_harness.py",
            ROOT / "tests" / "test_enva_migration_validation.py",
            ROOT / "doc_auto" / "gate_decision_table.md",
            ROOT / "doc_auto" / "remaining_implementation_delta.md",
            ROOT / "doc_auto" / "enva_compatibility_matrix.md",
            ROOT / "doc_auto" / "implementation_handoff.md",
            ROOT / "doc_auto" / "enva_migration_validation_recommendations.md",
            ROOT / "doc_auto" / "starter_repo_sync.md",
        ]

        for path in required_paths:
            with self.subTest(path=path):
                self.assertTrue(path.exists(), f"missing {path}")

    def test_workspace_members_are_declared(self) -> None:
        cargo_toml = (ROOT / "Cargo.toml").read_text(encoding="utf-8")
        self.assertIn("common/core", cargo_toml)
        self.assertIn("common/adapters", cargo_toml)
        self.assertIn("common/cli", cargo_toml)

    def test_core_contracts_are_present(self) -> None:
        content = (ROOT / "common" / "core" / "src" / "lib.rs").read_text(encoding="utf-8")
        for symbol in [
            "WorkspaceIdentity",
            "SurfaceKind",
            "DevLaunchRequest",
            "RouteDescriptor",
            "DocsNode",
            "ThemeTokenSet",
            "ReleaseDescriptor",
        ]:
            with self.subTest(symbol=symbol):
                self.assertIn(symbol, content)

    def test_docs_demo_use_shared_terms(self) -> None:
        readme = (ROOT / "README.md").read_text(encoding="utf-8")
        docs_index = (ROOT / "docs" / "index.md").read_text(encoding="utf-8")
        storyboard = (ROOT / "demo" / "storyboard.md").read_text(encoding="utf-8")
        route_manifest = (ROOT / "site" / "assets" / "route-manifest.json").read_text(
            encoding="utf-8"
        )
        runtime_contract = (ROOT / "site" / "assets" / "runtime-contract.json").read_text(
            encoding="utf-8"
        )

        self.assertIn("common_core", readme)
        self.assertIn("browser-backed desktop preview", readme)
        self.assertIn("runtime-decision", storyboard)
        self.assertIn("Choose Your Path", docs_index)
        self.assertIn("docs/architecture/overview.md", docs_index)
        self.assertIn("RuntimeMap", route_manifest)
        self.assertIn("browser_backed_preview", runtime_contract)

    def test_doc_auto_records_sync_status(self) -> None:
        content = (ROOT / "doc_auto" / "starter_repo_sync.md").read_text(encoding="utf-8")
        gate_content = (ROOT / "doc_auto" / "gate_decision_table.md").read_text(
            encoding="utf-8"
        )
        delta = (ROOT / "doc_auto" / "remaining_implementation_delta.md").read_text(
            encoding="utf-8"
        )
        compatibility = (ROOT / "doc_auto" / "enva_compatibility_matrix.md").read_text(
            encoding="utf-8"
        )
        handoff = (ROOT / "doc_auto" / "implementation_handoff.md").read_text(
            encoding="utf-8"
        )
        recommendations = (
            ROOT / "doc_auto" / "enva_migration_validation_recommendations.md"
        ).read_text(encoding="utf-8")
        index_content = (ROOT / "doc_auto" / "README.md").read_text(encoding="utf-8")
        self.assertIn("16_architecture_design_doc.md", content)
        self.assertIn("Last Updated", content)
        self.assertIn("Gate 2: `Enva` 兼容性复核", gate_content)
        self.assertIn("Phase 1 -> (Phase 2 并行 Phase 3) -> Phase 4 -> Phase 5", delta)
        self.assertIn("same_capability", compatibility)
        self.assertIn("browser-backed preview", compatibility)
        self.assertIn("Enva", handoff)
        self.assertIn("RWC_POST_INSTALL_HOOK", recommendations)
        self.assertIn("enva_migration_validation_recommendations.md", index_content)

    def test_release_and_pages_workflows_exist(self) -> None:
        pages = (ROOT / ".github" / "workflows" / "deploy-pages.yml").read_text(
            encoding="utf-8"
        )
        release = (ROOT / ".github" / "workflows" / "release-artifacts.yml").read_text(
            encoding="utf-8"
        )
        self.assertIn("upload-pages-artifact", pages)
        self.assertIn("python -m unittest tests.test_research_pack tests.test_starter_repo", pages)
        self.assertIn("tests.test_enva_migration_validation", pages)
        self.assertIn("cargo run -p common_cli -- demo", pages)
        self.assertIn("tests.test_enva_migration_validation", release)
        self.assertIn("scripts/validate-release.sh", release)
        self.assertIn("softprops/action-gh-release", release)

    def test_gitignore_preserves_repo_sources_and_ignores_build_outputs(self) -> None:
        content = (ROOT / ".gitignore").read_text(encoding="utf-8")
        self.assertIn("/target/", content)
        self.assertIn("/release/", content)
        self.assertIn(".tmp-install*/", content)
        self.assertNotIn("/site/", content)
        self.assertNotIn("Cargo.lock", content)
        self.assertNotIn(".local/", content)
        self.assertNotIn("doc_auto/", content)

    def test_release_contract_scripts_share_asset_logic(self) -> None:
        contract = (ROOT / "scripts" / "release-contract.sh").read_text(encoding="utf-8")
        build_script = (ROOT / "scripts" / "build-release.sh").read_text(encoding="utf-8")
        install_script = (ROOT / "scripts" / "install.sh").read_text(encoding="utf-8")
        update_script = (ROOT / "scripts" / "update-check.sh").read_text(encoding="utf-8")
        validate_script = (ROOT / "scripts" / "validate-release.sh").read_text(
            encoding="utf-8"
        )
        release_guide = (ROOT / "docs" / "guides" / "release.md").read_text(
            encoding="utf-8"
        )

        self.assertIn("rwc_detect_asset_name", contract)
        self.assertIn("rwc_validate_local_release_dir", contract)
        self.assertIn("release-manifest.json", build_script)
        self.assertIn("rwc_validate_local_release_dir", build_script)
        self.assertIn("rwc_verify_checksum", install_script)
        self.assertIn("RWC_POST_INSTALL_HOOK", install_script)
        self.assertIn("LOCAL_RELEASE_DIR", update_script)
        self.assertIn("Migration ready:", update_script)
        self.assertIn("compare_site_contracts", validate_script)
        self.assertIn("scripts/validate-release.sh", release_guide)
        self.assertIn("RWC_POST_INSTALL_HOOK", release_guide)


if __name__ == "__main__":
    unittest.main()
