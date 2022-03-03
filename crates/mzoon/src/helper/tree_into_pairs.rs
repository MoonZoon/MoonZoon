use std::{collections::VecDeque, rc::Rc};
use anyhow::{Result, bail, Error};
use fehler::throws;

pub enum NodeContent<N, K, V> {
    Children(Box<dyn Iterator<Item = (K, N)>>),
    Value(V),
}

struct StackItem<N, K> {
    node_metadata: Option<NodeMetadata<K>>,
    node: N,
}

enum NodeMetadata<K> {
    Keys { parent_key: Rc<K>, key: K },
    Key(K)
}

// ------ tree_into_pairs ------

#[throws]
pub fn tree_into_pairs<N, K, V>(
    tree: N,
    child_key: impl Fn(&K, &K) -> K,
    children_or_value: impl Fn(N) -> Result<NodeContent<N, K, V>>,
) -> Vec<(K, V)> 
{
    let mut pairs = Vec::<(K, V)>::new();

    let mut stack = VecDeque::<StackItem<N, K>>::new();
    let root = StackItem {
        node_metadata: None,
        node: tree,
    };
    stack.push_back(root);

    while let Some(StackItem { node_metadata, node }) = stack.pop_front() {
        let output_key = node_metadata.map(|metadata| {
            match metadata {
                NodeMetadata::Keys { parent_key, key } => {
                    child_key(&parent_key, &key)
                }
                NodeMetadata::Key(key) => key,
            }
        });
        let output_value = match children_or_value(node)? {
            NodeContent::Children(children) => {
                let parent_key = output_key.map(Rc::new);
                stack.extend(children.map(|(key, node)| {
                    let node_metadata = if let Some(parent_key) = parent_key.clone() {
                        NodeMetadata::Keys { parent_key, key }
                    } else {
                        NodeMetadata::Key(key)
                    };
                    StackItem { 
                        node_metadata: Some(node_metadata),
                        node 
                    }
                }));
                continue;
            },
            NodeContent::Value(value) => value,
        }; 
        if let Some(output_key) = output_key {
            pairs.push((output_key, output_value));
        } else {
            bail!("Root node cannot be a leaf node")
        }
    }
    pairs
}
