use rand::prelude::*;

fn main() {
    
    let mut index = 1;

    // Alle 3er Kombis für 12 Karten
    let mut variations12 = vec![vec![1; 3]; 220];
    let konarr12 = vec![1,2,3,4,5,6,7,8,9,10,11,12];
    
    // Alle 3er Kombis für 15 Karten
    let mut variations15 = vec![vec![1; 3]; 455];
    let konarr15 = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15];
 
    // Alle 3er Kombis für 18 Karten
    let mut variations18 = vec![vec![1; 3]; 816];
    let konarr18 = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18];

    // Alle 3er Kombis für 21 Karten
    let mut variations21 = vec![vec![1; 3]; 1330];
    let konarr21 = vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21];
    
    for j in 0..konarr12.len() - 2 {
        for k in 1..konarr12.len() - 1 {
            for l in 2..konarr12.len() {
                if l <= k || l <= j || k <= j {continue;};
                variations12[index-1][0] = konarr12[j];
                variations12[index-1][1] = konarr12[k];
                variations12[index-1][2] = konarr12[l];
                index += 1;
            }
        }
    }
    index = 1;
    for j in 0..konarr15.len() - 2 {
        for k in 1..konarr15.len() - 1 {
            for l in 2..konarr15.len() {
                if l <= k || l <= j || k <= j {continue;};
                variations15[index-1][0] = konarr15[j];
                variations15[index-1][1] = konarr15[k];
                variations15[index-1][2] = konarr15[l];
                index += 1;
            }
        }
    }
    index = 1;
    for j in 0..konarr18.len() - 2 {
        for k in 1..konarr18.len() - 1 {
            for l in 2..konarr18.len() {
                if l <= k || l <= j || k <= j {continue;};
                variations18[index-1][0] = konarr18[j];
                variations18[index-1][1] = konarr18[k];
                variations18[index-1][2] = konarr18[l];
                index += 1;
            }
        }
    }
    index = 1;
    for j in 0..konarr21.len() - 2 {
        for k in 1..konarr21.len() - 1 {
            for l in 2..konarr21.len() {
                if l <= k || l <= j || k <= j {continue;};
                variations21[index-1][0] = konarr21[j];
                variations21[index-1][1] = konarr21[k];
                variations21[index-1][2] = konarr21[l];
                index += 1;
            }
        }
    }
    // Alle Karten generieren - Array Allcars[index][feature]
    // --------------------------------------------------------
    let maxcard = 81;
    let maxfeat = 4;
    let mut allcards = vec![vec![0; maxfeat]; maxcard];
    let mut index = 0;

    for j in 1..=3 {
        for k in 1..=3 {
            for l in 1..=3 {
                for m in 1..=3 {
                    //print!("{}:", index);
                    allcards[index][0] = j;
                    allcards[index][1] = k;
                    allcards[index][2] = l;
                    allcards[index][3] = m;
                    index += 1;
                }
            }
        }
    }
    // --------------------------------------------------------

    // allcards shuffle für rnd Anordnung
    // --------------------------------------------------------
    let mut rng = rand::thread_rng();
    allcards.shuffle(&mut rng);
    // --------------------------------------------------------

    //println!("SHUFFLE:");
    //for i in 0..allcards.len() {
    //    println!("{}: {:?}", i, allcards[i]);
    //}

    // Ersten 15 Karten ausgeben und aus Array löschen
    // --------------------------------------------------------
    
    allcards[0][0] = 1;
    allcards[0][1] = 2;
    allcards[0][2] = 1;
    allcards[0][3] = 1;

    allcards[1][0] = 2;
    allcards[1][1] = 1;
    allcards[1][2] = 3;
    allcards[1][3] = 2;

    allcards[2][0] = 1;
    allcards[2][1] = 2;
    allcards[2][2] = 3;
    allcards[2][3] = 3;

    allcards[3][0] = 1;
    allcards[3][1] = 1;
    allcards[3][2] = 2;
    allcards[3][3] = 3;
    
    allcards[4][0] = 3;
    allcards[4][1] = 1;
    allcards[4][2] = 3;
    allcards[4][3] = 3;

    allcards[5][0] = 3;
    allcards[5][1] = 2;
    allcards[5][2] = 2;
    allcards[5][3] = 1;

    allcards[6][0] = 3;
    allcards[6][1] = 1;
    allcards[6][2] = 3;
    allcards[6][3] = 2;

    allcards[7][0] = 3;
    allcards[7][1] = 2;
    allcards[7][2] = 3;
    allcards[7][3] = 1;

    allcards[8][0] = 2;
    allcards[8][1] = 3;
    allcards[8][2] = 2;
    allcards[8][3] = 2;

    allcards[9][0] = 1;
    allcards[9][1] = 3;
    allcards[9][2] = 3;
    allcards[9][3] = 2;

    allcards[10][0] = 3;
    allcards[10][1] = 3;
    allcards[10][2] = 1;
    allcards[10][3] = 3;

    allcards[11][0] = 1;
    allcards[11][1] = 3;
    allcards[11][2] = 1;
    allcards[11][3] = 3;

    for i in 0..=11 {
        //println!("REMOVING:");
        print!("{:?} : ", allcards[i]);

        match allcards[i][0] {
            1 => print!("one - "),
            2 => print!("two - "),
            3 => print!("three - "),
            _ => print!("#"),
        }
        match allcards[i][1] {
            1 => print!("daimond - "),
            2 => print!("sqiggle - "),
            3 => print!("oval -"),
            _ => print!("#"),
        }
        match allcards[i][2] {
            1 => print!("solid - "),
            2 => print!("striped - "),
            3 => print!("open - "),
            _ => print!("#"),
        }
        match allcards[i][3] {
            1 => println!("red"),
            2 => println!("green"),
            3 => println!("purple"),
            _ => println!("#"),
        }
        //allcards.remove(0);
        println!("");
    }

    // --------------------------------------------------------
    
    for i in 0..220 {
        let x = (variations12[i][0]) - 1;
        let y = (variations12[i][1]) - 1;
        let z = (variations12[i][2]) - 1;

        let a = allcards[x][0] + allcards[y][0] + allcards[z][0];
        let b = allcards[x][1] + allcards[y][1] + allcards[z][1];
        let c = allcards[x][2] + allcards[y][2] + allcards[z][2];
        let d = allcards[x][3] + allcards[y][3] + allcards[z][3];

        if a % 3 != 0 {continue;};
        if b % 3 != 0 {continue;};
        if c % 3 != 0 {continue;};
        if d % 3 != 0 {continue;};

        print!("#{}: {:?} : ", i + 1, allcards[x]);
        print!("{:?} : ", allcards[y]);
        print!("{:?} - {} {} {} - ", allcards[z], x + 1, y + 1, z + 1);

        print!("Summe der n Elemente:");
        print!(" {} :", a);
        print!(" {} :", b);
        print!(" {} :", c);
        print!(" {}", d);
    
        println!("");
    }
    //println!("{:?}", variations);
}    
