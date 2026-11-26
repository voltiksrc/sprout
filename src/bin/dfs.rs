use std::collections::{HashMap, HashSet};

fn dfs<'a>(
    node: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
    resolved: &mut HashSet<&'a str>,
    visiting: &mut HashSet<&'a str>,
    path: &mut Vec<&'a str>,
    order: &mut Vec<&'a str>,
) -> Result<(), String> {
    if visiting.contains(node) {
        let start = path.iter().position(|&n| n == node).unwrap();

        let mut cycle = path[start..].to_vec();
        cycle.push(node);

        return Err(format!("Circular dependency chain: {}", cycle.join(" -> ")));
    }

    // Skip already resolved files
    if resolved.contains(node) {
        return Ok(());
    }

    visiting.insert(node);
    path.push(node);

    let deps = graph
        .get(node)
        .ok_or_else(|| format!("Unknown dependency: {node}"))?;

    for dep in deps {
        dfs(dep, graph, resolved, visiting, path, order)?;
    }

    visiting.remove(node);
    path.pop();
    resolved.insert(node);
    order.push(node);

    Ok(())
}

pub fn resolve<'a>(
    node: &'a str,
    graph: &HashMap<&'a str, Vec<&'a str>>,
) -> Result<Vec<&'a str>, String> {
    let mut resolved = HashSet::new();
    let mut visiting = HashSet::new();
    let mut path = Vec::new();
    let mut order = Vec::new();

    dfs(
        node,
        graph,
        &mut resolved,
        &mut visiting,
        &mut path,
        &mut order,
    )?;

    Ok(order)
}

fn main() {
    let graph = HashMap::from([
        ("app", vec!["main", "util"]),
        ("main", vec!["common"]),
        ("util", vec!["common"]),
        ("common", vec![]),
    ]);

    match resolve("app", &graph) {
        Ok(order) => println!("{}", order.join(" -> ")),
        Err(err) => eprintln!("{err}"),
    }
}
