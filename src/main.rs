use regex::Regex;

fn main() {
    let source = "\
first line
#begin
...
#end
second line";
    println!("{}", source);
    println!("------------------");

    let re = Regex::new("(?ms)#begin(.+)#end").unwrap();
    let inserted_data = "#begin\n1\n2\n#end";
    let result = re.replace(source, inserted_data);

    println!("{}", result);
}
