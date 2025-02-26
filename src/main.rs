use std::io::{self, Write};

fn main() {
    const ACCEPTED_FORMATS: [&str; 5] = ["ph", "poh", "h", "oh", "q"];
    const KW: f64 = 1e-14;
    loop {
        let mut mode = String::new();
        let mut input = String::new();
        loop {
            print!("Enter input format[ph, poh, h, oh, q]: ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut mode).unwrap();
            mode = mode.trim().to_lowercase();
            if mode == "q" {
                return;
            }
            if ACCEPTED_FORMATS.contains(&mode.trim()) {
                break;
            }
            println!("Invalid input format");
        }
        print!("Enter value: ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();
        let value: f64 = match input.trim().parse() {
            Ok(v) => v,
            Err(_) => {
                println!("Invalid value entered. Please enter a valid number.");
                continue;
            }
        };
        match mode.as_str() {
            "ph" => {
                let h = 10.0_f64.powf(-value);
                let oh = KW / h;
                let ph = value;
                let poh = 14.0 - ph;
                println!(
                    "h: {:.4e}, oh: {:.4e}, ph: {:.4e}, poh: {:.4e}",
                    h, oh, ph, poh
                );
            }
            "poh" => {
                let poh = value;
                let oh = 10.0_f64.powf(-poh);
                let h = KW / oh;
                let ph = 14.0 - poh;
                println!(
                    "h: {:.4e}, oh: {:.4e}, ph: {:.4e}, poh: {:.4e}",
                    h, oh, ph, poh
                );
            }
            "h" => {
                let h = value;
                let oh = KW / h;
                let ph = -h.log10();
                let poh = 14.0 - ph;
                println!(
                    "h: {:.4e}, oh: {:.4e}, ph: {:.4e}, poh: {:.4e}",
                    h, oh, ph, poh
                );
            }
            "oh" => {
                let oh = value;
                let h = KW / oh;
                let poh = -oh.log10();
                let ph = 14.0 - poh;
                println!(
                    "h: {:.4e}, oh: {:.4e}, ph: {:.4e}, poh: {:.4e}",
                    h, oh, ph, poh
                );
            }
            "q" => return,
            _ => unreachable!(),
        }
    }
}
