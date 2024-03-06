fn main() {
    let mut container_or_host = String::from("\\h");
    if let Ok(file) = std::fs::read("/run/.containerenv") {
        if let Ok(file) = String::from_utf8(file) {
            let mut name = String::new();
            let mut image = String::new();
            for line in file.split("\n") {
                let mut s = line.split("=");
                let k = s.next();
                let v = s.next();
                if k.is_some() && v.is_some() {
                    let k = k.unwrap();
                    let v = v.unwrap();
                    if k == "name" {
                        name = v.replace("\"", "");
                    }
                    if k == "image" {
                        image = v
                            .replace("\"", "")
                            .replace("registry.fedoraproject.org/", "")
                            .replace("quay.io/toolbx/", "")
                            .replace("-toolbox", "");
                    }
                }
            }
            container_or_host = format!("{name} ({image})");
        }
    };
    println!("\\[\\e[92m\\]\\u@\\[\\e[0m\\]\\[\\e[92m\\]{}\\[\\e[0m\\] \\[\\e[94m\\]\\w\\[\\e[0m\\] \\$ ",container_or_host);
}
