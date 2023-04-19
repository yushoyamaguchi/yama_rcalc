//rustではOptionで包まないとnullに相当するものが使えない
pub struct ast_node{
    left: Option<Box<ast_node>>,
    right: Option<Box<ast_node>>,
    value: u64,
}

impl ast_node {
    fn new(value: u64, left: Option<Box<ast_node>>, right: Option<Box<ast_node>>)
            -> ast_node {
        ast_node {
            left: left,
            right: right,
            value,
        }
    }
}

fn recur(node: &mut ast_node) {
    node.value+=1;

    // 左の子ノードが存在する場合、再帰的に recur 関数を実行
    if let Some(ref mut  left_child) = node.left {
        recur(left_child);
    }

    // 右の子ノードが存在する場合、再帰的に recur 関数を実行
    if let Some(ref mut right_child) = node.right {
        recur(right_child);
    }

    println!("{}", node.value); // 現在のノードの value を出力
}

fn main() {
    println!("Hello, world!");
    let mut node1=ast_node::new(1, None,None);
    let mut node0=ast_node::new(0, Some(Box::new(node1)), None);
    let mut node2=ast_node::new(2, None,None);
    node0.right=Some(Box::new(node2));
    recur(&mut node0);

}
