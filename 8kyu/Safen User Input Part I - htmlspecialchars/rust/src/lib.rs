fn html_special_chars(html: &str) -> String {
    // unimplemented!();
    html.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::html_special_chars;
    
    use rand;
    use rand::Rng;
    
    // fold 用法
    fn html_special_chars_fold(html: &str) -> String {
        html.chars().fold(String::new(), |mut s, x| {
            match x {
                '<' => s.push_str("&lt;"),
                '>' => s.push_str("&gt;"),
                '"' => s.push_str("&quote;"),
                '&' => s.push_str("&amp;"),
                _ => s.push(x),
            };
            s
        })
    }

    fn solution(html: &str) -> String {
        html.replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
    }
    
    #[test]
    fn sample_tests() {
        assert_eq!(html_special_chars_fold("<h2>Hello World</h2>"), 
            "&lt;h2&gt;Hello World&lt;/h2&gt;");
        assert_eq!(html_special_chars("Hello, how would you & I fare?"), 
            "Hello, how would you &amp; I fare?");
        assert_eq!(html_special_chars("How was \"The Matrix\"?  Did you like it?"), 
            "How was &quot;The Matrix&quot;?  Did you like it?");
        assert_eq!(html_special_chars("<script>alert('Website Hacked!');</script>"), 
            "&lt;script&gt;alert('Website Hacked!');&lt;/script&gt;");
    }
    
    #[test]
    fn advanced_example() {
        let mut message = "<h2>Lorem Ipsum</h2>\n".to_string();
        message.push_str(&"<p>Lorem ipsum dolor sit amet, adispicing eu.</p>\n");
        message.push_str(&"<p>Lorem ipsum dolor sit amet, adispicing eu.</p>\n");
        message.push_str(&"<p>Hello Website Owner,<br /> I think you & I can make an awesome \"hacking pair\".  What do you think?</p>\n");
        message.push_str(&"<script>\n");
        message.push_str(&"// Some very malicious code here - a redirect!\n");
        message.push_str(&"window.location = \"http://www.hacked.com/\";\n");
        message.push_str(&"</script>");
        
        // Coercion from struct String to str
        let message : &str = &message;
        
        let mut safened_message = "&lt;h2&gt;Lorem Ipsum&lt;/h2&gt;\n".to_string();
        safened_message.push_str(&"&lt;p&gt;Lorem ipsum dolor sit amet, adispicing eu.&lt;/p&gt;\n");
        safened_message.push_str(&"&lt;p&gt;Lorem ipsum dolor sit amet, adispicing eu.&lt;/p&gt;\n");
        safened_message.push_str(&"&lt;p&gt;Hello Website Owner,&lt;br /&gt; I think you &amp; I can make an awesome &quot;hacking pair&quot;.  What do you think?&lt;/p&gt;\n");
        safened_message.push_str(&"&lt;script&gt;\n");
        safened_message.push_str(&"// Some very malicious code here - a redirect!\n");
        safened_message.push_str(&"window.location = &quot;http://www.hacked.com/&quot;;\n");
        safened_message.push_str(&"&lt;/script&gt;");
        
        // Coercion from struct String to str
        let safened_message : &str = &safened_message;
        assert_eq!(html_special_chars(message), safened_message);
    }
    
    #[test]
    fn random_test() {
        let chars : Vec<char> = "&<>\"abcdefghijklmnopqrstuvwxyz".chars().collect();
    
        for _ in 0..100 {
            let mut html = String::new();
            let length = rand::thread_rng().gen_range(1..1001);
            
            for _ in 0..length {
                html.push(chars[rand::thread_rng().gen_range(0..chars.len())]);
            }
            
            let html : &str = &html;
            
            assert_eq!(html_special_chars(html), solution(html));
        }
    }
}