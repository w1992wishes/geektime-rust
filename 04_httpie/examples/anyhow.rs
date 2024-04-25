use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let result: String = do_something()?;
    println!("Result: {}", result);
    Ok(())
}

fn do_something() -> Result<String> {
    let value: &str = "some value";
    
    if value.len() < 10 {
        // 使用 anyhow 创建一个错误，并添加上下文信息
        return Err(anyhow!("Length is too short"));
    }
    
    Ok(value.to_string())
}
