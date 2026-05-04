---
name: hsy-comment-en
description: "把当前会话后续新增或改写的代码注释切到英文。触发词：hsy-comment-en, 英文注释, 注释英文."
argument-hint: "可选：补充本次英文注释只作用于哪些文件或模块"
agent: agent
---

按 [注释规范](../../docs/TauriCodeCommentStandard.md) 把当前会话后续新增或改写的代码注释切到英文。

执行要求：

1. 只影响从现在开始新写或改写的代码注释、Rustdoc、JSDoc/TSDoc，不要擅自回写未触及的旧注释。
2. 这是对仓库“默认中文注释”规则的显式临时覆盖；如果用户之后要求切回中文，或显式调用 [hsy-comment-zh](hsy-comment-zh.prompt.md)，就恢复中文模式。
3. 保留 API 名、协议名、错误码、crate/package 名和第三方术语原文，不要为了追求全英文而重命名标识符。
4. 如果当前任务还没有开始写注释，后续默认按英文注释执行；如果已经在做注释相关改动，则从下一处新改动开始应用英文模式。
5. 不要因为切到英文注释，就自动修改仓库 hook、prompt 或 UI 提醒的语言模式；这里只切换代码注释语言。