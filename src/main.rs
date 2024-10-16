use rand::Rng;

// Fonction pour l'exponentiation modulaire efficace
fn mod_exp(base: u64, exp: u64, modulo: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulo;
    let mut exp = exp;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulo;
        }
        base = (base * base) % modulo;
        exp /= 2;
    }
    result
}

// Alice génère x, calcule h_A = 3^x mod 2^63 et envoie h_A
fn generate_public_key() -> (u64, u64) {
    let x: u64 = rand::thread_rng().gen_range(1..=u32::MAX as u64); // x ∈ [1, 2^32]
    let h = mod_exp(3, x, 2^63);
    (x, h) // Retourne x et h_A pour l'étape suivante
}

fn calculate_secret(h: u64, x: u64) -> u64 {
    mod_exp(h, x, 2^63)
}



// Eve intercepte les messages et remplace h_A et h_B
fn eve_intercept() -> (u64, u64, u64, u64) {
    // Eve génère son propres secrets e_x
    let e_x: u64 = rand::thread_rng().gen_range(1..=u32::MAX as u64);
    let e_y: u64 = rand::thread_rng().gen_range(1..=u32::MAX as u64);

    let h_ea = mod_exp(3, e_x, 2^63); 
    let h_eb = mod_exp(3, e_y, 2^63); 

    println!("Eve envoie h_EA = {}, h_EB = {}", h_ea, h_eb);

    (h_ea, h_eb, e_x, e_y)
}

fn exercice1(){
    print!("a)\n");

    print!("Simulation de l'échange de clé entre Alice et Bob avec Eve en observatrice passive\n");

    // Étapes d'Alice
    let (x1, h_a1) = generate_public_key();
    println!("Alice envoie h_A = {}", h_a1);

    // Étapes de Bob
    let (y1, h_b1) = generate_public_key();
    println!("Bob envoie h_B = {}", h_b1);

    // Alice calcule sa clé
    let k_a1 = calculate_secret(h_b1, x1);
    println!("Alice calcule k_A = {}", k_a1);

    // Bob calcule sa clé
    let k_b1 = calculate_secret(h_a1, y1);
    println!("Bob calcule k_B = {}", k_b1);

    // Vérification que les deux clés sont identiques
    if k_a1 == k_b1 {
        println!("Échange de clé réussi, k_A = k_B = {}", k_a1);
    } else {
        println!("Erreur : les clés ne correspondent pas !");
    }

    // Simulation d'Eve passive
    println!("Eve observe les messages : h_A = {}, h_B = {}\n\n", h_a1, h_b1);

    print!("Simulation avec Eve agissant comme un homme-du-milieu\n");

    // Étapes d'Alice
    let (x2, h_a2) = generate_public_key();
    println!("Alice envoie h_A = {}", h_a2);

    // Étapes de Bob
    let (y2, h_b2) = generate_public_key();
    println!("Bob envoie h_B = {}", h_b2);

    // Eve intercepte les messages et remplace h_A et h_B
    let (h_ea, h_eb, e_x, e_y) = eve_intercept();

    // Alice reçoit h_EB, Bob reçoit h_EA
    let k_a2 = calculate_secret(h_eb, x2);
    println!("Alice recoit h_eb = {} calcule sa clé k_A avec h_EB = {}", h_eb, k_a2);

    let k_b2 = calculate_secret(h_ea, y2);
    println!("Bob recoit h_ea = {} calcule sa clé k_B avec h_EA = {}",h_ea,  k_b2);

    // Eve peut calculer les clés intermédiaires
    let k_e_alice = calculate_secret(h_a2, e_x);  // Clé partagée avec Alice
    let k_e_bob = calculate_secret(h_b2, e_y);    // Clé partagée avec Bob
    println!("Eve calcule la clé avec Alice: {}, et avec Bob: {}", k_e_alice, k_e_bob);

    // Clés différentes, puisque Eve intercepte et remplace les valeurs
    println!("Clés échangées : Alice calcul le secret = : {}, Bob calcul le secret = {}, Eve-Alice: {}
    , Eve-Bob: {}", k_a2, k_b2, k_e_alice, k_e_bob);

    print!("b)\n");
    print!("Selon alice et bob, si il ne peuvent pas communiqué entre eux, il n'y a pas de différence entre les deux scénarios. \n\n");
    print!("c)\n");
    print!("Dans le premier scénarion ka=kb et l'échange de secret s'est correctement effectué.
    Dans le deuxième scénario, les clés sont différentes entre alice et bob mais tout les deux ont eu un 
    échange de secret corectement éffectué avec eve. \n\n");
    print!("d)\n");
    //(d) Dans la notation des notes de cours (voir révision IKE), quelles sont les Π𝐴𝑖 , Π𝐵𝑖 ?
    print!("Lors du premier scénario :\n");
    print!("Π𝐴𝑖 = (x = {}, h_a = {}, h_b = {}, k = {}) \n", x1, h_a1, h_b1, k_a1);
    print!("Π𝐵𝑖 = (y = {}, h_b = {}, h_a = {}, k = {}) \n", y1, h_b1, h_a1, k_b1);
    print!("Lors du deuxième scénario :\n");
    print!("Π𝐴𝑖 = (x = {}, h_a = {}, h_b = {}, k = {}) \n", x2, h_a2, h_ea, k_a2);
    print!("Π𝐵𝑖 = (y = {}, h_b = {}, h_a = {}, k = {}) \n", y2, h_b2, h_eb, k_b2);
    
}

fn main() {
    exercice1();
    
    
}
