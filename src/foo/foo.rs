pub fn copy_from_slice<T: Copy>(dst: &mut Vec<T>, src: &[T]) {
  dst.copy_from_slice(&src);
}
