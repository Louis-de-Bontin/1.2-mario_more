use std::io;

fn main() {
    let mut height;

    loop {
        height = String::new();
        println!("Choose a number between 1 and 8 :");

        io::stdin()
            .read_line(&mut height)
            .expect("Failed to read input.");
        
        let height: u32 = match height.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            },
        };
        
        if height ==0 || height > 8 {
            continue;
        }
        println!("The height you choose is : {}", height);

        let mut i: u32 = 0;
        while i < height {
            let mut j: u32 = 1;

            while j <= height {
                if height-j > i {
                    print!(" ");
                } else {
                    print!("#")
                }
                j += 1;
            }

            print!("  ");
            j -= 1;
            
            while j > 0 {
                if height-j > i {
                    print!(" ");
                } else {
                    print!("#")
                }
                j -= 1;
            }

            i +=1;
            print!("\n");
        }
        break;
    }

}
