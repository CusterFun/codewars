// 标注：position and_then 使用
fn next_item<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    slice
        .iter()
        .position(|x| *x == find)
        .and_then(|j| slice.get(j+1))
        .cloned()
}

#[test]
fn returns_expected() {
  assert_eq!(next_item_skip_while(&[0, 1], 0), Some(1));
  assert_eq!(next_item_find(&[0, 1], 1), None);
  assert_eq!(next_item((1..10).collect::<Vec<_>>().as_slice(), 7), Some(8));
  assert_eq!(next_item("Hello, world!".chars().collect::<Vec<_>>().as_slice(), 'w'), Some('o'));
  assert_eq!(next_item(&["Joe", "Bob", "Sally"], "Bob"), Some("Sally"))
}

// 标记：skip_while 使用
fn next_item_skip_while<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    slice
        .iter()
        .skip_while(|&x| *x != find)
        .nth(1)
        .cloned()
}

// find 和 next 使用
fn next_item_find<T: PartialEq<T> + Clone>(slice: &[T], find: T) -> Option<T> {
    let mut iter = slice.iter();
    iter
        .find(|&x| *x == find)
        .and_then(|_| iter.next())
        .cloned()

    // iter.find(|&x| *x == find);
    // iter.next().cloned()
}