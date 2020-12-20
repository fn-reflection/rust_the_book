// testsディレクトリには結合テストをかく
// #[cfg(test)]は必要ない
extern crate testing; // testsディレクトリはsrcとは別のクレートになるのでexternが必要
mod common;
#[test]
fn it_adds_two() { common::setup(); assert_eq!(4, testing::add_two(2)); }