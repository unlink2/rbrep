pub struct Node<T> {
    next: Box<Node<T>>,
    data: T,
}
