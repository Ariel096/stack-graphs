// -*- coding: utf-8 -*-
// ------------------------------------------------------------------------------------------------
// Copyright © 2021, stack-graphs authors.
// Licensed under either of Apache License, Version 2.0, or MIT license, at your option.
// Please see the LICENSE-APACHE or LICENSE-MIT files in this distribution for license details.
// ------------------------------------------------------------------------------------------------

use std::collections::HashSet;

use maplit::hashset;
use stack_graphs::graph::ExportedScopeNode;
use stack_graphs::graph::InternalScopeNode;
use stack_graphs::graph::Node;
use stack_graphs::graph::StackGraph;
use stack_graphs::graph::UnknownNode;

use crate::test_graphs::CreateStackGraph;

#[test]
fn can_create_symbols() {
    let mut graph = StackGraph::new();
    let a1 = graph.add_symbol("a");
    let a2 = graph.add_symbol("a");
    let b = graph.add_symbol("b");
    let c = graph.add_symbol("c");
    // The content of each symbol be comparable
    assert_eq!(graph[a1], graph[a2]);
    assert_ne!(graph[a1], graph[b]);
    assert_ne!(graph[a1], graph[c]);
    assert_ne!(graph[a2], graph[b]);
    assert_ne!(graph[a2], graph[c]);
    assert_ne!(graph[b], graph[c]);
    // and because we deduplicate symbols, the handles should be comparable too.
    assert_eq!(a1, a2);
    assert_ne!(a1, b);
    assert_ne!(a1, c);
    assert_ne!(a2, b);
    assert_ne!(a2, c);
    assert_ne!(b, c);
}

#[test]
fn can_iterate_symbols() {
    let mut graph = StackGraph::new();
    graph.add_symbol("a");
    graph.add_symbol("b");
    graph.add_symbol("c");
    // We should get all of the symbols that we've created — though there's no guarantee in which
    // order they'll come out of the iterator.
    let symbols = graph
        .iter_symbols()
        .map(|symbol| graph[symbol].as_str())
        .collect::<HashSet<_>>();
    assert_eq!(symbols, hashset! {"a", "b", "c"});
}

#[test]
fn can_display_symbols() {
    let mut graph = StackGraph::new();
    graph.add_symbol("a");
    graph.add_symbol("b");
    graph.add_symbol("c");
    let mut symbols = graph
        .iter_symbols()
        .map(|symbol| symbol.display(&graph).to_string())
        .collect::<Vec<_>>();
    symbols.sort();
    assert_eq!(symbols, vec!["a", "b", "c"]);
}

#[test]
fn can_iterate_nodes() {
    let mut graph = StackGraph::new();
    let file = graph.add_file("test.py");
    let h1 = graph.internal_scope(file, 0);
    let h2 = graph.internal_scope(file, 1);
    let h3 = graph.internal_scope(file, 2);
    let handles = graph.iter_nodes().collect::<HashSet<_>>();
    assert_eq!(
        handles,
        hashset! {graph.root_node(), graph.jump_to_node(), h1, h2, h3}
    );
}

#[test]
fn can_create_and_resolve_unknown_nodes() {
    let mut graph = StackGraph::new();
    let file = graph.add_file("test.py");
    let id = graph.new_node_id(file);
    let unknown = UnknownNode { id };
    let _handle = unknown.add_to_graph(&mut graph);
    let resolved = InternalScopeNode { id };
    assert!(graph.resolve_unknown_node(resolved.into()).is_ok());
    let conflict = ExportedScopeNode { id };
    assert!(matches!(
        graph.resolve_unknown_node(conflict.into()),
        Err(Node::InternalScope(_))
    ));
}
