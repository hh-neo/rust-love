# Rust 示例代码集

这个仓库包含了几个简单的Rust示例程序，用于展示Rust语言的基本特性和概念。

## 如何运行这些示例

首先确保你已经安装了Rust。如果没有安装，可以访问 [rustup.rs](https://rustup.rs/) 进行安装。

### 运行主程序

```bash
cargo run
```

### 运行示例程序

每个示例程序都位于 `examples` 目录下，可以使用以下命令运行特定的示例：

```bash
cargo run --example 示例名称
```

例如，运行基本类型示例：

```bash
cargo run --example basic_types
```

## 示例列表

1. **basic_types.rs** - 基本数据类型和变量演示
   - 整数、浮点数、布尔值、字符、字符串
   - 元组和数组

2. **control_flow.rs** - 控制流演示
   - 条件语句 (if/else if/else)
   - 循环 (loop、while、for)
   - 范围和步长

3. **functions_ownership.rs** - 函数定义和所有权系统
   - 基本函数、多返回值函数
   - 所有权转移、借用、可变借用
   - 切片

4. **structs_enums.rs** - 结构体和枚举
   - 基本结构体、元组结构体、单元结构体
   - 结构体方法实现
   - 枚举定义和方法
   - Option和Result枚举的使用

5. **error_handling_generics.rs** - 错误处理和泛型
   - 自定义错误类型
   - Result和错误处理模式
   - 泛型函数和结构体
   - ?运算符的使用

## 学习资源

如果你想深入学习Rust，可以参考以下资源：

- [Rust官方文档](https://doc.rust-lang.org/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust By Example](https://doc.rust-lang.org/rust-by-example/)