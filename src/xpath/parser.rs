use super::errors::ParseError;
use super::node::Node;
use super::node::{Attribute, AttributeValue, Operator, Predicate};
use super::xpath::XPath;

#[derive(Debug, Clone, Copy, PartialEq)]
enum ParseState {
    Node,
    Predicate,
    Attribute,
    Value,
    // Function,
    // Argument,
}

impl XPath {
    pub fn parse(input: &str) -> Result<XPath, ParseError> {
        // initialize the XPath and state variables
        let mut xpath = XPath::new();
        let mut node: Option<Node> = None;

        let mut mode = ParseState::Node;
        let mut buffer = String::new();

        for c in input.chars() {
            match (c, mode) {
                ('/', ParseState::Node) => {
                    if buffer.is_empty() && node.is_none() {
                        continue; // root node
                    }

                    if buffer.is_empty() && node.is_some() {
                        if let Some(n) = node.take() {
                            xpath.append_node(n);
                        }
                        continue;
                    }

                    if !buffer.is_empty() && node.is_none() {
                        let new_node = Node::new(buffer.clone());
                        xpath.append_node(new_node);
                        buffer.clear();
                    }
                }
                ('[', ParseState::Node) => {
                    if node.is_none() && !buffer.is_empty() {
                        node = Some(Node::new(buffer.clone()));
                    }

                    mode = ParseState::Predicate;
                    buffer.clear();
                }
                ('=', ParseState::Predicate) | ('=', ParseState::Attribute) => {
                    if let Some(ref mut n) = node {
                        let attribute = Attribute {
                            name: buffer.clone(),
                            op: Operator::Equal("="),
                            value: None,
                        };

                        n.add_predicate(Predicate::Attribute(attribute));
                    }

                    mode = ParseState::Value;
                    buffer.clear();
                }
                (']', ParseState::Value) | (',', ParseState::Value) => {
                    // TODO: fail if buffer empty?
                    // Update the last Attribute that was appended to the node
                    // in the previous state
                    if let Some(ref mut n) = node {
                        if let Some(last) = n.predicates.last_mut() {
                            if let Predicate::Attribute(attr) = last {
                                attr.value = Some(AttributeValue::Text(buffer.clone()));
                            }
                        }
                    }

                    // The logic above is the same for both cases, except the next
                    // state is different based on the character
                    mode = if c == ']' {
                        ParseState::Node // Predicate is complete
                    } else {
                        ParseState::Attribute // Expect more attributes
                    };

                    buffer.clear();
                }
                (_, ParseState::Node) => {
                    // support // root node
                    if !c.is_alphanumeric() || c == '_' {
                        return Err(ParseError::NodeNameError);
                    }

                    buffer.push(c);
                }

                _ => {
                    buffer.push(c);
                }
            }
        }

        // Process remaining buffer if it exists
        if !buffer.is_empty() {
            if mode != ParseState::Node {
                return Err(ParseError::StateError);
            }

            let new_node = Node::new(buffer);
            xpath.append_node(new_node);
        }

        Ok(xpath)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        struct TestCase {
            input: &'static str,
            expected: Result<String, ParseError>,
        }

        let tests = vec![
            TestCase {
                input: "/valid/xpath",
                expected: Ok("/valid/xpath".to_string()),
            },
            TestCase {
                input: "/valid[a1=v1,a2=v2]/xpath[another=one]/leaf",
                expected: Ok("/valid[a1=v1,a2=v2]/xpath[another=one]/leaf".to_string()),
            },
            TestCase {
                input: "/#invalid/node",
                expected: Err(ParseError::NodeNameError),
            },
            TestCase {
                input: "/invalid[/xpath",
                expected: Err(ParseError::StateError),
            },
            TestCase {
                input: "/invalid[/x]path",
                expected: Err(ParseError::StateError),
            },
        ];

        for test in tests {
            let result = XPath::parse(test.input).map(|xpath| xpath.to_string());

            assert_eq!(
                result, test.expected,
                "Failed test for input: {}",
                test.input
            );
        }
    }
}
