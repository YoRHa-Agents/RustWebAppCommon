# Rust Web Common 研究输出与后续任务交接
> 更新时间: 2026-04-01

## 目标
本文件定义最终研究报告应如何组织，以及研究完成后如何把结论交接给设计、实现和 demo/storyboard 三类后续任务，避免每次重新解释原始 brief。

## 最终研究报告大纲
后续正式研究报告建议固定为以下结构：

1. **执行摘要**
   - 本次研究回答了哪些问题。
   - 哪些结论已经达到可决策状态。
   - 哪些问题仍需补样本。
2. **研究边界与证据门槛**
   - 当前纳入范围、排除范围。
   - `A/B/C` 证据等级和本次实际达到的覆盖度。
3. **样本总览**
   - 按主线 A/B/C 列出收集到的样本。
   - 标记 `adopt` / `adapt` / `reject` / `baseline_only`。
4. **对比矩阵结论**
   - 摘要矩阵中的关键差异。
   - 指出哪些差异真正影响 `common` 边界。
5. **`common` 边界建议**
   - 明确建议沉淀到 `RustWebAppCommon` 的能力。
   - 明确建议留在应用仓库的能力。
   - 明确建议通过适配层暴露的能力。
6. **Agent 文档与索引建议**
   - `README + index + 子主题入口` 的推荐组织方式。
   - 主 agent 与 subagent 的最小入口包。
7. **风格系统建议**
   - Nier: Automata 黑白灰方向的结构约束、主题 token 和 markdown 规范。
   - 哪些约束属于 docs 层，哪些属于页面/demo 层。
8. **Web/demo 蓝图**
   - demo 入口形式。
   - 页面分区与子页面跳转关系。
   - 本地启动与在线展示的对应关系。
9. **风险、限制与待验证项**
   - 风险清单。
   - 缺失样本。
   - 需要 `Enva` 二次验证的部分。
10. **后续执行路线**
   - 设计任务。
   - 实现任务。
   - demo/storyboard 任务。

## 推荐结论区模板
正式研究报告中的“建议”部分，必须按以下模板输出，而不是写成散文：

```md
## Recommendation Summary
- adopt_now:
  - ...
- adapt_before_adopt:
  - ...
- keep_in_app:
  - ...
- postpone:
  - ...

## Common Boundary
- common_core:
  - ...
- common_adapters:
  - ...
- app_owned:
  - ...

## Evidence Notes
- strongest_support:
  - ...
- weakest_area:
  - ...
- missing_samples:
  - ...
```

## 后续任务拆分
### 任务 1: 设计任务
- **输入**:
  - `01_research_charter.md`
  - `03_research_streams.md`
  - `04_comparison_matrix.md`
  - 正式研究报告
- **输出**:
  - `RustWebAppCommon` 架构设计文档
  - common/app 能力边界图
  - CLI、docs、demo、style 四个子系统接口定义
- **依赖**:
  - 研究报告完成并达到最低证据门槛
- **可并行对象**:
  - demo/storyboard 任务可以在核心边界锁定后并行启动
- **验证方式**:
  - 架构图能明确回答哪些能力属于 common、哪些属于 app
  - CLI、文档入口、demo 入口和风格系统都有明确接口边界
- **建议 agent**:
  - `generalPurpose` + 主 agent 直接收敛

### 任务 2: 实现任务
- **输入**:
  - 设计任务输出
  - `02_evidence_schema.md` 中归纳出的高优先级模式
  - `Enva` 二次验证结论
- **输出**:
  - 初始工程骨架
  - 统一 CLI 入口
  - 最小 docs/index 结构
  - 基础测试与验证命令
- **依赖**:
  - 设计任务完成
  - `Enva` 验证已确认不存在本地约束冲突
- **可并行对象**:
  - 无，应在架构锁定后串行推进
- **验证方式**:
  - 能通过统一命令启动项目
  - 能传入 `host` / `port`
  - 能访问主页面或子页面入口
  - 基础测试全部通过
- **建议 agent**:
  - 主 agent `direct` + `shell`

### 任务 3: Demo / Storyboard 任务
- **输入**:
  - 研究报告中的 Web/demo 建议
  - 设计任务中的页面入口与风格边界
- **输出**:
  - demo storyboard
  - 页面结构草图
  - Nier 风格 markdown / 页面主题约束说明
- **依赖**:
  - 至少拿到研究报告中的 `Web/demo 蓝图` 和 `风格系统建议`
- **可并行对象**:
  - 可与设计任务后半段并行
- **验证方式**:
  - storyboard 覆盖页面区块、跳转关系、展示目标和主题约束
  - 能说明本地启动与在线展示的映射方式
- **建议 agent**:
  - `generalPurpose`，必要时配合浏览器或 canvas 能力

## 执行顺序建议
1. 完成正式研究报告。
2. 在不读取 `Enva` 的前提下先输出设计草案和 demo/storyboard 草案。
3. 使用单独 subagent 对 `Enva` 做约束复核。
4. 基于设计稿与 `Enva` 复核结果进入实现。

## 未来建议的交接包
当研究结束时，建议交付一个最小交接包给下一个 agent：
- 一页执行摘要。
- 最新对比矩阵。
- 3 条最强支持结论。
- 3 条最大风险。
- 下一步应该启动的具体任务名称与依赖。
