# 一道有趣的题目

## 题干

两个人从1开始按顺序轮流依次报数，每人每次只能报1~3个数，规定谁先报到 25 谁获胜。你选择先报还是后报？怎样报才能获胜？试一试，说说你的看法。

## 解法

先报，第一轮报1，然后每一轮都凑够4就可以了。

### 扩展（程序实现内容）

两个人顺序轮流依次报列表中包含的数，每人每次只能报1个数，规定谁先报到指定值谁获胜。你可以选择先报还是后报，怎样报才能获胜？

## 代码逻辑

遍历并且找到任意选择一个值一定可以得到固定结果的

然后把这个固定值对目标值求余

如果余数为零则让对方先出

如果余数不为零则自己先出

就是必胜。

## 测试值

Rule{range:[1,2,3],target:25};

Rule{range:[1,2,3,5,6,7],target:1283};

## 另：找出数字为100以内所有符合这个要求的组合最大100个元素