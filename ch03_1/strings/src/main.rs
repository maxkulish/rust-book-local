use std::error::Error;
use std::fmt;

struct Mistake {
    path: &'static str,

    text: String,
    locations: Vec<usize>,
}

impl Mistake {
    fn line_bounds(&self, index: usize) -> (usize, usize) {
        let length = self.text.len();

        let before = &self.text[..index];
        let start = before.rfind("\n").map(|x| x + 1).unwrap_or(0);

        let after = &self.text[index + 1..];
        let end = after.find("\n").map(|x| x + index + 1).unwrap_or(length);

        (start, end)
    }
}

impl<'a> fmt::Display for Mistake {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for &location in &self.locations {
            let (start, end) = self.line_bounds(location);
            let line = &self.text[start..end];

            let line_num = self.text[..start].matches("\n").count() + 1;
            let comma_index = location - start;

            write!(f, "{}: commas are forbidden: \n", self.path)?;
            write!(f, "\n");
            // print the line number
            write!(f, "{:>8} | {}\n", line_num, line)?;

            // indicate where the comma is
            write!(f, "{}\x1b[31m^\x1b[0m\n\n", " ".repeat(11 + comma_index))?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let paths = [
        "./samples/sample1.txt",
        "./samples/sample2.txt",
        "./samples/sample3.txt",
    ];

    // check all documents
    let mut results = vec![];
    for path in &paths {
        let result = check(path)?;
        results.push(result);
    }

    for result in results {
        report(result)
    }

    Ok(())
}

fn check<'a>(path: &'static str) -> Result<Option<Mistake>, Box<dyn Error>> {
    let text = std::fs::read_to_string(path)?;
    let locations: Vec<_> = text.match_indices(",").map(|(index, _)| index).collect();

    Ok(if locations.is_empty() {
        None
    } else {
        Some(Mistake {
            path,
            text,
            locations,
        })
    })
}

fn report(result: Option<Mistake>) {
    if let Some(mistake) = result {
        println!("{}", mistake)
    }
}
