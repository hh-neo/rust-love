// Rust 特质(Trait)示例

// 1. 基本特质定义
// 特质类似于其他语言中的接口，可以定义方法签名
pub trait Animal {
    // 必须实现的方法
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
    
    // 带有默认实现的方法
    fn make_sound(&self) {
        println!("{} says {}", self.name(), self.sound());
    }
}

// 2. 实现特质
struct Dog {
    name: String,
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn sound(&self) -> &str {
        "Woof!"
    }
}

struct Cat {
    name: String,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn sound(&self) -> &str {
        "Meow!"
    }
    
    // 覆盖默认实现
    fn make_sound(&self) {
        println!("{} purrs and then says {}", self.name(), self.sound());
    }
}

// 3. 特质继承
pub trait Pet: Animal {
    fn is_friendly(&self) -> bool;
}

impl Pet for Dog {
    fn is_friendly(&self) -> bool {
        true
    }
}

impl Pet for Cat {
    fn is_friendly(&self) -> bool {
        false // 猫比较高冷
    }
}

// 4. 使用特质作为参数 (特质约束)
// 接受任何实现了Animal特质的类型
fn animal_sound<T: Animal>(animal: &T) {
    animal.make_sound();
}

// 5. 多个特质约束
fn pet_info<T: Pet>(pet: &T) {
    println!("Pet '{}' is friendly: {}", pet.name(), pet.is_friendly());
    pet.make_sound();
}

// 6. where子句形式的特质约束
fn animal_activities<T>(animal: &T) where T: Animal {
    println!("{} is active!", animal.name());
    animal.make_sound();
}

// 7. 返回实现了特质的类型
fn create_animal(kind: &str, name: &str) -> Box<dyn Animal> {
    match kind {
        "dog" => Box::new(Dog { name: name.to_string() }),
        "cat" => Box::new(Cat { name: name.to_string() }),
        _ => panic!("Unknown animal kind")
    }
}

// 8. 常见的标准库特质实现示例
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

// 实现PartialEq特质（用于比较）
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}

// 实现Display特质（用于格式化输出）
use std::fmt;

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
    }
}

// 9. 操作符重载 (通过实现相应特质)
#[derive(Clone, Copy)]
struct Vector2D {
    x: i32,
    y: i32,
}

impl std::ops::Add for Vector2D {
    type Output = Vector2D;
    
    fn add(self, other: Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl fmt::Display for Vector2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    // 测试基本特质实现
    let rover = Dog { name: "Rover".to_string() };
    let whiskers = Cat { name: "Whiskers".to_string() };
    
    println!("=== 基本特质实现 ===");
    rover.make_sound();
    whiskers.make_sound();
    
    // 测试特质作为参数
    println!("\n=== 特质作为参数 ===");
    animal_sound(&rover);
    animal_sound(&whiskers);
    
    // 测试多个特质约束
    println!("\n=== 多个特质约束 ===");
    pet_info(&rover);
    pet_info(&whiskers);
    
    // 测试where子句
    println!("\n=== where子句形式 ===");
    animal_activities(&rover);
    
    // 测试返回实现了特质的类型（动态分派）
    println!("\n=== 返回特质类型（动态分派）===");
    let animal1 = create_animal("dog", "Buddy");
    let animal2 = create_animal("cat", "Felix");
    animal1.make_sound();
    animal2.make_sound();
    
    // 测试标准库特质实现
    println!("\n=== 标准库特质实现 ===");
    let person1 = Person { name: "Alice".to_string(), age: 30 };
    let person2 = Person { name: "Alice".to_string(), age: 30 };
    let person3 = Person { name: "Bob".to_string(), age: 25 };
    
    println!("person1: {}", person1);
    println!("person1 == person2: {}", person1 == person2);
    println!("person1 == person3: {}", person1 == person3);
    
    // 测试操作符重载
    println!("\n=== 操作符重载 ===");
    let v1 = Vector2D { x: 3, y: 4 };
    let v2 = Vector2D { x: 1, y: 2 };
    let v3 = v1 + v2;
    println!("{} + {} = {}", v1, v2, v3);
}