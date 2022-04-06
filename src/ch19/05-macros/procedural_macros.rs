use proc_macro;

// some_attributeは、特定のマクロを使用するためのプレースホルダー
// 手続き型マクロを定義する関数はTokenStreamを入力として受け取り，
// TokenStreamを出力として生成する。
#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
