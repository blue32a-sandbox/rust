fn main() {
    // str値は同じ量のメモリを使用する必要がある。
    // しかし両者の長さは異なる。
    let s1: str = "Hello there!";
    let s2: str = "How's it going?";

    // &strにすればいい
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
}
