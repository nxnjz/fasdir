pub fn parse_extension(url: &str) -> Option<String> {
    url.split("//")
        .filter(|x| x.contains('/'))
        .last()
        .unwrap_or("")
        .split('/')
        .skip(1)
        .filter(|x| x.contains('.'))
        .last()
        .unwrap_or("")
        .split('.')
        .filter(|x| x != &"")
        .last()
        .map(|x| format!(".{}", x))
}

// https://nxnjz.net/
// https://nxnjz.net
// https://nxnjz.net/test
// https://nxnjz.net/test.txt
// https://nxnjz.net/test/
// https://nxnjz.net/test/file.txt

pub fn pseudo_extension(url: &str) -> String {
    if url.ends_with('/') {
        return "/".to_string();
    } else if !url.split("//").last().unwrap_or("").contains('/') {
        return "/".to_string();
    } else {
        let filename = url
            .split("//")
            .last()
            .unwrap_or("")
            .split('/')
            .filter(|x| x != &"")
            .last();
        if filename.is_none() {
            return "".to_string();
        } else if filename.unwrap().contains('.') {
            return format!(".{}", filename.unwrap().split('.').last().unwrap());
        } else {
            return "".to_string();
        }
    }
}
