fn main() {
    let mut functions: Vec<(String, String, String)> = vec![];
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        if line.trim() == "quit" {
            break;
        }
        if line.trim().starts_with("define") {
            functions.push(define(line));
        } else {
            let function_name = line.split(" ").nth(0).unwrap();
            let function = functions.iter().find(|f| f.0 == function_name).unwrap();
            let value = line.split(" ").nth(1).unwrap().trim();
            let values = line.split(" ").skip(1).collect::<Vec<_>>();
            let mut new_line = function.2.replace(&function.1, value);

            //println!("function.1={}", function.1);
            function.1.split(",").enumerate().for_each(|(index, var)| {
                //    println!("var={}", var);
                //    println!("values[index]={}", values[index]);
                new_line = new_line.replace(var, values[index]);
            });

            //println!("value={}", value);
            //println!("new_line={}", new_line);
            println!("resultado = {}", parse(new_line));
        }
    }
    println!("exiting");
}

fn define(line: String) -> (String, String, String) {
    let mut command = line.clone();

    command = command.replace("define", "");

    let command_function_name = command.clone();
    let function_name = command_function_name.trim().split(" ").nth(0).unwrap();
    command = command.replace(function_name, "");
    let vars = command
        .split("=")
        .nth(0)
        .unwrap()
        .trim()
        .split(" ")
        .collect::<Vec<_>>();

    //println!("function_name={}, args={}", function_name, vars.join(","));
    let body = command.split("=").nth(1).unwrap().trim();

    return (
        function_name.to_string(),
        vars.join(",").to_string(),
        body.to_string(),
    );
}
fn parse(line: String) -> i32 {
    let mut op = "+";
    let func = if line.contains("+") {
        |x: i32, res: i32| x + res
    } else {
        op = "-";
        |x: i32, res: i32| {
            if res == 0 {
                x - res
            } else {
                res - x
            }
        }
    };
    let parts = line.trim().split(op);
    let mut res = 0;
    // println!("[DEBUG] res={}", res);
    // println!("[DEBUG] line: {}", line);
    for part in parts {
        //println!("[DEBUG] part={}", part);
        res = func(part.trim().parse::<i32>().unwrap(), res);
        //println!("[DEBUG] res={}", res);
    }

    return res;
}
//
//soma X e Y = X + Y
