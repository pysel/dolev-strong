use ed25519_dalek::Keypair;
use rand::rngs::OsRng;

pub fn gen_keypair() -> Keypair {
    let mut csprng = OsRng{};
    let keypair: Keypair = Keypair::generate(&mut csprng);
    keypair
}