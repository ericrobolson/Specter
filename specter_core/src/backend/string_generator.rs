pub struct StringGenerator {
    indent_count: usize,
    value: String,
}
impl StringGenerator {
    pub fn new() -> Self {
        return Self {
            indent_count: 0,
            value: String::new(),
        };
    }

    pub fn from_string(value: String) -> Self {
        let mut gen = Self::new();

        gen.value = value;

        return gen;
    }

    pub fn indent(&mut self) -> &mut Self {
        self.indent_count += 1;
        return self;
    }

    pub fn unindent(&mut self) -> &mut Self {
        if 0 < self.indent_count {
            self.indent_count -= 1;
        }

        return self;
    }

    pub fn prepend(&mut self, value: String) -> &mut Self {
        self.value.insert_str(0, value.as_str());

        return self;
    }

    pub fn append(&mut self, value: String) -> &mut Self {
        self.value += &value;

        return self;
    }

    pub fn add_lines(&mut self, lines: usize) -> &mut Self {
        for _ in 0..lines {
            self.add_line();
        }

        return self;
    }

    pub fn add_line(&mut self) -> &mut Self {
        self.value += "\n";

        for _ in 0..self.indent_count {
            self.value += "\t";
        }

        return self;
    }
    pub fn to_string(&self) -> String {
        return self.value.clone();
    }
}
