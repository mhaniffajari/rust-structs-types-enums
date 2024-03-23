enum WineGrapes {
    CarbenetFranc,
    Tannat,
    Merlot
}

fn taste_wine (grapes: WineGrapes) {
    match grapes {
        WineGrapes::CarbenetFranc => println!("This wine is a Carbenet Franc"),
        _ => println!("This wine is not a Carbenet Franc")
    }
}

fn main(){
    taste_wine(WineGrapes::CarbenetFranc);
}