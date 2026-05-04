---
name: hsy-comment-zh
description: "把当前会话后续新增或改写的代码注释切回中文。触发词：hsy-comment-zh, 中文注释, 注释中文."
argument-hint: "可选：补充本次注释切换只作用于哪些文件或模块"
agent: agent
---

按 [注释规范](../../docs/TauriCodeCommentStandard.md) 把当前会话后续新增或改写的代码注释切回简体中文。

执行要求：

1. 只影响从现在开始新写或改写的代码注释、Rustdoc、JSDoc/TSDoc，不要擅自回写未触及的旧注释。
2. 当前用户消息如果又明确要求英文注释，则以当前用户消息为准。
3. API 名、协议名、错误码、crate/package 名和第三方术语保持原文，不要强行翻译标识符。
4. 如果当前任务还没有开始写注释，后续默认按中文注释执行；如果已经在做注释相关改动，则从下一处新改动开始应用中文模式。
5. 之后只有在用户再次明确要求英文，或显式调用 [hsy-comment-en](hsy-comment-en.prompt.md) 时，才切换离开中文模式。