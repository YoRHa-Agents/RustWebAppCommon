import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
PACK_DIR = ROOT / ".local" / "research_pack"

REQUIRED_FILES = {
    "README.md": ["## 文件清单", "## 建议阅读顺序"],
    "01_research_charter.md": ["## 关键决策问题", "## 评价维度", "## 硬约束与延后输入"],
    "02_evidence_schema.md": ["## 必填字段", "## 单样本记录模板", "## 样本淘汰规则"],
    "03_research_streams.md": ["## 总览", "## 主线 A: Common 工程与运行时", "## 并行与依赖"],
    "04_comparison_matrix.md": ["## 基线矩阵", "## 模式归纳模板", "## 最低证据门槛"],
    "05_execution_handoffs.md": ["## 最终研究报告大纲", "## 后续任务拆分", "## 执行顺序建议"],
    "06_threshold_gap_register.md": ["## 当前状态", "## 主线 A 剩余缺口", "## 下一步产出优先级"],
    "07_docs_index_and_style_recommendation.md": ["## 推荐入口结构", "## Nier 风格方向", "## 推荐结论"],
    "08_common_architecture_proposal.md": ["## 推荐结论", "## 分层建议", "## 能力归属表"],
    "09_demo_storyboard.md": ["## 页面地图", "## 页面清单", "## 本地运行与在线展示映射"],
    "10_final_research_report.md": ["## 1. 执行摘要", "## 5. Recommendation Summary", "## 12. 后续执行路线"],
    "11_follow_on_tasks.md": ["## 任务 A: 统一架构设计稿", "## Blocker 判断", "## 推荐执行顺序"],
    "12_architecture_scope_and_decisions.md": ["## 本轮必须锁定的内容", "## 决策状态表", "## 设计稿应输出的产物"],
    "13_common_core_contracts.md": ["## 状态边界模型", "## 核心契约清单", "## Invariants"],
    "14_adapter_boundary_matrix.md": ["## adapter 总览", "## 运行面矩阵", "## adapter 选择规则"],
    "15_docs_demo_information_architecture.md": ["## 顶层目录建议", "## 文档导航图", "## 与 `common_core` 的一致性要求"],
    "16_architecture_design_doc.md": ["## 2. 架构总览", "## 5. Adapter Boundaries 摘要", "## 14. 设计结论"],
    "17_pre_implementation_validation_checklist.md": ["## Gate 1: starter repo 验证", "## Gate 2: `Enva` 兼容性复核", "## 放行规则"],
}

REQUIRED_SAMPLE_FILES = [
    "README.md",
    "local_openai_agents_python.md",
    "local_agentic_context_engine.md",
    "local_contextos.md",
    "official_dioxus.md",
    "official_tauri_v2.md",
    "official_trunk.md",
    "official_github_pages.md",
]


class ResearchPackTests(unittest.TestCase):
    def test_required_files_exist(self) -> None:
        for filename in REQUIRED_FILES:
            with self.subTest(filename=filename):
                self.assertTrue((PACK_DIR / filename).is_file(), f"missing {filename}")

    def test_required_headings_exist(self) -> None:
        for filename, headings in REQUIRED_FILES.items():
            content = (PACK_DIR / filename).read_text(encoding="utf-8")
            for heading in headings:
                with self.subTest(filename=filename, heading=heading):
                    self.assertIn(heading, content)

    def test_readme_indexes_all_artifacts(self) -> None:
        content = (PACK_DIR / "README.md").read_text(encoding="utf-8")
        for filename in REQUIRED_FILES:
            if filename != "README.md":
                with self.subTest(filename=filename):
                    self.assertIn(filename, content)

    def test_matrix_contains_local_baselines_and_thresholds(self) -> None:
        content = (PACK_DIR / "04_comparison_matrix.md").read_text(encoding="utf-8")
        self.assertIn("openai-agents-python", content)
        self.assertIn("agentic-context-engine", content)
        self.assertIn("ContextOS", content)
        self.assertIn("主线 A 至少有 2 个 `A` 级样本", content)
        self.assertIn("至少有 1 个样本被明确标记为 `reject` 或 `adapt`", content)

    def test_handoffs_cover_all_next_stage_tasks(self) -> None:
        content = (PACK_DIR / "05_execution_handoffs.md").read_text(encoding="utf-8")
        self.assertIn("### 任务 1: 设计任务", content)
        self.assertIn("### 任务 2: 实现任务", content)
        self.assertIn("### 任务 3: Demo / Storyboard 任务", content)

    def test_sample_dossiers_exist(self) -> None:
        sample_dir = PACK_DIR / "samples"
        self.assertTrue(sample_dir.is_dir(), "missing samples directory")
        for filename in REQUIRED_SAMPLE_FILES:
            with self.subTest(filename=filename):
                self.assertTrue((sample_dir / filename).is_file(), f"missing sample {filename}")

    def test_final_report_and_tasks_link_strategy(self) -> None:
        report = (PACK_DIR / "10_final_research_report.md").read_text(encoding="utf-8")
        tasks = (PACK_DIR / "11_follow_on_tasks.md").read_text(encoding="utf-8")
        self.assertIn("common_core", report)
        self.assertIn("common_adapters", report)
        self.assertIn("app_owned", report)
        self.assertIn("任务 C: `Enva` 约束复核", tasks)
        self.assertIn("starter repo 验证", tasks)

    def test_architecture_design_package_is_indexed(self) -> None:
        readme = (PACK_DIR / "README.md").read_text(encoding="utf-8")
        self.assertIn("12_architecture_scope_and_decisions.md", readme)
        self.assertIn("16_architecture_design_doc.md", readme)
        self.assertIn("17_pre_implementation_validation_checklist.md", readme)

    def test_architecture_design_doc_links_core_and_adapters(self) -> None:
        content = (PACK_DIR / "16_architecture_design_doc.md").read_text(encoding="utf-8")
        self.assertIn("common_core", content)
        self.assertIn("common_adapters", content)
        self.assertIn("app_owned", content)
        self.assertIn("13_common_core_contracts.md", content)
        self.assertIn("14_adapter_boundary_matrix.md", content)


if __name__ == "__main__":
    unittest.main()
