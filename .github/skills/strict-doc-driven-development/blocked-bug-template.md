# Blocked Bug 记录模板

当同一个阻塞修了 5 次仍失败，必须停下来等用户决断时，使用这个模板。

## Bug Summary

- task or atomic-task id:
- component or module:
- short description:
- current status: blocked after 5 attempts

## 控制性文档

1. docs/...
2. docs/...

## 受影响文件

1. path/to/file
2. path/to/file

## 最新错误

- command or check:
- error text:
- last observed time:

## 尝试记录

1. attempt 1 - what changed, what failed
2. attempt 2 - what changed, what failed
3. attempt 3 - what changed, what failed
4. attempt 4 - what changed, what failed
5. attempt 5 - what changed, what failed

## 当前状态

- last good commit:
- current HEAD:
- uncommitted changes:
- validation state:

## 建议的下一步决策

1. 等待用户决断
2. 如有必要，回退到哪个目标
3. 最安全的下一步诊断动作