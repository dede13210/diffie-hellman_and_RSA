use rand::Rng;

// Alice génère x, calcule h_A = 3^x mod 2^63 et envoie h_A
fn generate_public_key() -> (u64, u64) {
    let mut h = 1;
    let mut x;
    while h == 1 {
        // Génère un nombre aléatoire x
        x = rand::thread_rng().gen_range(2..=u32::MAX as u64); // x ∈ [1, 2^32]
        h = mod_exp(3, x, 2 ^ 63);
    }
    let x: u64 = rand::thread_rng().gen_range(2..=u32::MAX as u64); // x ∈ [1, 2^32]
    let h = mod_exp(3, x, 2 ^ 63);
    (x, h) // Retourne x et h_A pour l'étape suivante
}

fn calculate_secret(h: u64, x: u64) -> u64 {
    mod_exp(h, x, 2 ^ 63)
}

// Eve intercepte les messages et remplace h_A et h_B
fn eve_intercept() -> (u64, u64, u64, u64) {
    let mut h_ea = 1;
    let mut h_eb = 1;
    let mut e_x=0 ;
    let mut e_y=0;
    // Eve génère son propres secrets e_x
    while h_ea == 1 || h_eb == 1 {
        // Eve génère son propres secrets e_x et e_y
        e_x = rand::thread_rng().gen_range(2..=u32::MAX as u64);
        e_y = rand::thread_rng().gen_range(2..=u32::MAX as u64);

        h_ea = mod_exp(3, e_x, 2 ^ 63);
        h_eb = mod_exp(3, e_y, 2 ^ 63);
    }

    println!("Eve envoie h_EA = {}, h_EB = {}", h_ea, h_eb);
    (h_ea, h_eb, e_x, e_y)
}

fn exercice1() {
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
    println!(
        "Eve observe les messages : h_A = {}, h_B = {}\n\n",
        h_a1, h_b1
    );

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
    let k_a2 = calculate_secret(h_ea, x2);
    println!(
        "Alice recoit h_eb = {} calcule sa clé k_A = {}",
        h_eb, k_a2
    );

    let k_b2 = calculate_secret(h_eb, y2);
    println!(
        "Bob recoit h_ea = {} calcule sa clé k_B = {}",
        h_ea, k_b2
    );

    // Eve peut calculer les clés intermédiaires
    let k_e_alice = calculate_secret(h_a2, e_x); // Clé partagée avec Alice
    let k_e_bob = calculate_secret(h_b2, e_y); // Clé partagée avec Bob
    println!(
        "Eve calcule la clé avec Alice: {}, et avec Bob: {}",
        k_e_alice, k_e_bob
    );

    // Clés différentes, puisque Eve intercepte et remplace les valeurs
    println!(
        "Clés échangées : Alice calcul le secret = : {}, Bob calcul le secret = {}, Eve calcul le secret avec Alice: {}
    , Eve calcul le secret avec Bob: {}",
        k_a2, k_b2, k_e_alice, k_e_bob
    );

    print!("b)\n");
    print!("Selon alice et bob, si il ne peuvent pas communiqué entre eux, il n'y a pas de différence entre les deux scénarios. \n\n");
    print!("c)\n");
    print!("Dans le premier scénarion ka=kb et l'échange de secret s'est correctement effectué.
    Dans le deuxième scénario, les clés sont différentes entre alice et bob mais tout les deux ont eu un 
    échange de secret corectement éffectué avec eve. \n\n");
    print!("d)\n");
    //(d) Dans la notation des notes de cours (voir révision IKE), quelles sont les Π𝐴𝑖 , Π𝐵𝑖 ?
    print!("Lors du premier scénario :\n");
    print!(
        "Π𝐴𝑖 = (x = {}, h_a = {}, h_b = {}, k = {}) \n",
        x1, h_a1, h_b1, k_a1
    );
    print!(
        "Π𝐵𝑖 = (y = {}, h_b = {}, h_a = {}, k = {}) \n",
        y1, h_b1, h_a1, k_b1
    );
    print!("Lors du deuxième scénario :\n");
    print!(
        "Π𝐴𝑖 = (x = {}, h_a = {}, h_b = {}, k = {}) \n",
        x2, h_a2, h_ea, k_a2
    );
    print!(
        "Π𝐵𝑖 = (y = {}, h_b = {}, h_a = {}, k = {}) \n",
        y2, h_b2, h_eb, k_b2
    );
}

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

//Calcule le plus grand diviseur commun entre deux a et b
fn pgcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        pgcd(b, a % b)
    }
}

//Calcule de la clé publique e
fn calculate_e(phi: u64) -> u64 {
    //Génère un nombre aléatoire < phi
    let mut e = rand::thread_rng().gen_range(1..phi);
    while pgcd(e, phi) != 1 {
        e = rand::thread_rng().gen_range(1..phi);
    }
    e
}

//Calcule de la clé privé d
fn calculate_d(e: u64, phi: u64) -> u64 {
    let mut d = 1;
    while (d * e) % phi != 1 {
        d += 1;
    }
    d
}

fn encrypt(message: u64, e: u64, n: u64) -> u64 {
    mod_exp(message, e, n)
}

fn decrypt(ciphertext: u64, d: u64, n: u64) -> u64 {
    mod_exp(ciphertext as u64, d, n)
}

fn exercice2() {
    print!("Exo 2\n");
    //Alice génère sa clé publique et privée
    let (n, p, q) = (143, 11, 13);
    let phi = (p - 1) * (q - 1);
    let e = calculate_e(phi);
    let d = calculate_d(e, phi);

    //Bob envoie un message chiffré
    let message1 = 3;
    let message2 = 5;
    let message3 = 7;
    let encrypted_message1 = encrypt(message1, e, n);
    let encrypted_message2 = encrypt(message2, e, n);
    let encrypted_message3 = encrypt(message3, e, n);

    //Alice déchiffre le message
    let decrypted_message1 = decrypt(encrypted_message1, d, n);
    let decrypted_message2 = decrypt(encrypted_message2, d, n);
    let decrypted_message3 = decrypt(encrypted_message3, d, n);

    print!("Alice genère sa clé privé sk = (n = {},d ={} ) envoie sa clé publique pk = ( e = {},n = {}), elle recoit les messages chiffrées (m1 = {}, m2 = {}, m3 = {}), 
    Alice déchiffre le message (m1 = {}, m2 = {}, m3 = {})\n", d, n, e,  n, encrypted_message1, encrypted_message2, encrypted_message3,
     decrypted_message1, decrypted_message2, decrypted_message3);
    print!("bob recoit la clé publique (n = {}, e = {}), il chiffre les messages (m1 = {}, m2 = {}, m3 = {}) et envoie à alice les
    messages chiffres (m1 = {}, m2 = {}, m3 = {})\n", n, e, message1, message2, message3, encrypted_message1, encrypted_message2,
    encrypted_message3);
    print!("eve intercepte la cle publique (n = {}, e = {}), elle intercepte les messages chiffrés (m1 = {}, m2 = {}, m3 = {})\n", n, e, encrypted_message1, encrypted_message2, encrypted_message3);


    print!("\n\nd)\n");
     //Répéter le scénario avec m = 0 et m = 1
    let message4 = 0;
    let message5 = 1;
    let encrypted_message4 = encrypt(message4, e, n);
    let encrypted_message5 = encrypt(message5, e, n);
    let decrypted_message4 = decrypt(encrypted_message4, d, n);
    let decrypted_message5 = decrypt(encrypted_message5, d, n);
    print!("Alice genère sa clé privé sk = (n = {},d ={} ) envoie sa clé publique (e = {}, n = {}), elle recoit les messages chiffrées (m4 = {}, m5 = {}),
    Alice déchiffre le message (m4 = {}, m5 = {})\n", n, d, e, n, encrypted_message4, encrypted_message5, decrypted_message4, decrypted_message5);
    print!("bob recoit la clé publique (n = {}, e = {}), il chiffre les messages (m4 = {}, m5 = {}) et envoie à alice les
    messages chiffres (m4 = {}, m5 = {})\n", n, e, message4, message5, encrypted_message4, encrypted_message5);
    print!("eve intercepte la cle publique (n = {}, e = {}), elle intercepte les messages chiffrés (m4 = {}, m5 = {})\n", n, e, encrypted_message4, encrypted_message5);

}

// Algorithme de signature RSA (Sign2)
fn sign2(sk: (u64, u64), m: u64) -> u64 {
    let (n, d) = sk;
    mod_exp(m, d, n) // Calcul de la signature: m^d mod n
}

// Algorithme de vérification RSA (Verif2)
fn verif2(pk: (u64, u64), m: u64, sigma: u64) -> bool {
    let (n, e) = pk;
    let m_prime = mod_exp(sigma, e, n); // Calcul de la vérification: σ^e mod n
    m_prime == m
}


fn exercice3(){
    //la clé publique de Alice tel que l'exo 2
    let (n, p, q) = (143, 11, 13);
    let phi = (p - 1) * (q - 1);
    let e = calculate_e(phi);
    let d = calculate_d(e, phi);
    let pk = (n, e);
    let sk = (n, d);

    //Alice signe les messages
    let message1 = 3;
    let message2 = 5;
    let message3 = 7;
    let sigma1 = sign2(sk, message1);
    let sigma2 = sign2(sk, message2);
    let sigma3 = sign2(sk, message3);

    //Eve modifie le message3
    let fake_message3 = 8;

    //Bob vérifie les signatures
    let is_valid1 = verif2(pk, message1, sigma1);
    let is_valid2 = verif2(pk, message2, sigma2);
    let is_valid3 = verif2(pk, fake_message3, sigma3);

    print!("Alice génère clé publique et privées Alice d = {} et envoit sa clés publique (n = {}, e ={}), elle signe les messages (m1 = {}, m2 = {}, m3 = {}), Alice envoie les signatures (s1 = {}, s2 = {}, s3 = {})\n",d, n, e , message1, message2, message3, sigma1, sigma2, sigma3);
    print!("Eve intercepte la clé publique (n = {}, e ={}) et les signatures (s1 = {}, s2 = {}, s3 = {}), elle modifie le message 3 (m3 = {})\n", n, e, sigma1, sigma2, sigma3, fake_message3);
    print!("Bob reçoit la clé publique (n = {}, e ={}), et il reçoit et vérifie les signatures (s1 = {}, s2 = {}, s3 = {}), les signatures sont valides pour (s1 = {}, s2 = {}, s3 = {})\n", n, e , sigma1, sigma2, sigma3, is_valid1, is_valid2, is_valid3);




}
     
fn main() {
    // exercice1();
    exercice2();
    // exercice3();


}
