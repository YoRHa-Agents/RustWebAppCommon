import unittest

from tests.common_side_harness import (
    ROOT,
    assert_site_contract_parity,
    mock_release_api_server,
    run_checked,
    temporary_directory,
)


class EnvaMigrationValidationTests(unittest.TestCase):
    def test_release_contract_supports_target_override(self) -> None:
        result = run_checked(
            [
                "bash",
                "-lc",
                "source scripts/release-contract.sh && export RWC_TARGET_PLATFORM=macos-aarch64 && rwc_detect_asset_name",
            ]
        )
        self.assertEqual(result.stdout.strip(), "rustwebappcommon-macos-aarch64")

    def test_validate_release_script_exercises_install_hook_and_update_check(self) -> None:
        with temporary_directory("rwc-install-") as install_dir:
            result = run_checked(
                ["bash", "scripts/validate-release.sh"],
                env={"RWC_VALIDATE_INSTALL_DIR": str(install_dir)},
            )
            self.assertTrue((install_dir / "common").is_file())
            self.assertIn("Installed common to", result.stdout)

        self.assertIn("Release validation: ok", result.stdout)
        self.assertIn("Running post-install hook", result.stdout)
        self.assertIn("Migration ready: yes", result.stdout)

    def test_release_bundle_keeps_repo_and_release_site_contracts_in_sync(self) -> None:
        run_checked(["bash", "scripts/build-release.sh"])
        assert_site_contract_parity(ROOT / "site", ROOT / "release" / "site")

    def test_update_check_reports_remote_manifest_and_migration_readiness(self) -> None:
        payload = {
            "tag_name": "v9.9.9",
            "assets": [
                {
                    "name": "rustwebappcommon-linux-x86_64",
                    "browser_download_url": "https://example.test/rustwebappcommon-linux-x86_64",
                },
                {
                    "name": "SHA256SUMS",
                    "browser_download_url": "https://example.test/SHA256SUMS",
                },
                {
                    "name": "release-manifest.json",
                    "browser_download_url": "https://example.test/release-manifest.json",
                },
            ],
        }

        with mock_release_api_server(payload) as server_url:
            result = run_checked(
                ["bash", "scripts/update-check.sh"],
                env={
                    "RWC_RELEASE_REPO": "example/common",
                    "RWC_TARGET_PLATFORM": "linux-x86_64",
                    "RWC_UPDATE_API_BASE": server_url,
                    "RWC_UPDATE_REQUIRE_MANIFEST": "1",
                    "RWC_UPDATE_REQUIRE_CHECKSUMS": "1",
                    "RWC_CURRENT_TAG": "v0.1.0",
                },
            )

        self.assertIn("Latest release tag: v9.9.9", result.stdout)
        self.assertIn("Expected asset: rustwebappcommon-linux-x86_64", result.stdout)
        self.assertIn("Checksums present: yes", result.stdout)
        self.assertIn("Manifest present: yes", result.stdout)
        self.assertIn("Migration ready: yes", result.stdout)
        self.assertIn("Update available: yes", result.stdout)


if __name__ == "__main__":
    unittest.main()
