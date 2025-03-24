use std::{fs::File, io::Read, ops::Index};

enum NodeType {
    Country,
    Region,
    District
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
                iter_lexem_table: & mut std::str::Split<'_, char>
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

    var_stack.push(iter_lexem[1].to_string());

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like a ':' after 'country'".to_string());
    }

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "String"
    {
        return Err("Must be name of country in a String format".to_string());
    }
    
    var_stack.push(iter_lexem[1].to_string());

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();


    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like a ':' after name of country".to_string());
    }

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "Number"
    {
        return Err("Must be Numeric value of area".to_string());
    }

    var_stack.push(iter_lexem[1].to_string());

    let area = var_stack.pop().unwrap().parse::<i64>().unwrap();
    let name = var_stack.pop().unwrap();
    let node_type = NodeType::Country;

    var_stack.pop();

    let childs:Vec<Node> = Vec::new();

    let country_node = Node {
        node_type,
        name,
        area,
        childs
    };

    Ok(country_node)
}

fn region_node(var_stack: & mut Vec<String>,
               iter_lexem_table: & mut std::str::Split<'_, char>,
               tree: & mut Node
              ) -> Result<i64, String>
{
    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                               .unwrap()
                                               .split('-')
                                               .collect();

    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like ':'".to_string());
    }

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                               .unwrap()
                                               .split('-')
                                               .collect();

    if iter_lexem[0].to_string() != "String"
    {
        return Err("Must be String value of region".to_string());
    }

    var_stack.push(iter_lexem[1].to_string());

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                               .unwrap()
                                               .split('-')
                                               .collect();

    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like ':'".to_string());
    }

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                               .unwrap()
                                               .split('-')
                                               .collect();

    if iter_lexem[0].to_string() != "Number"
    {
        return Err("Must be numeric value of area".to_string());
    }

    var_stack.push(iter_lexem[1].to_string());

    let area = var_stack.pop().unwrap().parse::<i64>().unwrap();
    let name = var_stack.pop().unwrap();
    let node_type = NodeType::Region;

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
                 node_index: i64
                ) -> Result<String, String>
{
    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like ':'".to_string());
    }

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
                                                .unwrap()
                                                .split('-')
                                                .collect();

    if iter_lexem[0].to_string() != "String"
    {
        return Err("Must be String value of district".to_string());
    }

    var_stack.push(iter_lexem[1].to_string());

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
        .unwrap()
        .split('-')
        .collect();

    if iter_lexem[0].to_string() != "Delimeter" || iter_lexem[1].to_string() != ":"
    {
        return Err("Must be delimeter like ':'".to_string());
    }

    let iter_lexem:Vec<&str> = iter_lexem_table.next()
        .unwrap()
        .split('-')
        .collect();

    if iter_lexem[0].to_string() != "Number"
    {
        return Err("Must be numeric value of area".to_string());
    }

    var_stack.push(iter_lexem[1].to_string());

    let area = var_stack.pop().unwrap().parse::<i64>().unwrap();
    let name = var_stack.pop().unwrap();
    let node_type = NodeType::District;

    let childs:Vec<Node> = Vec::new();

    let district_node = Node {
        node_type,
        name,
        area,
        childs
    };

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

    let mut tree = country_node(&mut var_stack, &mut iter_lexem_table).unwrap();
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
            match region_node(&mut var_stack, &mut iter_lexem_table, &mut tree)
            {
                Ok(node) => node_index = node,
                Err(text) => {
                    println!("{}", text);
                    continue;
                }
            } ;
        }
        else if iter_lexem[0].to_string() == "String" && iter_lexem[1].to_string() == "District" 
        {
            match district_node(&mut var_stack, &mut iter_lexem_table,  &mut tree , node_index)
            {
                Ok(_) => continue,
                Err(text) => {
                    println!("{}", text);
                    continue;
                }
            };
        }
        else {
            return Err("Must key word 'region' or 'District'".to_string());
        }
    }

    tree.print_struct(0);

    match check_struct(&tree){
        true => println!("Structure is valid"),
        false => return Err("Structure is not valid".to_string())
    }

    Ok("OKi doki".to_string())
}