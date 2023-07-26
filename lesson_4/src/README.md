
枚举和trait object的主要区别在于:
1. 枚举方式可以直接存放值，trait object需要放Box，或者要使用用引用 &dyn
2. 枚举方式需要匹配处理，trait object可以直接调用
3. trait object有额外的动态分发开销（但开销不会差距太大）。
总结：需要根据实际场景选择合适的方式。
