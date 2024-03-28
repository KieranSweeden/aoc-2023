use std::collections::HashMap;

pub struct InstructionParseError {
    reason: String,
}

#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    Left,
    Right,
}

impl Instruction {
    fn new(character: char) -> Result<Self, InstructionParseError> {
        match character {
            'L' => Ok(Instruction::Left),
            'R' => Ok(Instruction::Right),
            _ => Err(InstructionParseError {
                reason: format!("Failed to parse {} into instruction", character),
            }),
        }
    }
}

#[derive(Debug)]
pub struct InstructionsParseError {
    reason: String,
}

#[derive(Debug)]
pub struct Instructions(Vec<Instruction>);

impl Instructions {
    pub fn new(line_of_instructions: &str) -> Result<Self, InstructionsParseError> {
        let mut instructions: Vec<Instruction> = vec![];

        for instruction_char in line_of_instructions.chars() {
            let instruction = match Instruction::new(instruction_char) {
                Ok(instruction) => instruction,
                Err(err) => return Err(InstructionsParseError { reason: err.reason }),
            };

            instructions.push(instruction);
        }

        Ok(Self(instructions))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId(String);

impl NodeId {
    pub fn new(id: &str) -> Self {
        Self(id.to_string())
    }
}

#[derive(Debug)]
pub struct Node {
    id: NodeId,
    left_node_id: NodeId,
    right_node_id: NodeId,
}

impl Node {
    pub fn new(id: &str, left_node_id: &str, right_node_id: &str) -> Self {
        Self {
            id: NodeId::new(id),
            left_node_id: NodeId::new(left_node_id),
            right_node_id: NodeId::new(right_node_id),
        }
    }
}

#[derive(Debug)]
pub struct Network {
    nodes: HashMap<NodeId, Node>,
}

impl Network {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn calculate_steps_to_zzz(&self, instructions: &Instructions) -> Result<u32, String> {
        let mut steps: u32 = 0;
        let mut node_id = NodeId::new("AAA");
        let mut instruction_index = 0;

        while node_id.0 != "ZZZ" {
            let node = self.nodes.get(&node_id).expect("Failed");

            if instruction_index == instructions.0.len() {
                instruction_index = 0;
            }

            let instruction = instructions.0[instruction_index];

            match instruction {
                Instruction::Left => node_id = node.left_node_id.clone(),
                Instruction::Right => node_id = node.right_node_id.clone(),
            };

            instruction_index += 1;
            steps += 1;
        }

        Ok(steps)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn a_new_node_can_be_created_with_str() {
        let node_id = "AAA";
        let left_node_id = "BBB";
        let right_node_id = "CCC";
        let node = Node::new(node_id, left_node_id, right_node_id);
        assert_eq!(node.id.0, node_id);
        assert_eq!(node.left_node_id.0, left_node_id);
        assert_eq!(node.right_node_id.0, right_node_id);
    }
}
