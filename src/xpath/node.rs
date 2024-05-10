use std::fmt;

/// Represents a node in an XPath.
#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub predicates: Vec<Predicate>,
    pub next: Option<Box<Node>>,
}

/// Represents a predicate in an XPath node.
#[derive(Debug)]
pub enum Predicate {
    Attribute(Attribute),
    Function(Function),
}

/// Represents an attribute in an XPath node predicate.
#[derive(Debug)]
pub struct Attribute {
    pub name: String,
    pub op: Operator,
    pub value: Option<AttributeValue>,
}

/// Represents supported operators joining attribute key values.
#[derive(Debug, PartialEq)]
pub enum Operator {
    Equal(&'static str), // =
}

#[derive(Debug)]
pub enum AttributeValue {
    Text(String),
    Number(f64),
    Boolean(bool),
}

/// Represents a function in an XPath node predicate.
#[derive(Debug)]
pub struct Function {
    _name: String,
    _arguments: Vec<String>,
}

impl Node {
    pub fn new(name: String) -> Self {
        Node {
            name,
            predicates: Vec::new(),
            next: None,
        }
    }

    pub fn add_predicate(&mut self, predicate: Predicate) {
        self.predicates.push(predicate);
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.name)?;

        // append predicates if they exist
        if !self.predicates.is_empty() {
            let preds: Vec<String> = self.predicates.iter().map(|p| p.to_string()).collect();

            write!(f, "[{}]", preds.join(","))?;
        }

        // recursively call on next
        if let Some(ref next) = self.next {
            write!(f, "{}", next)?;
        }

        Ok(())
    }
}

impl fmt::Display for Predicate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Predicate::Attribute(ref attr) => write!(f, "{}", attr),
            Predicate::Function(ref func) => write!(f, "{}", func),
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operator::Equal(_) => write!(f, "="),
        }
    }
}

impl fmt::Display for AttributeValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AttributeValue::Text(ref val) => write!(f, "{}", val),
            AttributeValue::Number(num) => write!(f, "{}", num),
            AttributeValue::Boolean(b) => write!(f, "{}", b),
        }
    }
}

impl fmt::Display for Attribute {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.name, self.op)?;
        if let Some(ref value) = self.value {
            write!(f, "{}", value)?; // Directly format `value` into the formatter
        }
        Ok(())
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}(", self._name)?;
        for (i, arg) in self._arguments.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", arg)?;
        }
        write!(f, ")")
    }
}
