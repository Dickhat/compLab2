use std::{fs::File, io::Read, rc::Rc, cell::RefCell};
use std::collections::HashMap;

enum NodeType {
    Country,
    Region,
    District,
    Root
}

enum StatesParser {
    Start,
    CountryState,
    RegionState,
    DistrictState,
    DelimeterState
}

struct Node {
    node_type: NodeType,
    name: String,
    area: i64,
    childs: Vec<Node>
}

impl Node 
{
    fn print_struct(&self, depth: usize) {
        let indent = "  ".repeat(depth); // Отступы по уровню вложенности
        println!("{}- {} (area: {})", indent, self.name, self.area);
    
        for child in &self.childs {
            child.print_struct(depth + 1);
        }
    }
}

#[derive(Debug)]
struct RuleNode {
    name: String,
    children: Vec<Rc<RefCell<RuleNode>>>,
}

impl RuleNode {
    fn change_name(&mut self, new_name: &str) {
        self.name = new_name.to_string();
    }

    // Конструктор нового узла
    fn new(name: &str) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(
            RuleNode {
                name: name.to_string(),
                children: Vec::new(),
                }
            )
        )
    }

    // Добавление дочернего узла
    fn add_child(parent: Rc<RefCell<Self>>, child: Rc<RefCell<Self>>) {
        parent.borrow_mut().children.push(child);
    }


    fn parse_tree(node: &Rc<RefCell<RuleNode>>, depth: usize) {
        let indent = "  ".repeat(depth); // Отступы по уровню вложенности
        let node_borrowed = node.borrow();
        
        println!("{}- {}", indent, node_borrowed.name);
        
        for child in &node_borrowed.children {
            RuleNode::parse_tree(child, depth + 1);
        }
    }
}

fn check_struct(node: &Node) -> bool {
    let mut is_valid = true;

    // 1. Проверяем, что площадь узла в диапазоне [1, 999]
    if node.area < 1 || node.area > 999 {
        println!(
            "Узел '{}' имеет недопустимую площадь {} (должно быть 1 ≤ area ≤ 999)",
            node.name, node.area
        );
        is_valid = false;
    }

    // 2. Если у узла есть дети, проверяем сумму их площадей
    if !node.childs.is_empty() {
        let sum_child_areas: i64 = node.childs.iter().map(|child| child.area).sum();

        if node.area != sum_child_areas {
            println!(
                "Площадь узла '{}' ({}) не равна сумме площадей детей ({})",
                node.name, node.area, sum_child_areas
            );
            
            // Выводим список площадей детей
            let child_areas: Vec<String> = node
                .childs
                .iter()
                .map(|child| format!("{} ({})", child.name, child.area))
                .collect();

            println!("Дочерние площади: [{}]", child_areas.join(", "));

            is_valid = false;
        }
    }

    // 3. Рекурсивно проверяем детей
    for child in &node.childs {
        if !check_struct(child) {
            is_valid = false;
        }
    }

    is_valid
}


fn country_node(var_stack: & mut Vec<String>,
                iter_lexem_table: & mut std::str::Split<'_, char>,
                rules_tree: Rc<RefCell<RuleNode>>,
                last_node: & mut Rc<RefCell<RuleNode>>
               ) -> Result<Node, String>
{
    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "String" || iter_lexem[1].to_string() != "country"
    {
        return Err("Must key word 'country'".to_string());
    }

    let node_country = RuleNode::new("country");
    RuleNode::add_child(Rc::clone(&rules_tree), Rc::clone(&node_country));

    var_stack.push(iter_lexem[1].to_string());

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like a ':' after 'country'".to_string());
    }

    let delim = RuleNode::new("N");
    RuleNode::add_child(Rc::clone(&rules_tree), Rc::clone(&delim));

    let delim_d = RuleNode::new(":");
    RuleNode::add_child(Rc::clone(&delim), Rc::clone(&delim_d));

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "String"
    {
        return Err("Must be name of country in a String format".to_string());
    }
    
    let C = RuleNode::new("C");
    RuleNode::add_child(Rc::clone(&delim), Rc::clone(&C));

    let node_name = RuleNode::new("name");
    RuleNode::add_child(Rc::clone(&C), Rc::clone(&node_name));

    var_stack.push(iter_lexem[1].to_string());

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();


    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like a ':' after name of country".to_string());
    }

    let A = RuleNode::new("A");
    RuleNode::add_child(Rc::clone(&rules_tree), Rc::clone(&A));

    let delim = RuleNode::new(":");
    RuleNode::add_child(Rc::clone(&A), Rc::clone(&delim));

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "Number"
    {
        return Err("Must be Numeric value of area".to_string());
    }

    let B = RuleNode::new("B");
    RuleNode::add_child(Rc::clone(&A), Rc::clone(&B));

    let area_node = RuleNode::new("area");
    RuleNode::add_child(Rc::clone(&B), Rc::clone(&area_node));

    var_stack.push(iter_lexem[1].to_string());

    let area = var_stack.pop().unwrap().parse::<i64>().unwrap();
    let name = var_stack.pop().unwrap();
    let node_type = NodeType::Country;

    area_node.borrow_mut().change_name(&area.to_string());
    node_name.borrow_mut().change_name(&name);

    var_stack.pop();

    let childs:Vec<Node> = Vec::new();

    let country_node = Node {
        node_type,
        name,
        area,
        childs
    };

    let D = RuleNode::new("D");
    //let D = Rc::new(RefCell::new(RuleNode::new("D")));
    RuleNode::add_child(Rc::clone(&rules_tree), Rc::clone(&D));

    *last_node =  D; // Теперь last_node изменится и за пределами функции

    Ok(country_node)
}

fn region_node(var_stack: & mut Vec<String>,
               iter_lexem_table: & mut std::str::Split<'_, char>,
               tree: & mut Node,
               rules_tree: Rc<RefCell<RuleNode>>,
               last_node: &mut Rc<RefCell<RuleNode>>
              ) -> Result<i64, String>
{
    let d = RuleNode::new("|");
    RuleNode::add_child(Rc::clone(&rules_tree), Rc::clone(&d));

    let R = RuleNode::new("R");
    RuleNode::add_child(Rc::clone(&rules_tree), Rc::clone(&R));

    let region = RuleNode::new("region");
    RuleNode::add_child(Rc::clone(&R), Rc::clone(&region));

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                               .unwrap()
                                               .split('-')
                                               .collect();

    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like ':'".to_string());
    }

    let N = RuleNode::new("N");
    RuleNode::add_child(Rc::clone(&R), Rc::clone(&N));

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                               .unwrap()
                                               .split('-')
                                               .collect();

    if iter_lexem[0].to_string() != "String"
    {
        return Err("Must be String value of region".to_string());
    }

    let d1 = RuleNode::new(":");
    RuleNode::add_child(Rc::clone(&N), Rc::clone(&d1));

    let C = RuleNode::new("C");
    RuleNode::add_child(Rc::clone(&N), Rc::clone(&C));

    let node_name = RuleNode::new("name");
    RuleNode::add_child(Rc::clone(&C), Rc::clone(&node_name));

    var_stack.push(iter_lexem[1].to_string());

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                               .unwrap()
                                               .split('-')
                                               .collect();

    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like ':'".to_string());
    }

    let A = RuleNode::new("A");
    RuleNode::add_child(Rc::clone(&R), Rc::clone(&A));

    let d1 = RuleNode::new(":");
    RuleNode::add_child(Rc::clone(&A), Rc::clone(&d1));

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                               .unwrap()
                                               .split('-')
                                               .collect();

    if iter_lexem[0].to_string() != "Number"
    {
        return Err("Must be numeric value of area".to_string());
    }

    let B = RuleNode::new("B");
    RuleNode::add_child(Rc::clone(&A), Rc::clone(&B));

    let area_node = RuleNode::new("area");
    RuleNode::add_child(Rc::clone(&B), Rc::clone(&area_node));

    var_stack.push(iter_lexem[1].to_string());

    let area = var_stack.pop().unwrap().parse::<i64>().unwrap();
    let name = var_stack.pop().unwrap();
    let node_type = NodeType::Region;

    area_node.borrow_mut().change_name(&area.to_string());
    node_name.borrow_mut().change_name(&name);

    let name_cp = name.clone();

    let childs:Vec<Node> = Vec::new();

    let region_node = Node {
        node_type,
        name,
        area,
        childs
    };

    add_child(tree, region_node);

    let mut index = 0;

    let D = RuleNode::new("D");
    RuleNode::add_child(Rc::clone(&R), Rc::clone(&D));

    *last_node = Rc::clone(&D); // Теперь last_node изменится и за пределами функции

    for element in & mut tree.childs
    {

        if element.name == name_cp
        {
            return Ok(index);
        }
        index = index + 1;
    };

    return Err("Node is not exist".to_string());
}

fn district_node(var_stack: & mut Vec<String>,
                 iter_lexem_table: & mut std::str::Split<'_, char>,
                 tree: & mut Node,
                 node_index: i64,
                 rules_tree: Rc<RefCell<RuleNode>>,
                 last_node: & mut Rc<RefCell<RuleNode>>
                ) -> Result<String, String>
{
    let d = RuleNode::new("|");
    RuleNode::add_child(Rc::clone(&rules_tree), Rc::clone(&d));

    let E = RuleNode::new("E");
    RuleNode::add_child(Rc::clone(&rules_tree), Rc::clone(&E));

    let district = RuleNode::new("district");
    RuleNode::add_child(Rc::clone(&E), Rc::clone(&district));

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like ':'".to_string());
    }

    let N = RuleNode::new("N");
    RuleNode::add_child(Rc::clone(&E), Rc::clone(&N));

    let d = RuleNode::new(":");
    RuleNode::add_child(Rc::clone(&N), Rc::clone(&d));

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "String"
    {
        return Err("Must be String value of district".to_string());
    }

    let C = RuleNode::new("C");
    RuleNode::add_child(Rc::clone(&N), Rc::clone(&C));

    let node_name = RuleNode::new("name");
    RuleNode::add_child(Rc::clone(&C), Rc::clone(&node_name));

    var_stack.push(iter_lexem[1].to_string());

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
        .unwrap()
        .split('-')
        .collect();

    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like ':'".to_string());
    }

    let A = RuleNode::new("A");
    RuleNode::add_child(Rc::clone(&E), Rc::clone(&A));

    let d1 = RuleNode::new(":");
    RuleNode::add_child(Rc::clone(&A), Rc::clone(&d1));

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
        .unwrap()
        .split('-')
        .collect();

    if iter_lexem[0].to_string() != "Number"
    {
        return Err("Must be numeric value of area".to_string());
    }

    let B = RuleNode::new("B");
    RuleNode::add_child(Rc::clone(&A), Rc::clone(&B));
    
    let area_node = RuleNode::new("area");
    RuleNode::add_child(Rc::clone(&B), Rc::clone(&area_node));

    var_stack.push(iter_lexem[1].to_string());

    let area = var_stack.pop().unwrap().parse::<i64>().unwrap();
    let name = var_stack.pop().unwrap();
    let node_type = NodeType::District;

    area_node.borrow_mut().change_name(&area.to_string());
    node_name.borrow_mut().change_name(&name);

    let childs:Vec<Node> = Vec::new();

    let district_node = Node {
        node_type,
        name,
        area,
        childs
    };

    let D = RuleNode::new("D");
    RuleNode::add_child(Rc::clone(&E), Rc::clone(&D));

    *last_node = Rc::clone(&D); // Теперь last_node изменится и за пределами функции


    add_child(& mut tree.childs[node_index as usize], district_node);

    return Ok("Node is added".to_string());
}

fn add_child(parent:& mut Node, child: Node)
{
    parent.childs.push(child);
}

pub fn syntax_parse(lexem_file:&str) -> Result<String, String>
{
    let mut parse_file = match File::open(lexem_file) {
        Ok(descript) => descript,
        Err(_) => return Err("Cannot open the lexems file ".to_string() + lexem_file)
    };

    let mut var_stack:Vec<String> = Vec::new();

    let mut buffer_lexems: String = String::new();

    let count_byte_read = match parse_file.read_to_string(& mut buffer_lexems) {
        Err(_) => return Err("Can't reads byte out of file".to_string()),
        Ok(count) => count
    };

    let mut iter_lexem_table: std::str::Split<'_, char> = buffer_lexems.split('\n');
    let mut cur_state:StatesParser = StatesParser::Start;

    let mut rules_tree = RuleNode::new("S");
    let mut last_node = Rc::clone(&rules_tree);

    let mut tree = country_node(&mut var_stack, &mut iter_lexem_table, Rc::clone(&rules_tree), &mut last_node).unwrap();
    let mut node_index = 0;

    loop 
    {
        let mut next_lexem = iter_lexem_table.next();

        if next_lexem == None
        {
            break;
        }

        let mut iter_lexem:Vec<&str> = next_lexem.unwrap()
                                                .split('-')
                                                .collect();


        if iter_lexem[0].to_string() == "" 
        {
            break;
        }

        if iter_lexem[0].to_string() != "Delimeter" && iter_lexem[1].to_string() != "|"
        {
            return Err("Must be delimeter '|' after 'Region' or 'District' sentence".to_string());
        }

        iter_lexem = iter_lexem_table.next()
                                    .unwrap()
                                    .split('-')
                                    .collect();

        if iter_lexem[0].to_string() == "String" && iter_lexem[1].to_string() == "region"
        {
            match region_node(&mut var_stack, &mut iter_lexem_table, &mut tree, Rc::clone(&last_node), &mut last_node)
            {
                Ok(node) => node_index = node,
                Err(text) => {
                    println!("{}", text);
                    continue;
                }
            } ;
        }
        else if iter_lexem[0].to_string() == "String" && iter_lexem[1].to_string() == "district" 
        {
            match district_node(&mut var_stack, &mut iter_lexem_table,  &mut tree , node_index, Rc::clone(&last_node), &mut last_node)
            {
                Ok(_) => continue,
                Err(text) => {
                    println!("{}", text);
                    continue;
                }
            };
        }
        else {
            return Err("Must key word 'region' or 'district'".to_string());
        }
    }

    println!("Parse tree output");
    RuleNode::parse_tree(&rules_tree, 0);

    match check_struct(&tree){
        true => println!("Structure is valid"),
        false => return Err("Structure is not valid".to_string())
    }


    Ok("OKi doki".to_string())
}